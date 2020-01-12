extern crate serde; 
extern crate serde_json;

use serde::{Serialize, Deserialize};
use std::fmt::{self, Formatter, Display};

#[derive(Serialize, Deserialize, Debug)]
pub struct News{
    pub id:String,
    pub desc:String,
    pub url:String,
}

impl Display for News {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}:{}:{}",self.id, self.desc, self.url)
    }
}
