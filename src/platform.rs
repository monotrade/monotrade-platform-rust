use actix::System;
use actix::*;
use crate::actors::mock_actor::{ MockActor,Sum};

pub struct Platform {
    pub name: String,
}

impl Platform {
    pub fn new(name:String) -> Platform {
        Platform {
            name,
        }
    }

    pub async fn initialize(&self) {
        let system = actix::System::new(&self.name);
        // System::run(|| {
        //     let addr = MockActor.start(); // <- start actor and get its address
        // });
         
        let addr = MockActor.start();
        
        let res = addr.send(Sum::new(10usize, 5usize)).await; // <- send message and get future for result

        match res {
            Ok(result) => println!("SUM: {}", result),
            _ => println!("Communication to the actor has failed"),
        }

        system.run();
    }
}


