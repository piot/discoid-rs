pub struct DiscoidBuffer<T> {
    buffer: Vec<Option<T>>,
    front: usize,
    capacity: usize,
}

/// A circular buffer that efficiently handles elements of small and non-complex types.
///
/// This buffer is designed to operate on types `T` that implement the `Copy` trait, which allows for
/// an efficient and safe handling of simple scalar values or small structs. Using `Copy` ensures that
/// elements can be duplicated simply by copying bits without the need for complex cloning operations.
/// This design choice is optimal for scenarios where performance and memory efficiency are critical,
/// and the elements do not manage resources or require complex duplication logic.
///
/// The buffer automatically wraps around at its ends, providing a continuous view of the data even as
/// elements are added or removed, making it ideal for use cases like audio processing, real-time data
/// sampling, or any situation where a fixed-capacity queue is beneficial.
///
/// # Type Parameter
///
/// * `T` - The type of elements stored in the buffer. Must implement the `Copy` trait to ensure
///   that elements can be easily and safely duplicated with no additional overhead typical of more
///   complex data types.
///
/// # Examples
///
/// Here is how you might initialize and use a `DiscoidBuffer` with a simple integer type, which
/// implements `Copy`:
///
/// ```rust
/// use discoid::discoid::DiscoidBuffer;
///
/// let mut buffer = DiscoidBuffer::new(5); // Initialize a new buffer with capacity of 5 elements.
/// buffer.set_at_index(0, 42);             // Set the first element to 42.
/// buffer.set_at_index(1, 35);             // Set the second element to 35.
///
/// if let Some(value) = buffer.get_at_index(0) {
///     println!("The first element is {}", value);
/// }
/// ```
///
/// # Panics
///
/// - `set_at_index` will panic if an index is provided that exceeds the buffer's current capacity.
///
impl<T: Copy> DiscoidBuffer<T> {
    pub fn new(size: usize) -> Self {
        Self {
            buffer: (0..size).map(|_| None).collect(),
            front: 0,
            capacity: size,
        }
    }

    pub fn set_at_index(&mut self, index: usize, value: T) {
        let buffer_len = self.capacity;
        if index >= buffer_len {
            panic!("discoid buffer: index out of bounds");
        }

        let absolute_index = (self.front + index) % buffer_len;
        self.buffer[absolute_index] = Some(value);
    }

    pub fn get_ref_at_index(&self, index: usize) -> Option<&T> {
        let buffer_len = self.capacity;
        if index >= buffer_len {
            return None;
        }

        let absolute_index = (self.front + index) % buffer_len;
        self.buffer[absolute_index].as_ref()
    }

    pub fn get_at_index(&self, index: usize) -> Option<T> {
        let buffer_len = self.capacity;
        if index >= buffer_len {
            return None;
        }

        let absolute_index = (self.front + index) % buffer_len;
        self.buffer[absolute_index]
    }

    pub fn discard_front(&mut self, count: usize) {
        if count > self.capacity {
            panic!("discoid buffer: discarding too much")
        }

         for _ in 0..count {
            self.buffer[self.front] = None;
            self.front = (self.front + 1) % self.capacity;
        }
    }

    pub fn get_bits_representation(&self) -> u64 {
        let mut bits = 0u64;
        let buffer_len = self.capacity;

        for i in 0..buffer_len {
            let index = (self.front + i) % buffer_len;
            if self.buffer[index].is_some() {
                bits |= 1u64 << i;
            }
        }

        bits
    }
}

#[cfg(test)]
mod discoid_tests {
    use super::*;

    #[derive(Debug, PartialEq, Copy, Clone)]
    pub struct Item {
        pub x: i32,
        pub boolean: bool,
    }

    #[test]
    fn discoid_test() {
        let mut discoid_buffer = DiscoidBuffer::<Item>::new(8);
        assert_eq!(discoid_buffer.get_bits_representation(), 0);

        let first_item = Item {
            x: 23,
            boolean: false,
        };
        discoid_buffer.set_at_index(0, first_item.clone());

        assert_eq!(discoid_buffer.get_bits_representation(), 1);

        assert_eq!(
            discoid_buffer.get_ref_at_index(0),
            Some(first_item.clone()).as_ref()
        );
        assert_eq!(discoid_buffer.get_at_index(1), None);

        let middle_item = Item {
            x: 99,
            boolean: true,
        };
        discoid_buffer.set_at_index(3, middle_item.clone());

        assert_eq!(discoid_buffer.get_bits_representation(), 0b1001);

        assert_eq!(discoid_buffer.get_ref_at_index(1), None);

        assert_eq!(discoid_buffer.get_ref_at_index(3), Some(&middle_item));
        assert_eq!(discoid_buffer.get_at_index(3), Some(middle_item));

        assert_eq!(discoid_buffer.get_ref_at_index(7), None);
    }
}
