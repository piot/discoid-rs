use std::fmt;

/// Represents errors that can occur when interacting with the CircularBuffer.
#[derive(Debug, PartialEq)]
pub enum BufferError {
    /// Error returned when attempting to push to a full buffer.
    BufferFull,
}

impl fmt::Display for BufferError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BufferFull => write!(f, "circular buffer: cannot push, buffer is full"),
        }
    }
}

impl std::error::Error for BufferError {}

#[derive(Debug, Clone)]
pub struct CircularBuffer<T> {
    buffer: Vec<Option<T>>,
    front: usize,
    rear: usize,
    capacity: usize,
    len: usize,
}

impl<T> CircularBuffer<T> {
    /// Creates a new CircularBuffer with the specified capacity.
    pub fn new(size: usize) -> Self {
        let mut buffer = Vec::with_capacity(size);
        for _ in 0..size {
            buffer.push(None);
        }
        CircularBuffer {
            buffer,
            front: 0,
            rear: 0,
            capacity: size,
            len: 0,
        }
    }

    /// Adds an element to the rear of the buffer.
    /// Returns `Ok(())` if successful or `Err(BufferError::BufferFull)` if the buffer is full.
    pub fn push(&mut self, value: T) -> Result<(), BufferError> {
        if self.is_full() {
            return Err(BufferError::BufferFull);
        }

        self.buffer[self.rear] = Some(value);
        self.rear = (self.rear + 1) % self.capacity;
        self.len += 1;
        Ok(())
    }

    /// Removes and returns the element from the front of the buffer.
    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        let value = self.buffer[self.front]
            .take()
            .expect("Buffer invariant violated: None value in non-empty buffer");
        self.front = (self.front + 1) % self.capacity;
        self.len -= 1;
        Some(value)
    }

    /// Returns the number of elements in the buffer.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Checks if the buffer is empty.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Checks if the buffer is full.
    pub fn is_full(&self) -> bool {
        self.len == self.capacity
    }

    /// Retrieves a reference to the element at the given index.
    /// Returns `Some(&T)` if the index is within bounds or `None` otherwise.
    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.len {
            return None;
        }

        let absolute_index = (self.front + index) % self.capacity;
        self.buffer[absolute_index].as_ref()
    }

    /// Removes multiple elements from the front of the buffer.
    pub fn remove_multiple(&mut self, count: usize) {
        let count_to_remove = if count > self.len { self.len } else { count };

        for _ in 0..count_to_remove {
            self.buffer[self.front] = None;
            self.front = (self.front + 1) % self.capacity;
            self.len -= 1;
        }
    }
}

/// Iterator that consumes the CircularBuffer and yields its elements in order.
pub struct CircularBufferIntoIter<T> {
    buffer: Vec<Option<T>>,
    front: usize,
    capacity: usize,
    current: usize,
    remaining: usize,
}

impl<T> Iterator for CircularBufferIntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }

        let index = (self.front + self.current) % self.capacity;
        self.current += 1;
        self.remaining -= 1;

        self.buffer[index].take()
    }
}

impl<T> IntoIterator for CircularBuffer<T> {
    type Item = T;
    type IntoIter = CircularBufferIntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        CircularBufferIntoIter {
            buffer: self.buffer,
            front: self.front,
            capacity: self.capacity,
            current: 0,
            remaining: self.len,
        }
    }
}
