mod vars;
mod functions;
mod structfun;
mod enumfun;

fn main() {
    println!("Hello, world!");
    vars::execute();
    functions::execute();
    structfun::printStruct();
    enumfun::execute();
}
