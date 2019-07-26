pub fn execute(){

    trait Animal {
        fn name(&self) -> &'static str; 
        fn talk(&self){
            print!("{} cannot talk",self.name());
        }
    }

    struct Human{
        name: &'static str
    }

    impl Animal for Human {
        fn name(&self) -> &'static str {
            return self.name;
        }
        fn talk(&self){
            print!("{} says hello",self.name());
        }
    }

    struct Cat{
        name: &'static str
    }

    impl Animal for Cat {
        fn name(&self) -> &'static str {
            return self.name;
        }
        fn talk(&self){
            print!("{} meooow",self.name());
        }
    }


    let h = Human{name: "John"};
    print!("Traits fun Human.name == {} \n",h.name());
    h.talk();

    let c = Cat{name: "Melina"};
    print!("Traits fun Cat.name == {} \n",c.name());
    c.talk();
}