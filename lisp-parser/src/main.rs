fn tokenize(lisp_code:&String) -> Vec<&str> {
    lisp_code.split_whitespace().collect()
}

fn main() {
    let lisp_code = "(+ 1 2 (+ 3 4))".to_string();
    let tokens = tokenize(&lisp_code);
    println!("{:?}",tokens);

    let mut stack:Vec<String> = Vec::new();
    for token_ref in tokens{
        let token = token_ref.to_string();
        if token==")"{
            let mut sub_stack:Vec<String> = Vec::new();
            loop{
                let t = match stack.pop(){
                    None => break,
                    Some(t) => t
                };
                if t=="("{
                    continue;
                }
                sub_stack.push(t.clone());
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

            match op.as_ref(){
                "+" => {
                    let total = ivals.
                        iter().
                        fold(0,|acc,&x|acc+x);
                    stack.push(total.to_string());
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
