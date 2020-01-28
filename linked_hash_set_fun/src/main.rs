extern crate linked_hash_set;
use linked_hash_set::LinkedHashSet;

fn main() {
  let mut set = LinkedHashSet::new();
  assert!(set.insert(234));
  assert!(set.insert(123));
  assert!(set.insert(345));
  assert!(!set.insert(123));
  println!("{:?}",set);
  assert_eq!(set.into_iter().collect::<Vec<_>>(), vec![234, 345, 123]);
}
