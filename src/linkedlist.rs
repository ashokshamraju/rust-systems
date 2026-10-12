/* Box<Node<T>>: In Rust, recursive types must have a known size at compile time. 
   Placing the Node inside a Box moves it to the heap, turning a 
   variable-sized recursive struct into a fixed-size pointer. */
/* Option<...>: Rust has no null values. Option serves as a type-safe alternative. 
   None indicates the end of the list (or an empty list). */
struct Node {
    val: T,
    next: Option<Box<Node<T>>>,
}

pub struct List<T> {
    head: Option<Box<Node<T>>>,
    len: usize,
}

impl<T> List<T> {

    /* Initialises a brand-new, empty list. 
    It sets the head to None and the tracking 
    length len to 0 */
    pub fn new() -> Self {
        Self {
            head:None,
            len: 0
        }
    }

    /* Adds a new node containing val to the beginning of the list. */
    pub fn push_front(&mut self, val:T) {
        
        /* Extract the value out of self.head, 
           replaces self.head with None, and returns the original value. 
           This allows to temporarily take ownership of the old head without 
           violating borrow checker rules. */
        let prev = self.head.take();
        
        /* A new Node is created on the heap via Box::new(), 
           with its next field pointing to the old head */
        self.head = Some(Box::new(Node {val, next:prev}));
        self.len += 1; // increment length
    }

    /*Returns an immutable reference to the value inside the head node
      if it exists */
    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|n| &n.val)
    }

    /* Removes the first element from the list and returns its value */
    pub fn pop_front(&mut self) -> Option<T> {
        /* .take pulls entire list out of self.head.If self.head
        is Some(node), the closure executes, else map returns None safely */
        self.head.take.map(|n| {
            self.head = n.next;
            self.len -= 1;
            n.val
        })
    }

    pub fn reverse(&mut self) {
        let mut prev = None;
        let mut curr = self.head.take();

        while let Some(n) = curr {
            curr = n.next.take();
            n.next = prev;
            prev = Some(n)
        }

        self.head = prev;
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut curr = self.head.take();

        while let Some(n) = curr {
            curr = n.next.take()
        }
    }
}

// Implementing Iterators

// Type 1: consumes List
pub struct IntoIter<T>(List<T>);

/* IntoIterator is a converter trait. It defines how a 
   collection can be turned into an iterator. It doesn't 
   actually step through elements */
impl<T> IntoIterator for List<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self)
    }
}

/* Iterator is the engine trait. It requires the next() method, 
   which actually tracks the state and spits out elements one by one. 
   List<T> itself cannot implement Iterator directly because an iterator 
   needs to maintain mutating state (like where it currently is in the list), 
   whereas List<T> is just a data container. */
impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(self)-> Option<Self::Item> {
        self.0.pop_front() // using existing method to pop value out
    }
}

//Type 2 Iter - immutable reference. Borrowing only
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<T> List<T> {
    pub fn Iter(&self)-> Iter<'_, T> {
        Iter {
            next: self.head.as_deref(),
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.val
        })
    }
}