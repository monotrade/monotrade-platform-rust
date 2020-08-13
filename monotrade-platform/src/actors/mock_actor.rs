use actix::{Actor, Addr, Arbiter, Context, System};
use actix::prelude::*;


// this is our Message
#[derive(Message)]
#[rtype(result = "usize")] // we have to define the response type for `Sum` message
pub struct Sum(pub usize,pub usize);
// impl Sum {
//     pub fn new(a:usize, b:usize) -> Sum {
//         Sum(a,b)
//     }
// }

// Actor definition
pub struct MockActor;

impl Actor for MockActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        println!("I am alive!");
        //System::current().stop(); // <- stop system
    }
}

// now we need to define `MessageHandler` for the `Sum` message.
impl Handler<Sum> for MockActor {
    type Result = usize; // <- Message response type

    fn handle(&mut self, msg: Sum, ctx: &mut Context<Self>) -> Self::Result {
        msg.0 + msg.1
    }
}
