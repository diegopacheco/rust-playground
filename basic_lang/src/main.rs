mod vars;
mod functions;
mod structfun;
mod enumfun;
mod listsfun;
mod core_data_types;

fn main() {
    println!("Hello, world!");
    vars::execute();
    functions::execute();
    structfun::printStruct();
    enumfun::execute();
    listsfun::execute();
    core_data_types::execute();
}
