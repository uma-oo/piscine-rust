use std::fmt::{ self, Debug };
use std::cmp::{ Ord, Ordering };
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub enum Antigen {
    A,
    AB,
    B,
    O,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum RhFactor {
    Positive,
    Negative,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseBloodTypeError;

#[derive(PartialEq, Eq, PartialOrd)]
pub struct BloodType {
    pub antigen: Antigen,
    pub rh_factor: RhFactor,
}

impl FromStr for Antigen {
    type Err = ParseBloodTypeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let blood_type = match s {
            "A" => Antigen::A,
            "B" => Antigen::B,
            "O" => Antigen::O,
            "AB" => Antigen::AB,
            _ => {
                return Err(ParseBloodTypeError);
            }
        };
        Ok(blood_type)
    }
}

impl FromStr for RhFactor {
    type Err = ParseBloodTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.chars().count() < 2 || s.chars().count() > 3 {
            return Err(ParseBloodTypeError);
        }

        let rh_factor = match s {
            "-" => RhFactor::Negative,

            "+" => RhFactor::Positive,
            _ => {
                return Err(ParseBloodTypeError);
            }
        };

        Ok(rh_factor)
    }
}

impl Ord for BloodType {
    fn cmp(&self, other: &Self) -> Ordering {
        self.antigen.cmp(&other.antigen).then(self.rh_factor.cmp(&other.rh_factor))
    }
}

impl FromStr for BloodType {
    type Err = ParseBloodTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.chars().count() < 2 || s.chars().count() > 3 {
            return Err(ParseBloodTypeError);
        }

        let rh_factor = match
            s
                .chars()
                .nth(s.chars().count() - 1)
                .unwrap()
        {
            '-' => RhFactor::Negative,

            '+' => RhFactor::Positive,
            _ => {
                return Err(ParseBloodTypeError);
            }
        };

        let bolld_type = match &s[0..s.len() - 1] {
            "A" => Antigen::A,
            "B" => Antigen::B,
            "O" => Antigen::O,
            "AB" => Antigen::AB,
            _ => {
                return Err(ParseBloodTypeError);
            }
        };

        Ok(BloodType { antigen: bolld_type, rh_factor: rh_factor })
    }
}

impl Debug for BloodType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let antigen = match self.antigen {
            Antigen::A => "A",
            Antigen::B => "B",
            Antigen::O => "O",
            Antigen::AB => "AB",
        };

        let rh_type = match self.rh_factor {
            RhFactor::Negative => '-',
            RhFactor::Positive => '+',
        };

        write!(f, "{}{}", antigen, rh_type)
    }
}

impl BloodType {
    pub fn can_receive_from(&self, other: &Self) -> bool {
        for receiver in self.donors() {
            if receiver == *other {
                return true;
            }
        }
        false
    }

    pub fn donors(&self) -> Vec<Self> {
        let mut result = Vec::new();
        match self.rh_factor {
            RhFactor::Negative => {
                match self.antigen {
                    Antigen::A => {
                        result.push(BloodType {
                            antigen: Antigen::O,
                            rh_factor: RhFactor::Negative,
                        });
                        result.push(BloodType {
                            antigen: Antigen::A,
                            rh_factor: RhFactor::Negative,
                        });
                    }
                    Antigen::B => {
                        result.push(BloodType {
                            antigen: Antigen::O,
                            rh_factor: RhFactor::Negative,
                        });
                        result.push(BloodType {
                            antigen: Antigen::B,
                            rh_factor: RhFactor::Negative,
                        });
                    }
                    Antigen::O => {
                        result.push(BloodType {
                            antigen: Antigen::O,
                            rh_factor: RhFactor::Negative,
                        });
                    }
                    Antigen::AB => {
                        result.push(BloodType {
                            antigen: Antigen::O,
                            rh_factor: RhFactor::Negative,
                        });
                        result.push(BloodType {
                            antigen: Antigen::A,
                            rh_factor: RhFactor::Negative,
                        });
                        result.push(BloodType {
                            antigen: Antigen::B,
                            rh_factor: RhFactor::Negative,
                        });
                        result.push(BloodType {
                            antigen: Antigen::AB,
                            rh_factor: RhFactor::Negative,
                        });
                    }
                };
            }
            RhFactor::Positive => {
                match self.antigen {
                    Antigen::A => {
                        result.push(BloodType {
                            antigen: Antigen::O,
                            rh_factor: RhFactor::Negative,
                        });
                        result.push(BloodType {
                            antigen: Antigen::A,
                            rh_factor: RhFactor::Negative,
                        });
                        result.push(BloodType {
                            antigen: Antigen::O,
                            rh_factor: RhFactor::Positive,
                        });
                        result.push(BloodType {
                            antigen: Antigen::A,
                            rh_factor: RhFactor::Positive,
                        });
                    }
                    Antigen::B => {
                        result.push(BloodType {
                            antigen: Antigen::O,
                            rh_factor: RhFactor::Positive,
                        });
                        result.push(BloodType {
                            antigen: Antigen::B,
                            rh_factor: RhFactor::Positive,
                        });
                    }
                    Antigen::O => {
                        result.push(BloodType {
                            antigen: Antigen::O,
                            rh_factor: RhFactor::Negative,
                        });
                        result.push(BloodType {
                            antigen: Antigen::O,
                            rh_factor: RhFactor::Positive,
                        });
                    }
                    Antigen::AB => {
                        result.push(BloodType {
                            antigen: Antigen::O,
                            rh_factor: RhFactor::Negative,
                        });
                        result.push(BloodType {
                            antigen: Antigen::A,
                            rh_factor: RhFactor::Negative,
                        });
                        result.push(BloodType {
                            antigen: Antigen::B,
                            rh_factor: RhFactor::Negative,
                        });
                        result.push(BloodType {
                            antigen: Antigen::AB,
                            rh_factor: RhFactor::Negative,
                        });
                        result.push(BloodType {
                            antigen: Antigen::O,
                            rh_factor: RhFactor::Positive,
                        });
                        result.push(BloodType {
                            antigen: Antigen::A,
                            rh_factor: RhFactor::Positive,
                        });
                        result.push(BloodType {
                            antigen: Antigen::B,
                            rh_factor: RhFactor::Positive,
                        });
                        result.push(BloodType {
                            antigen: Antigen::AB,
                            rh_factor: RhFactor::Positive,
                        });
                    }
                };
            }
        }

        result
    }

