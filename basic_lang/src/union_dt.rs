union IntOrFloat {
    i:i32,
    f:f32
}

pub fn execute(){

    let mut iof = IntOrFloat { i: 10};
    println!("iof is {}", unsafe { iof.i } );
    
    iof.i = 11;
    println!("iof is {}", unsafe { iof.i } );


    fn proc(iof:IntOrFloat){
        unsafe{
            match iof {
                IntOrFloat{i:42} => {
                    println!("the Awnser for life");
                }
                IntOrFloat{f:_} => {
                    println!("You regular f == {}", iof.f);
                }
            }
        }
    }
    proc(IntOrFloat{i:42});
    proc(IntOrFloat{f:42.1});
}