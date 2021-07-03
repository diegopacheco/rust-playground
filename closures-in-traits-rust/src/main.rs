#[derive(Debug)]
struct Profile{
    id: u128,
    name: String,
    twitter_url: String,
}

impl Profile{

    fn new(id:u128,name:String,twitter_url:String)->Self{
        Profile{
            id,
            name,
            twitter_url
        }
    }

    fn mutate<M>(&mut self,mutator:M)
    where
        M: Fn(&mut Profile)
    {
        mutator(self);
    }
}

fn main() {
    let mut oniyflans = Profile::new(1,
                                 "Only Awesome Flans".to_string(),
                                 "https://twitter.com/oniyflans".to_string());
    oniyflans.mutate(|profile|{
        profile.name = "Only Flans".to_string();
    });
    println!("{:?}",oniyflans);
}
