pub struct BrazillianStudent<'a>{
    pub name: &'a str,
    pub university: &'a str,
    pub gituser: &'a str,
    pub favlang: &'a str,
}

pub trait Person {
    fn name(&self) -> String;
}

pub trait Student: Person {
    fn university(&self) -> String;
}

pub trait Programmer {
    fn fav_language(&self) -> String;
}

pub trait CompSciStudent: Programmer + Student {
    fn git_username(&self) -> String;
}

impl Person for BrazillianStudent<'_> {
    fn name(&self) -> String {
        return String::from(self.name);
    }
}

impl Student for BrazillianStudent<'_>{
    fn university(&self) -> String {
        return String::from(self.university);
    }
}

impl Programmer for BrazillianStudent<'_>{
    fn fav_language(&self) -> String{
        return String::from(self.favlang);
    }
}

impl CompSciStudent for BrazillianStudent<'_>{
    fn git_username(&self) -> String {
        return String::from(self.gituser);
    }
}

pub fn comp_sci_student_greeting(student: &dyn CompSciStudent) -> String {
    format!(
        "My name is {} and I attend {}. My Git username is {} I love {}",
        student.name(),
        student.university(),
        student.git_username(),
        student.fav_language(),
    )
}

pub fn print_sci_student_greeting(student: &dyn CompSciStudent) {
    println!("{}", comp_sci_student_greeting(student));
}