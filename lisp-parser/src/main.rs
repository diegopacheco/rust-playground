use std::collections::HashMap;

type OpFn = fn(Vec<String>) -> String;

fn plus(args:Vec<String>) -> String {
    println!("args {:?}",args);
    let iargs:Vec<i32> = args.iter().map(|s| s.parse().unwrap()).collect();
    iargs.iter().fold(0,|acc,&x|acc+x).to_string()
}

fn minus(args:Vec<String>) -> String {
    let mut iargs:Vec<i32> = args.iter().map(|s| s.parse().unwrap()).collect();
    let first = iargs.pop().unwrap();
    iargs.iter().fold(first,|acc,&x|acc-x).to_string()
}

fn multiply(args:Vec<String>) -> String {
    let iargs:Vec<i32> = args.iter().map(|s| s.parse().unwrap()).collect();
    iargs.iter().fold(1,|acc,&x|acc*x).to_string()
}

fn tokenize(lisp_code:&String) -> Vec<&str> {
    lisp_code.split_whitespace().collect()
}

fn evaluate(code:String,ops:HashMap<String,OpFn>) -> String {
    let tokens = tokenize(&code);
    println!("tokens {:?}",tokens);

    let mut stack:Vec<String> = Vec::new();
    for token_ref in tokens{
        let token = token_ref.to_string();

        if token == "("{
            // do nothing by design
        }else if token==")"{
            let mut sub_stack:Vec<String> = Vec::new();
            loop{
                let t = match stack.pop(){
                    None => break,
                    Some(t) => t
                };
                sub_stack.push(t.clone());
                if ops.contains_key(&t){
                    break;
                }
            }
            let mut op = sub_stack.pop().unwrap();
            op = op.replace(")", "");
            op = op.replace("(", "");

            let mut ivals:Vec<i32> = Vec::new();
            for val in &sub_stack{
                let mut val_clean = val.replace(")", "");
                val_clean = val_clean.replace("(", "");
                println!("val_clean {:?}",val_clean);
                let i_val= val_clean.parse().unwrap_or(0);
                ivals.push(i_val);
            }

            let result = ops.get(&op).unwrap()(sub_stack);
            println!("sub_eval result is == {:?}",result);
            stack.push(result);
        } else{
            stack.push(token);
        }
    }

    assert!(stack.len()==1);
    stack.pop().unwrap()
}

fn ops() -> HashMap<String,OpFn>{
    let mut ops:HashMap<String,OpFn> = HashMap::new();
    ops.insert(String::from("+"),plus);
    ops.insert(String::from("-"),minus);
    ops.insert(String::from("*"),multiply);
    ops
}

fn main() {
    let lisp_code = "( + 1 2 ( + 3 4 ) )".to_string();
    let result = evaluate(lisp_code.to_string(),ops());
    println!("{} == {:?}", &lisp_code, result);
}

#[test]
fn evaluate_test(){
    assert_eq!(evaluate("( + 1 2 ( + 3 4 ) )".to_string(),ops()),"10");
}

#[test]
fn plus_test(){
    assert_eq!(plus(vec!["1".to_string(),"2".to_string(),"3".to_string()]),"6");
}

#[test]
fn minus_test(){
    assert_eq!(minus(vec!["4".to_string(),"10".to_string()]),"6");
}

#[test]
fn multiply_test(){
    assert_eq!(multiply(vec!["6".to_string(),"10".to_string()]),"60");
}