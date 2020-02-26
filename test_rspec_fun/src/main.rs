extern crate speculate;
use speculate::speculate;

speculate!{

    #[allow(dead_code)]    
    const ZERO: i32 = 0;

    #[allow(dead_code)]
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    describe "math" {

        #[allow(dead_code)]
        const ONE: i32 = 1;

        #[allow(dead_code)]
        fn sub(a: i32, b: i32) -> i32 {
            a - b
        }

        before {
            let two = ONE + ONE;
        }

        it "can add stuff" {
            assert_eq!(ONE, add(ZERO, ONE));
            assert_eq!(two, add(ONE, ONE));
        }

        it "can subtract stuff" {
            assert_eq!(ZERO, sub(ONE, ONE));
            assert_eq!(ONE, sub(two, ONE));
        }
    }
}

fn main(){}