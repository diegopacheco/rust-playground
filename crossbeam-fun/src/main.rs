extern crate crossbeam;

fn main() {
    let arr = &[1, 25, -4, 10,1,2,3,5,6,2,5,1,5,3,5,2,5,2];
    let max = find_max(arr);
    assert_eq!(max, Some(25));
    println!("{:?}",max);
}

fn find_max(arr: &[i32]) -> Option<i32> {
    const THRESHOLD: usize = 2;
  
    if arr.len() <= THRESHOLD {
        return arr.iter().cloned().max();
    }

    let mid = arr.len() / 2;
    let (left, right) = arr.split_at(mid);
  
    crossbeam::scope(|s| {
        let thread_l = s.spawn(|_| find_max(left));
        let thread_r = s.spawn(|_| find_max(right));
  
        let max_l = thread_l.join().unwrap()?;
        let max_r = thread_r.join().unwrap()?;
  
        Some(max_l.max(max_r))
    }).unwrap()
}