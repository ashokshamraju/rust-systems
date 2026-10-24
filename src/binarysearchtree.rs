use std::cmp::Ordering;

// A single node inside the Binary Search Tree
// Use `Box` to place children on the heap since recursive types must have a known static size.
struct Node<T> {
    val: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

// The top-level Binary Search Tree container.
// Holds an optional pointer to the root node.
pub struct BinarySearchTree<T> {
    root: Option<Box<Node<T>>>,
}

impl<T: Ord> BinarySearchTree<T> {
    // Initialises a brand-new, empty Binary Search Tree.
    pub fn new() -> Self {
        Self { root: None }
    }

    // Public interface to insert a value into the tree.
    // Returns `true` if the insertion succeeded, or `false` if it was a duplicate.
    pub fn insert(&mut self, val: T) -> bool {
        Self::insert_at(&mut self.root, val)
    }
    
    // Internal recursive helper to find the correct vacant leaf slot and insert the data.
    fn insert_at(slot: &mut Option<Box<Node<T>>>, val: T) -> bool {
        match slot {
            // Case 1: Found an empty position in the tree structure.
            None => {
                *slot = Some(Box::new(Node {
                    val,
                    left: None,
                    right: None,
                }));
                true // Successfully inserted
            }
            // Case 2: Slot is occupied. Compare values to determine which direction to branch.
            Some(node) => match val.cmp(&node.val) {
                Ordering::Less => Self::insert_at(&mut node.left, val),
                Ordering::Greater => Self::insert_at(&mut node.right, val),
                Ordering::Equal => false,// Duplicates are rejected by standard BST definition
            },
        }
    }

    // Public interface to search for a value in the tree.
    // Accepts a read-only reference `&T` to avoid taking ownership of the search key.
    pub fn contains(&self, val: &T) -> bool {
        let mut curr = &self.root;
        
        // Iteratively traverse down the tree
        while let Some(node) = curr {
            match val.cmp(&node.val) {
                Ordering::Less => curr = &node.left,
                Ordering::Greater => curr = &node.right,
                Ordering::Equal => return true, //element found
            }
        }
        false // item not found
    }

    // Public interface to delete a specific value from the tree.
    // Returns `true` if the item was found and successfully deleted.
    pub fn delete(&mut self, val: &T) -> bool {
        Self::delete_at(&mut self.root, val)
    }

    // Internal recursive helper to locate the target deletion node and alter pointer tracks.
    fn delete_at(slot: &mut Option<Box<Node<T>>>, val: &T) -> bool {
        let slot_ref = match slot {
            None => return false,
            Some(node) => node,
        };

        match val.cmp(&slot_ref.val) {
            Ordering::Less => Self::delete_at(&mut slot_ref.left, val),
            Ordering::Greater => Self::delete_at(&mut slot_ref.right, val),
            Ordering::Equal => {
                // MATCH FOUND: Extract the node out of the tree context using .take()
                let node = slot.take().unwrap();
                
                match (node.left, node.right) {
                    // Case 1: Target node is a leaf node (zero children)
                    (None, None) => {
                        *slot = None;
                    }
                    
                    // Case 2a: Node only has a right child branch
                    (None, Some(right_child)) => {
                        *slot = Some(right_child);
                    }

                    // Case 2b: Node only has a left child branch
                    (Some(left_child), None) => {
                        *slot = Some(left_child);
                    }

                    // Case 3: Node has two children branches
                    (Some(left_child), Some(right_child)) => {
                        // Package the right branch back up so extract_min can parse it
                        let mut right_opt = Some(right_child);
                        // Find, detach, and extract the value of the In-Order Successor
                        let successor_val = Self::extract_min(&mut right_opt);
                        
                        // Reconstruct a replacement node with the successor's value,
                        // keeping both original branches intact.
                        *slot = Some(Box::new(Node {
                            val: successor_val,
                            left: Some(left_child),
                            right: right_opt,
                        }));
                    }
                }
                true // Deletion completely processed
            }
        }
    }

    // Helper function that finds, detaches, and returns the smallest value in a subtree.
    fn extract_min(slot: &mut Option<Box<Node<T>>>) -> T {
        // Base Case: If the next node down on the left is None, the current node is the minimum.
        if slot.as_ref().unwrap().left.is_none() {
            let mut min_node = slot.take().unwrap();
            // Bypass this minimum node by attaching its right children to the parent's slot
            *slot = min_node.right.take();
            min_node.val
        } else {
            // Loop Case: Look ahead to stop at the PARENT node of the true minimum
            let mut curr = slot.as_mut().unwrap();
            while curr.left.as_ref().unwrap().left.is_some() {
                curr = curr.left.as_mut().unwrap();
            }
            // Parent found
            let mut min_node = curr.left.take().unwrap();
            curr.left = min_node.right.take();
            min_node.val
        }
    }
}

fn main() {
    let mut bst = BinarySearchTree::new();
    bst.insert(60);
    bst.insert(50);
    bst.insert(80);
    bst.insert(70);
    bst.insert(90);

    println!("Tree contains 70: {}", bst.contains(&70));
    bst.delete(&70);
    println!("Tree contains 70 after deletion: {}", bst.contains(&70));
    println!("Tree still contains 80: {}", bst.contains(&80));
}
