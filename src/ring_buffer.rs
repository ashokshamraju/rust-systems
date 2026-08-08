use std::ops::Add;

pub struct RingBuffer<T> {
    // A mutable raw pointer pointing to the head of memory allocation
    raw_ptr: *mut T,
    // Maximum element capacity of block
    capacity: usize,
    // Write tracker index cursor
    head: usize,
    // Read tracker index cursor
    tail: usize,
}

impl<T> RingBuffer<T> {
    // 1. Unsafe Constructor
    pub unsafe fn from_raw(ptr: *mut T, capacity: usize) -> Self {
        Self {
            raw_ptr: ptr,
            capacity,
            head: 0,
            tail: 0,
        }
    }

    pub unsafe fn push_unchecked(&mut self, item: T) {

        unsafe {
            let dest = self.raw_ptr.add(self.head);

            *dest = item;

            self.head = (self.head + 1) % self.capacity;
        }
    }

    pub fn read_next<'a>(&mut self) -> Option<&'a T> {
        if self.head == self.tail {
            // Buffer is completely empty
            None
        } else {
            unsafe {
                let  source_ptr = self.raw_ptr.add(self.tail);

                let reference = &*source_ptr;
                self.tail = (self.tail + 1) % self.capacity;

                Some(reference)
            }
        }
    }
}

fn main() {
    // Allocate a standard array representing raw firmware memory registers
    let mut storage = [0; 4];
    let ptr = storage.as_mut_ptr();
    let cap = storage.len();

    println!("--- Initializing Performance Ring Buffer ---");
    let mut buffer = unsafe { RingBuffer::from_raw(ptr, cap) };

    println!("\nWriting stream packets to memory...");
    unsafe {
        buffer.push_unchecked(42);
        buffer.push_unchecked(99);
        buffer.push_unchecked(1001);
    }

    println!("\nExecuting Safe Lifetime-Bound Reads:");
    println!("Read 1: {:?}", buffer.read_next()); // Should be Some(&42)
    println!("Read 2: {:?}", buffer.read_next()); // Should be Some(&99)
    println!("Read 3: {:?}", buffer.read_next()); // Should be Some(&1001)
    println!("Read 4: {:?}", buffer.read_next()); // Should be None (empty!)
}
