fn main() {
    let mut _mutable_integer = 7i32;

    {
        // Shadowing by immutable `_mutable_integer`
        let _mutable_integer = _mutable_integer;

        // Error! `_mutable_integer` is frozen in this scope
        // E0384 cannot mutate immutable variable `_mutable_integer
        // uncoment next line and see the issue
        // _mutable_integer = 50;
        println!("{_mutable_integer}");

        // `_mutable_integer` goes out of scope
    }

    // Ok! `_mutable_integer` is not frozen in this scope
    _mutable_integer = 3;
    println!("{_mutable_integer}");
}