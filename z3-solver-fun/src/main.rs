use z3::{Solver, ast::Int};

fn main() {
    let solver = Solver::new();
	let x = Int::new_const("x");
	solver.assert((&x + 4).eq(7));
	_ = solver.check();
	let model = solver.get_model().unwrap();
	println!("{model:?}");
}