    pub fn recipients(&self) -> Vec<BloodType> {
        let mut result = Vec::new();
        match self.rh_factor {
            RhFactor::Negative => {
                match self.antigen {
                    Antigen::A => {
                        result.push(BloodType {
                            antigen: Antigen::A,
                            rh_factor: RhFactor::Negative,
                        });
                        result.push(BloodType {
                            antigen: Antigen::AB,
                            rh_factor: RhFactor::Negative,
                        });
						  result.push(BloodType {
                            antigen: Antigen::AB,
                            rh_factor: RhFactor::Positive,
                        });
						 result.push(BloodType {
                            antigen: Antigen::A,
                            rh_factor: RhFactor::Positive,
                        });
                    }
                    Antigen::B => {
                        result.push(BloodType {
                            antigen: Antigen::B,
                            rh_factor: RhFactor::Negative,
                        });
                        result.push(BloodType {
                            antigen: Antigen::AB,
                            rh_factor: RhFactor::Negative,
                        });
						result.push(BloodType {
                            antigen: Antigen::AB,
                            rh_factor: RhFactor::Positive,
                        });
						result.push(BloodType {
                            antigen: Antigen::B,
                            rh_factor: RhFactor::Positive,
                        });
                    }
                    Antigen::O => {
                        result.push(BloodType {
                            antigen: Antigen::O,
                            rh_factor: RhFactor::Negative,
                        });
                    }
                    Antigen::AB => {
                        result.push(BloodType {
                            antigen: Antigen::AB,
                            rh_factor: RhFactor::Negative,
                        });
                    }
                };
            }
            RhFactor::Positive => {
                match self.antigen {
                    Antigen::A => {
                        result.push(BloodType {
                            antigen: Antigen::A,
                            rh_factor: RhFactor::Negative,
                        });

                        result.push(BloodType {
                            antigen: Antigen::AB,
                            rh_factor: RhFactor::Positive,
                        });
                    }
                    Antigen::B => {
                        result.push(BloodType {
                            antigen: Antigen::B,
                            rh_factor: RhFactor::Positive,
                        });
                        result.push(BloodType {
                            antigen: Antigen::AB,
                            rh_factor: RhFactor::Positive,
                        });
                    }
                    Antigen::O => {
                        result.push(BloodType {
                            antigen: Antigen::O,
                            rh_factor: RhFactor::Positive,
                        });
                        result.push(BloodType {
                            antigen: Antigen::B,
                            rh_factor: RhFactor::Positive,
                        });
                        result.push(BloodType {
                            antigen: Antigen::A,
                            rh_factor: RhFactor::Negative,
                        });
                        result.push(BloodType {
                            antigen: Antigen::AB,
                            rh_factor: RhFactor::Positive,
                        });
                    }
                    Antigen::AB => {
                        result.push(BloodType {
                            antigen: Antigen::AB,
                            rh_factor: RhFactor::Positive,
                        });
                    }
                };
            }
        }

        result
    }
}
