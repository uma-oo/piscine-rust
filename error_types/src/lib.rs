use chrono::{ Local };

enum Description {
    Empty,
    Size,
    Combination,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Form {
    pub name: String,
    pub password: String,
}

impl Form {
    pub fn validate(&self) -> Result<(), FormError> {
        if self.name.is_empty() {
            return Err(FormError::new("name", self.name.clone(), "Username is empty"));
        }

        if self.password.chars().collect::<Vec<_>>().len() < 8 {
            return Err(
                FormError::new(
                    "password",
                    self.password.clone(),
                    "Password should be at least 8 characters long"
                )
            );
        }

        let mut has_special_characters = false;
        let mut has_numbers = false;

        for c in self.password.clone().chars() {
            if !c.is_alphanumeric() {
                if c.is_ascii_punctuation() {
                    has_special_characters = true;
                } else {
                    return Err(
                        FormError::new(
                            "password",
                            self.password.clone(),
                            "Password should be a combination of ASCII numbers, letters and symbols"
                        )
                    );
                }
            } else if c.is_numeric() {
                has_numbers = true;
            } else {
                continue;
            }
        }
        if !has_special_characters || !has_numbers {
            return Err(
                FormError::new(
                    "password",
                    self.password.clone(),
                    "Password should be a combination of ASCII numbers, letters and symbols"
                )
            );
        }
        Ok(())
    }
}

// this will be the structure that wil handle the errors
#[derive(Debug, Eq, PartialEq)]
pub struct FormError {
    pub form_values: (&'static str, String),
    pub date: String,
    pub err: &'static str,
}

impl FormError {
    pub fn new(field_name: &'static str, field_value: String, err: &'static str) -> Self {
        Self {
            form_values: (field_name, field_value),
            date: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            err: err,
        }
    }
}
