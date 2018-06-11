mod vars;
mod functions;
mod structfun;

fn main() {
    println!("Hello, world!");
    vars::execute();

    let i = functions::addOne(10);
    println!("addOne: {:?}",i);
    let f:fn(i32)->i32 = functions::addOne;
    println!("addOne: {:?}",f(i));

    structfun::printStruct();
}
