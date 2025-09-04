#[derive(Debug)]

pub struct Student (pub u32, pub String, pub  String);


pub fn id(student: &Student) -> u32 {
    let Student(id, _, _) =  student;
    return *id ;

}

pub fn first_name(student: &Student) -> &str {
    let Student(_, first_name, _) =  student;
    return first_name;
}

pub fn last_name(student: &Student) -> &str {
     let Student(_, _, last_name) =  student;
     return last_name;
}