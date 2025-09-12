use std::{ collections::HashMap, num::ParseFloatError };

#[derive(Debug)]
pub struct Flag {
    // expected public fields
    pub short_hand: String,
    pub long_hand: String,
    pub desc: String,
}

impl Flag {
    pub fn opt_flag(name: &str, d: &str) -> Self {
        Self {
            desc: d.to_string(),
            long_hand: "--".to_owned() + &name.to_string(),
            short_hand: "-".to_owned() + &name.chars().nth(0).unwrap().to_string(),
        }
    }
}

pub type Callback = fn(&str, &str) -> Result<String, ParseFloatError>;

pub struct FlagsHandler {
    pub flags: HashMap<String, Callback>,
}

impl FlagsHandler {
    pub fn add_flag(&mut self, flag: Flag, func: Callback) {
        self.flags.insert(flag.short_hand, func);
        self.flags.insert(flag.long_hand, func);
    }

    pub fn exec_func(&self, input: &str, argv: &[&str]) -> Result<String, String> {
        let res :String = match self.flags[input](argv[0], argv[1]) {
            Ok(value)=> value,
            Err(err)=> return Err(err.to_string()),
        };
        Ok(res)
    }
}

pub fn div(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let a: f64 = match a.parse() {
        Ok(value) => value,
        Err(err) => {
            return Err(err);
        }
    };

    let b: f64 = match b.parse() {
        Ok(value) => value,
        Err(err) => {
            return Err(err);
        }
    };

    Ok((a / b).to_string())
}

pub fn rem(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let a: f64 = match a.parse() {
        Ok(value) => value,
        Err(err) => {
            return Err(err);
        }
    };

    let b: f64 = match b.parse() {
        Ok(value) => value,
        Err(err) => {
            return Err(err);
        }
    };

    Ok((a%b).to_string())
}
