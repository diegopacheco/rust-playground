struct Point<T>{
    x: T,
    y: T
}

pub fn execute(){

    let p = Point { x: 10, y:20 };
    println!("{0}-{1}",p.x,p.y);

}