use crate::OS::Linux;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
    height: u32,
    weight: u32,
    prefered_os: OS,
}

#[derive(Debug,Copy, Clone)]
enum OS{
    Linux,
    Windows,
    MacOs
}

impl Person{
    fn new(name:String,age:u8,height:u32,weight:u32,prefered_os:OS) -> Self {
        Person{
            name: name,
            age: age,
            height: height,
            weight: weight,
            prefered_os: prefered_os
        }
    }
    fn with_name(mut self,name:&str) -> Self {
        self.name=name.to_string();
        self
    }
    fn with_age(mut self,age:u8) -> Self {
        self.age=age;
        self
    }
    fn with_height(mut self,height:u32) -> Self {
        self.height=height;
        self
    }
    fn with_weight(mut self,weight:u32) -> Self {
        self.weight=weight;
        self
    }
    fn with_os(mut self,os:OS) -> Self {
        self.prefered_os=os;
        self
    }
    fn build(mut self) -> std::result::Result<Self,String> {
        let os = self.prefered_os;
        if matches!(os,OS::MacOs){
            let mut msg = "No noney for this kind of os like: ".to_string();
                msg.push_str(OS::MacOs.to_string().as_str());
            Err(msg)
        }else{
            Ok(self)
        }
    }
}

impl Default for Person{
    fn default() -> Self {
        Self {
            name: "John Doe".to_string(),
            age: 40,
            height: 57,
            weight: 130,
            prefered_os: Linux
        }
    }
}

impl Display for OS{
    fn fmt(&self, f: &mut Formatter) -> Result {
        let printable:String = match *self {
            OS::Linux => "Linux".to_string(),
            OS::Windows => "Windows".to_string(),
            OS::MacOs => "MacOs".to_string(),
        };
        write!(f, "{}", printable)
    }
}

impl Display for Person {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Name:{} Age:{} Height:{} Weight:{} PreferedOS:{}",
               self.name, self.age,self.height,self.weight,self.prefered_os)
    }
}

fn main() {
    let john = Person::default().build().unwrap();
    println!("John == {}",&john);

    let mary = Person::default().with_name("Mary")
                                       .with_os(OS::Windows)
                                       .with_age(50)
                                       .build().unwrap();
    println!("Mary == {}",&mary);

    if let Err(e) = Person::default().with_os(OS::MacOs).build(){
        println!("Failed to create person - {}",e);
    }
}
