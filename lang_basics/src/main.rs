fn main() {
    println!("Hello, world!");

    for x in 0..10 {
      print!("for {}", x);
    }

    let mut range = 0..10;

    loop {
       match range.next() {
          Some(x) => {
            print!("{}", x);
          },
          None => { break }
      }
   }

   let nums = vec![1, 2, 3];
   for i in 0..nums.len() {
     print!("vec {}", nums[i]);
   }
   
   let one_to_one_hundred = (1..101).collect::<Vec<i32>>();

   let sum = (1..4).fold(0, |sum, x| sum + x);

   println!("fold {}",sum);

   (1..100).map(|x| print!("map {}", x));

      
   (1..1000)
    .filter(|&x| x % 2 == 0)
    .filter(|&x| x % 3 == 0)
    .take(5)
    .map(|x| println!("{}",x));

}
