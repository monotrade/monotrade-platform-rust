use actix::System;
use actix::*;


use monotrade_api::api::gateway;
use crate::actors::mock_actor::{ MockActor,Sum};

pub struct Platform {
    pub name: String,
    //pub system :System,
}

impl Platform {
    pub fn new(name:String) -> Platform {
        Platform {
            name,
      //      system: actix::System::new(name),
        }
    }

    pub fn initialize(&self) {
        let system = actix::System::new(&self.name); 
    }

    // pub fn add_gateway(&self, &gateway){

    // }

    pub  fn start(&self) {
        // self.system.run();
        System::run( || {
            let addr = MockActor.start(); // <- start actor and get its address
            let res = addr.send(Sum(10usize, 5usize));

            // match res {
            //     Ok(result) => println!("SUM: {}", result),
            //     _ => println!("Communication to the actor has failed"),
            // }
        });
    }

    pub fn stop(&self) {
        System::current().stop(); // <- stop system
    }
}


