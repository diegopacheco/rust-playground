
mod basic;
mod dynamic;
mod supers;

fn main() {
    basic::fun_with_traits();
    dynamic::fun_dynamic();

    let student = supers::BrazillianStudent{
        name: "Diego",
        university: "Ulbra",
        gituser: "diegopacheco",
        favlang: "Rust",
    };
    supers::print_sci_student_greeting(&student);
}
