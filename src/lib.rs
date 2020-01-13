use std::str::FromStr;

#[derive(Debug)]
pub enum Operator {
  Add,
  Sub,
  Mul,
  Div,
  Int(isize),
  Float(f64),
  Print,
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
          println!("et {:?}", s);
          s
        }
    };
    Ok(s)
  }
}