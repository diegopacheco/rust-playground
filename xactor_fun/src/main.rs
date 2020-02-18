extern crate xactor;
extern crate async_std;

use xactor::*;

#[xactor::message(String)]
struct ToUppercase(String);

struct MyActor;

impl Actor for MyActor {}

#[async_trait::async_trait]
impl Handler<ToUppercase> for MyActor {
    async fn handle(&mut self, _ctx: &Context<Self>, msg: ToUppercase){
        msg.0.to_uppercase();
    }
}

#[async_std::main]
async fn main() -> Result<()> {
    // Start actor and get its address
    let mut addr = MyActor.start();

    // Send message `ToUppercase` to actor via addr
    let res = addr.call(ToUppercase(std::string::String::from("lowercase"))).await?;
    println!("{:?}",res);
    Ok(())
}