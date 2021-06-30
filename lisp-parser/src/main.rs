use std::collections::{HashMap, VecDeque};

type OpFn = fn(VecDeque<String>) -> String;

fn inc(args:VecDeque<String>) -> String {
    println!("args {:?}",args);
    let iargs:Vec<i32> = args.iter().map(|s| s.parse().unwrap()).collect();
    "(".to_string() + &iargs.iter().map(|x| (x+1).to_string() ).
        collect::<Vec<String>>().join(",") + ")"
}

fn plus(args:VecDeque<String>) -> String {
    println!("args {:?}",args);
    let iargs:Vec<i32> = args.iter().map(|s| s.parse().unwrap()).collect();
    iargs.iter().fold(0,|acc,&x|acc+x).to_string()
}

fn minus(args:VecDeque<String>) -> String {
    let mut iargs:Vec<i32> = args.iter().map(|s| s.parse().unwrap()).collect();
    let first = iargs.pop().unwrap();
    iargs.iter().fold(first,|acc,&x|acc-x).to_string()
}

fn multiply(args:VecDeque<String>) -> String {
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
            let mut queue:VecDeque<String> = VecDeque::new();
            loop{
                let t = match stack.pop(){
                    None => break,
                    Some(t) => t
                };
                queue.push_front(t.clone());
                if ops.contains_key(&t){
                    break;
                }
            }
            let op = queue.pop_front().unwrap();
            let result = ops.get(&op).unwrap()(queue);
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
    ops.insert(String::from("inc"),inc);
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
    assert_eq!(plus(VecDeque::from(vec!["1".to_string(),"2".to_string(),"3".to_string()])),"6");
}

#[test]
fn minus_test(){
    assert_eq!(minus(VecDeque::from(vec!["4".to_string(),"10".to_string()])),"6");
}

#[test]
fn multiply_test(){
    assert_eq!(multiply(VecDeque::from(vec!["6".to_string(),"10".to_string()])),"60");
}

#[test]
fn inc_test(){
    assert_eq!(inc(VecDeque::from(vec!["1".to_string(),"2".to_string(),"3".to_string()])),"(2,3,4)");
    assert_eq!(inc(VecDeque::from(vec!["1".to_string()])),"(2)");
    assert_eq!(evaluate("( inc 2 3 )".to_string(),ops()),"(3,4)");
}