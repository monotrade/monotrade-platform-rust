pub struct Platform {
    pub name: String,
}

impl Platform {
    pub fn new(name:String) -> Platform {
        Platform {
            name,
        }
    }
    
    pub fn initialize(&self) {
        let system = actix::System::new(&self.name);

        system.run();
    }
}


