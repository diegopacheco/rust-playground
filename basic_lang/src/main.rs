mod vars;
mod functions;
mod structfun;
mod enumfun;
mod listsfun;

fn main() {
    println!("Hello, world!");
    vars::execute();
    functions::execute();
    structfun::printStruct();
    enumfun::execute();
    listsfun::execute();
}
