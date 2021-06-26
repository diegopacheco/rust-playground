use std::str::FromStr;

fn main() {
    let lisp_code = "(+ 1 2 (+ 3 4))";
    let tokens:Vec<&str> = lisp_code.split_whitespace().collect();
    println!("{:?}",tokens);

    let mut stack:Vec<&str> = Vec::new();
    for token in tokens{
        if token==")"{
            let mut sub_stack:Vec<&str> = Vec::new();
            loop{
                let t = match stack.pop(){
                    None => break,
                    Some(t) => t
                };
                if t=="("{
                    continue;
                }
                sub_stack.push(t);
                if t=="+"{
                    break;
                }
            }

            let op = sub_stack.pop().unwrap();
            let mut ivals:Vec<i32> = Vec::new();
            for val in sub_stack{
                let i_val = match i32::from_str(val){
                   Err(e) => {
                       writeln!(std::io::stderr(),"could not parse number");
                       std::process::exit(1);
                    },
                    Ok(v) => v
                };
                ivals.push(i_val);
            }
            println!("{:?}",sub_stack);
        }
    }
}
