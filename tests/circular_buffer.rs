use discoid::BufferError;
use discoid::CircularBuffer;

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
    circular_buffer.push(first_item.clone()).unwrap();

    assert_eq!(circular_buffer.len(), 1);

    assert_eq!(circular_buffer.get(0), Some(first_item.clone()).as_ref());
    assert_eq!(circular_buffer.get(1), None);

    let popped_item = circular_buffer.pop(); // front is increased
    assert_eq!(popped_item.unwrap(), first_item);

    assert_eq!(circular_buffer.get(1), None);

    assert_eq!(circular_buffer.get(7), None);
}

#[test]
fn test_push_and_pop_single_element() {
    let mut cb = CircularBuffer::new(3);
    assert_eq!(cb.len(), 0);
    assert!(cb.is_empty());

    // Push an element
    assert_eq!(cb.push(10), Ok(()));
    assert_eq!(cb.len(), 1);
    assert!(!cb.is_empty());

    // Pop the element
    assert_eq!(cb.pop(), Some(10));
    assert_eq!(cb.len(), 0);
    assert!(cb.is_empty());
}

#[test]
fn test_push_full_buffer() {
    let mut cb = CircularBuffer::new(2);
    assert_eq!(cb.push(1), Ok(()));
    assert_eq!(cb.push(2), Ok(()));
    assert!(cb.is_full());

    // Attempt to push to a full buffer
    assert_eq!(cb.push(3), Err(BufferError::BufferFull));
}

#[test]
fn test_pop_empty_buffer() {
    let mut cb: CircularBuffer<i32> = CircularBuffer::new(2);
    assert!(cb.is_empty());

    // Attempt to pop from an empty buffer
    assert_eq!(cb.pop(), None);
}

#[test]
fn test_push_remove_multiple_elements() {
    let mut cb = CircularBuffer::new(5);
    for i in 1..=5 {
        assert_eq!(cb.push(i), Ok(()));
    }
    assert!(cb.is_full());

    // Attempt to push to a full buffer
    assert_eq!(cb.push(6), Err(BufferError::BufferFull));

    // Pop all elements
    for i in 1..=5 {
        assert_eq!(cb.pop(), Some(i));
    }
    assert!(cb.is_empty());

    // Attempt to pop from an empty buffer
    assert_eq!(cb.pop(), None);
}

#[test]
fn test_pop_multiple_success() {
    let mut cb = CircularBuffer::new(5);
    for i in 1..=5 {
        cb.push(i).unwrap();
    }

    // Pop 3 elements
    cb.remove_multiple(3);
    assert_eq!(cb.len(), 2);

    // Remaining elements should be 4 and 5
    assert_eq!(cb.pop(), Some(4));
    assert_eq!(cb.pop(), Some(5));
    assert!(cb.is_empty());
}

#[test]
fn test_into_iter_empty_buffer() {
    let cb: CircularBuffer<i32> = CircularBuffer::new(5);
    let mut iter = cb.into_iter();
    assert_eq!(iter.next(), None);
}

#[test]
fn test_into_iter_single_element() {
    let mut cb = CircularBuffer::new(5);
    cb.push(10).unwrap();
    let mut iter = cb.into_iter();
    assert_eq!(iter.next(), Some(10));
    assert_eq!(iter.next(), None);
}

#[test]
fn test_into_iter_multiple_elements() {
    let mut cb = CircularBuffer::new(5);
    cb.push(1).unwrap();
    cb.push(2).unwrap();
    cb.push(3).unwrap();
    let collected: Vec<_> = cb.into_iter().collect();
    assert_eq!(collected, vec![1, 2, 3]);
}

#[test]
fn test_into_iter_after_wrap_around() {
    let mut cb = CircularBuffer::new(5);
    cb.push(1).unwrap();
    cb.push(2).unwrap();
    cb.push(3).unwrap();
    cb.pop().unwrap(); // Remove 1
    cb.pop().unwrap(); // Remove 2
    cb.push(4).unwrap();
    cb.push(5).unwrap();
    cb.push(6).unwrap(); // Buffer is now [3,4,5,6]

    let collected: Vec<_> = cb.into_iter().collect();
    assert_eq!(collected, vec![3, 4, 5, 6]);
}

#[test]
fn test_into_iter_full_buffer() {
    let mut cb = CircularBuffer::new(4); // Capacity 4
    cb.push('a').unwrap();
    cb.push('b').unwrap();
    cb.push('c').unwrap();
    cb.push('d').unwrap(); // Buffer is full

    let collected: Vec<_> = cb.into_iter().collect();
    assert_eq!(collected, vec!['a', 'b', 'c', 'd']);
}

#[test]
fn test_into_iter_consumes_buffer() {
    let mut cb = CircularBuffer::new(5);
    cb.push(100).unwrap();
    cb.push(200).unwrap();
    cb.push(300).unwrap();

    let mut iter = cb.into_iter();
    assert_eq!(iter.next(), Some(100));
    assert_eq!(iter.next(), Some(200));
    assert_eq!(iter.next(), Some(300));
    assert_eq!(iter.next(), None);
}

#[test]
fn test_into_iter_no_elements_after_popping() {
    let mut cb = CircularBuffer::new(5);
    cb.push(1).unwrap();
    cb.push(2).unwrap();
    cb.push(3).unwrap();
    cb.pop().unwrap(); // Remove 1
    cb.pop().unwrap(); // Remove 2

    let collected: Vec<_> = cb.into_iter().collect();
    assert_eq!(collected, vec![3]);
}

#[test]
fn test_into_iter_mixed_operations() {
    let mut cb = CircularBuffer::new(6);
    cb.push(1).unwrap();
    cb.push(2).unwrap();
    cb.push(3).unwrap();
    cb.pop().unwrap(); // Remove 1
    cb.push(4).unwrap();
    cb.push(5).unwrap();
    cb.pop().unwrap(); // Remove 2
    cb.push(6).unwrap();
    cb.push(7).unwrap(); // Buffer should have [3,4,5,6,7]

    let collected: Vec<_> = cb.into_iter().collect();
    assert_eq!(collected, vec![3, 4, 5, 6, 7]);
}

#[test]
fn test_full_capacity_storage() {
    let capacity = 5;
    let mut cb = CircularBuffer::new(capacity);
    for i in 0..capacity {
        assert_eq!(cb.push(i), Ok(()));
    }
    assert!(cb.is_full());
    assert_eq!(cb.len(), capacity);

    let collected: Vec<_> = cb.into_iter().collect();
    assert_eq!(collected, vec![0, 1, 2, 3, 4]);
}

#[test]
fn test_repeated_push_pop() {
    let capacity = 3;
    let mut cb = CircularBuffer::new(capacity);

    // First cycle
    assert_eq!(cb.push(1), Ok(()));
    assert_eq!(cb.push(2), Ok(()));
    assert_eq!(cb.push(3), Ok(()));
    assert!(cb.is_full());

    let collected: Vec<_> = cb.clone().into_iter().collect();
    assert_eq!(collected, vec![1, 2, 3]);

    assert_eq!(cb.pop(), Some(1));
    assert_eq!(cb.pop(), Some(2));
    assert_eq!(cb.pop(), Some(3));
    assert!(cb.is_empty());

    // Second cycle
    assert_eq!(cb.push(4), Ok(()));
    assert_eq!(cb.push(5), Ok(()));
    assert_eq!(cb.push(6), Ok(()));
    assert!(cb.is_full());

    let collected: Vec<_> = cb.into_iter().collect();
    assert_eq!(collected, vec![4, 5, 6]);
}
