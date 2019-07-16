extern crate rand;

mod vars;
mod functions;
mod structfun;
mod enumfun;
mod listsfun;
mod core_data_types;
mod unsafe_global;
mod if_stmt;
mod loops;
mod match_stmt;

fn main() {
    println!("Hello, world!");
    vars::execute();
    functions::execute();
    structfun::printStruct();
    enumfun::execute();
    listsfun::execute();
    core_data_types::execute();
    unsafe_global::execute();
    if_stmt::execute();
    loops::execute();
    match_stmt::execute();
}
