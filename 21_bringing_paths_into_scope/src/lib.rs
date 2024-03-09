use std::collections::HashMap;
use std::fmt;
use std::fmt::Result;
use std::io;
use std::io::Result as IoResult;
use std::io::{self, Write};
use std::collections::*;


mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}


mod customer {
    use crate::front_of_house::hosting;

    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist()
    }
}

fn test() {
    let mut map = HashMap::new();

    map.insert(1, 2);
}

fn function1() -> fmt::Result {

}

fn function2() -> io::Result<()>{
    
}

fn function3() -> Result {

}

fn function4() -> IoResult{
    
}