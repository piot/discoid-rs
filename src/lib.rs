pub mod discoid;

pub struct CircularBuffer<T> {
    buffer: Vec<Option<T>>,
    front: usize,
    rear: usize,
    capacity: usize,
}

impl<T> CircularBuffer<T> {
    pub fn new(size: usize) -> Self {
        CircularBuffer {
            buffer: (0..size).map(|_| None).collect(),
            front: 0,
            rear: 0,
            capacity: size,
        }
    }

    pub fn push(&mut self, value: T) {
        let next_rear = (self.rear + 1) % self.capacity;

        if next_rear == self.front {
            panic!("circular buffer: can not push, buffer is full");
        }

        self.buffer[self.rear] = Some(value);
        self.rear = next_rear;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.front == self.rear {
            panic!("circular buffer: can not pop, buffer is empty");
        }

        let value = self.buffer[self.front].take();
        self.front = (self.front + 1) % self.capacity;

        value
    }

    pub fn len(&self) -> usize {
        (self.rear + self.capacity - self.front) % self.capacity
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn is_full(&self) -> bool {
        let next_rear = (self.rear + 1) % self.capacity;
        next_rear == self.front
    }

    pub fn get_at_index(&self, index: usize) -> Option<&T> {
        let buffer_len = self.capacity;
        if index >= buffer_len {
            return None;
        }

        let absolute_index = (self.front + index) % buffer_len;
        self.buffer[absolute_index].as_ref()
    }

    pub fn pop_multiple(&mut self, count: usize) {
        if count > self.len() {
            panic!("circular buffer: pop_multiple too much")
        }

        for _ in 0..count {
            self.buffer[self.front] = None;
            self.front = (self.front + 1) % self.buffer.len();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq, Clone)]
    pub struct Item {
        pub x: i32,
        pub boolean: bool,
    }

    #[test]
    fn add_test() {
        let mut circular_buffer = CircularBuffer::<Item>::new(8);

        let first_item = Item {
            x: 23,
            boolean: false,
        };
        circular_buffer.push(first_item.clone());

        assert_eq!(circular_buffer.len(), 1);

        assert_eq!(
            circular_buffer.get_at_index(0),
            Some(first_item.clone()).as_ref()
        );
        assert_eq!(circular_buffer.get_at_index(1), None);

        let popped_item = circular_buffer.pop(); // front is increased
        assert_eq!(popped_item.unwrap(), first_item);

        assert_eq!(circular_buffer.get_at_index(1), None);

        assert_eq!(circular_buffer.get_at_index(7), None);
    }
}
