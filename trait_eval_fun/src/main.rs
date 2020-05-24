use trait_eval::*;
use std::fmt::Display;

trait FizzBuzzType {
    fn show() -> String; // Don't worry about this -- it's just so we can print the result
}

struct Fizz;

impl FizzBuzzType for Fizz {
    fn show() -> String {
        "Fizz".to_string()
    }
}

struct Buzz;

impl FizzBuzzType for Buzz {
    fn show() -> String {
        "Buzz".to_string()
    }
}

struct FizzBuzz;

impl FizzBuzzType for FizzBuzz {
    fn show() -> String {
        "FizzBuzz".to_string()
    }
}

impl<T: Nat> FizzBuzzType for T
where
    T: Eval,
    <T as Eval>::Output: Display,
{
    fn show() -> String {
        format!("{}", T::eval())
    }
}

trait FizzBuzzEval: Nat {
    type Result: FizzBuzzType;
}

impl<T: Nat,
    Mod3: Nat,
    Mod5: Nat,
    ShouldFizz: Bool,
    ShouldBuzz: Bool,
    ShouldFizzBuzz: Bool,
    DidBuzz: FizzBuzzType,
    DidFizz: FizzBuzzType,
    DidFizzBuzz: FizzBuzzType> FizzBuzzEval for T
where
    T: Mod<Three, Result = Mod3> + Mod<Five, Result = Mod5>,
    Mod3: Equals<Zero, Result = ShouldFizz>,
    Mod5: Equals<Zero, Result = ShouldBuzz>,
    ShouldFizz: AndAlso<ShouldBuzz, Result = ShouldFizzBuzz>,
    (Fizz, T): If<ShouldFizz, Result = DidFizz>,
    (Buzz, DidFizz): If<ShouldBuzz, Result = DidBuzz>,
    (FizzBuzz, DidBuzz): If<ShouldFizzBuzz, Result = DidFizzBuzz>,
{
    type Result = DidFizzBuzz;
}

fn main() {
    assert_eq!(<One as FizzBuzzEval>::Result::show(), "1");
    assert_eq!(<Two as FizzBuzzEval>::Result::show(), "2");
    assert_eq!(<Three as FizzBuzzEval>::Result::show(), "Fizz");
    assert_eq!(<Four as FizzBuzzEval>::Result::show(), "4");
    assert_eq!(<Five as FizzBuzzEval>::Result::show(), "Buzz");
    assert_eq!(<Six as FizzBuzzEval>::Result::show(), "Fizz");
    assert_eq!(<Seven as FizzBuzzEval>::Result::show(), "7");
    assert_eq!(<Eight as FizzBuzzEval>::Result::show(), "8");
    assert_eq!(<Nine as FizzBuzzEval>::Result::show(), "Fizz");
    assert_eq!(<Ten as FizzBuzzEval>::Result::show(), "Buzz");

    type Fifteen = <Three as Times<Five>>::Result;
    assert_eq!(<Fifteen as FizzBuzzEval>::Result::show(), "FizzBuzz"); // !!
    
    println!("works");
}
