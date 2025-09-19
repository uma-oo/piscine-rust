#[derive(Debug, PartialEq, Clone)]
pub enum Role {
    CEO,
    Manager,
    Worker,
}

impl From<&str> for Role {
    fn from(s: &str) -> Role {
        match s {
            "CEO" => Role::CEO,
            "Manager" => Role::Manager,
            _ => Role::Worker,
        }
    }
}

#[derive(Debug)]
pub struct WorkEnvironment {
    pub grade: Link,
}

pub type Link = Option<Box<Worker>>; // Complete type alias

#[derive(Debug, Clone)]
pub struct Worker {
    pub role: Role,
    pub name: String,
    pub next: Link,
}

impl WorkEnvironment {
    pub fn new() -> Self {
        Self {
            grade: None,
        }
    }

    pub fn add_worker(&mut self, name: &str, role: &str) {
        match &self.grade {
            None => {
                self.grade = Some(
                    Box::new(Worker {
                        role: Role::from(role),
                        name: name.to_string(),
                        next: None,
                    })
                );
            }
            Some(other_grade) => {
                self.grade = Some(
                    Box::new(Worker {
                        role: Role::from(role),
                        name: name.to_string(),
                        next: Some(other_grade.clone()),
                    })
                );
            }
        };
    }

    pub fn remove_worker(&mut self) -> Option<String> {
        let mut value: Option<String>;
            match &self.grade{
                None => {
                    value = None; 
                    *self = WorkEnvironment{grade : None} ;
                }
                Some(other_grade) => {
                    value = Some(other_grade.name.clone());
                    *self = WorkEnvironment { grade: other_grade.next.clone()};

                }
            };
    

        value
    }

    pub fn last_worker(&self) -> Option<(String, Role)> {
        let mut value: Option<(String, Role)>;
        match &self.grade {
            None => {
                value = None;
            }
            Some(other_grade) => {
                value = Some((
                    other_grade.name.clone(),
                    other_grade.role.clone(),
                ));
            }
        }
        value
    }
}
