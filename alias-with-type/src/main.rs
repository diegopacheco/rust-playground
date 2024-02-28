type NanoSecond = u64;
type Inch = u64;
type TimeMeasure = u64;
type DistanceMeasure = u64;

fn main() {
    let nanoseconds:NanoSecond = 5 as TimeMeasure;
    let inches:Inch = 2 as DistanceMeasure;

    // Note that type aliases *don't* provide any extra type safety, because
    // aliases are *not* new types
    println!("{} nanoseconds + {} inches = {} unit?",
             nanoseconds,
             inches,
             nanoseconds + inches);
}