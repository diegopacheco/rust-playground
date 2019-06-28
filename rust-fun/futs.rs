use std::sync::Future;

fn main() {
	
	let mut delayed_value = Future::spawn(proc() {
    	// just return anything for examples' sake
    	12345i
	});
	
	println!("value = {}", delayed_value.get());

}