fn main() {
  let fruit = vec!['🥝', '🍌', '🍇'];
  let buffer_overflow = fruit[2];
  assert_eq!(buffer_overflow, '🍉');
}
