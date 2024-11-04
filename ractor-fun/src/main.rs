use async_trait::async_trait;
use ractor::{Actor, ActorRef, ActorProcessingErr};

pub struct MyFirstActor;

#[async_trait::async_trait]
impl Actor for MyFirstActor {
    type State = ();
    type Msg = MyFirstActorMessage;
    type Arguments = ();

    async fn pre_start(&self, _myself: ActorRef<Self::Msg>, _arguments: Self::Arguments)
        -> Result<Self::State, ActorProcessingErr> 
    {
        Ok(())
    }
}

pub enum MyFirstActorMessage {
    /// Print's hello world
    PrintHelloWorld,
}

#[tokio::main]
async fn main() {
    // Build an ActorRef along with a JoinHandle which lives for the life of the 
    // actor. Most of the time we drop this handle, but it's handy in the 
    // main function to wait for clean actor shut-downs (all stop handlers will
    // have completed)
    let (actor, actor_handle) = Actor::spawn(None, MyFirstActor, ()).await.expect("Actor failed to start");
    
    for _i in 0..10 {
        // Sends a message, with no reply
        actor.cast(MyFirstActorMessage::PrintHelloWorld).expect("Failed to send message to actor");
    }

    // give a little time to print out all the messages
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    // Cleanup
    actor.stop(None);
    actor_handle.await.unwrap();
}