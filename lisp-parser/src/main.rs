use std::io::Write;

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
            for val in &sub_stack{
                let i_val= val.parse().unwrap();
                ivals.push(i_val);
            }

            match op{
                "+" => {
                    stack.push(
                        ivals.
                            iter().
                            fold(0,|acc,&x|acc+x)
                    );
                },
                _ => {
                    eprintln!("unknown op {:?}", op);
                    std::process::exit(1);
                }
            }

            println!("{:?}",sub_stack);
        } else{
            stack.push(token);
        }
    }
}
