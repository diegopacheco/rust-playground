use rand::Rng;

pub fn execute(){
    let mut rng = rand::thread_rng();
    let temperature:i16 = rng.gen_range(20, 100);

    if temperature<=30 {
        println!("Super cold. Temperature: {}", temperature);
    }else {
        println!("Super Hot. Temperature: {}", temperature);
    }
}