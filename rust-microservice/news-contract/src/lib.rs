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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        let n = News{
            id:String::from("1"),
            desc:String::from("google"),
            url:String::from("google.com"),
        };
        println!("{}",n);
    }

}