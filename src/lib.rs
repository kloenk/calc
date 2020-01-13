use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Int(isize),
    Float(f64),
    Print,
}

impl Operator {
    fn isNumber(&self) -> bool {
      match self {
          Operator::Int(_) | Operator::Float(_) => true,
          _ => false,
      }
    }

    fn number(&self) -> f64 {
      match self {
        Operator::Int(v) => *v as f64,
        Operator::Float(v) => *v,
        _ => 0.0,
      }
    }
}

impl std::str::FromStr for Operator {
    type Err = String;

    fn from_str(s: &str) -> Result<Operator, Self::Err> {
        let s = match s {
            "+" => Operator::Add,
            "-" => Operator::Sub,
            "/" => Operator::Div,
            "*" => Operator::Mul,
            "p" => Operator::Print,
            _ => {
                let s: Operator = if let Ok(s) = isize::from_str(s) {
                    Operator::Int(s)
                } else {
                    f64::from_str(s).map(|v| Operator::Float(v)).unwrap()
                };
                s
            }
        };
        Ok(s)
    }
}

#[derive(Debug)]
pub struct Calculator {}

impl Calculator {
    pub fn exe(s: &mut Vec<Operator>) -> Result<(), String> {
        let exe = match s.pop() {
            Some(s) => s,
            None => return Err("utraen".to_string()),
        };

        match exe {
            Operator::Print => {
                println!("{:?}", s);
                Ok(())
            }
            Operator::Int(_) | Operator::Float(_) => {
              s.push(exe);
              Ok(())
            }
            Operator::Add => {
              if s.len() < 2 {
                return Err("aeuea".to_string());
              }
              let s1 = s.pop().unwrap();
              let s2 = s.pop().unwrap();

              if !s1.isNumber() || !s2.isNumber() {
                return Err("aeuaea".to_string());
              }

              let s1 = s1.number();
              let s2 = s2.number();

              s.push(Operator::Float(s1 + s2));
              Ok(())
            }
            Operator::Sub => {
              if s.len() < 2 {
                return Err("aeuea".to_string());
              }
              let s1 = s.pop().unwrap();
              let s2 = s.pop().unwrap();

              if !s1.isNumber() || !s2.isNumber() {
                return Err("aeuaea".to_string());
              }

              let s1 = s1.number();
              let s2 = s2.number();

              s.push(Operator::Float(s2 - s1));
              Ok(())
            }
            Operator::Div => {
              if s.len() < 2 {
                return Err("aeuea".to_string());
              }
              let s1 = s.pop().unwrap();
              let s2 = s.pop().unwrap();

              if !s1.isNumber() || !s2.isNumber() {
                return Err("aeuaea".to_string());
              }

              let s1 = s1.number();
              let s2 = s2.number();

              s.push(Operator::Float(s2 / s1));
              Ok(())
            }
            Operator::Mul => {
              if s.len() < 2 {
                return Err("aeuea".to_string());
              }
              let s1 = s.pop().unwrap();
              let s2 = s.pop().unwrap();

              if !s1.isNumber() || !s2.isNumber() {
                return Err("aeuaea".to_string());
              }

              let s1 = s1.number();
              let s2 = s2.number();

              s.push(Operator::Float(s2 * s1));
              Ok(())
            }
            _ => todo!(),
        }
    }
}
