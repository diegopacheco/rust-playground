use ractor::{Actor, ActorRef, ActorProcessingErr, RpcReplyPort};

pub enum MyFirstActorMessage {
    /// Print's hello world
    PrintHelloWorld,
    /// Replies with how many hello worlds have occurred
    HowManyHelloWorlds(RpcReplyPort<u16>),
}

pub struct MyFirstActor;

#[async_trait::async_trait]
impl Actor for MyFirstActor {
    type State = u16;
    type Msg = MyFirstActorMessage;
    type Arguments = ();

    async fn pre_start(&self, _myself: ActorRef<Self::Msg>, _arguments: Self::Arguments)
        -> Result<Self::State, ActorProcessingErr>
    {
        Ok(0)
    }

    async fn handle(&self, _myself: ActorRef<Self::Msg>, message: Self::Msg, state: &mut Self::State) 
        -> Result<(), ActorProcessingErr>
    {
        match message {
            MyFirstActorMessage::PrintHelloWorld => {
                println!("Hello world!");
                *state += 1;
            }
            MyFirstActorMessage::HowManyHelloWorlds(reply) => {
                if reply.send(*state).is_err() {
                    println!("Listener dropped their port before we could reply");
                }
            }
        }
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    // Build an ActorRef along with a JoinHandle which lives for the life of the 
    // actor. Most of the time we drop this handle, but it's handy in the 
    // main function to wait for clean actor shut-downs (all stop handlers will
    // have completed)
    let (actor, actor_handle) = 
        Actor::spawn(None, MyFirstActor, ())
            .await
            .expect("Actor failed to start");
    
    for _i in 0..10 {
        // Sends a message, with no reply
        actor.cast(MyFirstActorMessage::PrintHelloWorld)
            .expect("Failed to send message to actor");
    }

    let hello_world_count = 
        ractor::call_t!(actor, MyFirstActorMessage::HowManyHelloWorlds, 100)
        .expect("RPC failed");
    
    println!("Actor replied with {} hello worlds!", hello_world_count);

    // Cleanup
    actor.stop(None);
    actor_handle.await.unwrap();
}
