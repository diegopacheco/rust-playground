pub fn execute(){

    let mut x = 1;
    while x<=10 {
        println!("2 * {} == {}",x,x*2);
        x=x+1;
    }

    x=1;
    loop{
        println!("3 * {} == {}",x,x*3);
        x=x+1;
        if x > 10 { break };
    }

    for x in 1..11{
        println!("4 * {} == {}",x,x*4);
    }

}