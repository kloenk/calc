use std::io;
use std::io::prelude::*;

use std::str::FromStr;

use calc;

fn main() {
    let mut input = String::new();
    let mut foo = Vec::new();
    loop {
        input.clear();
        print!(">> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).expect("foo");
        let input = input.trim();

        for v in input.split_whitespace() {
            foo.push(calc::Operator::from_str(v).unwrap());
            calc::Calculator::exe(&mut foo).unwrap();
        }
        //let mut foo: Vec<calc::Operator> = input
        //    .split_whitespace()
        //    .map(|v| calc::Operator::from_str(v).unwrap())
        //    .collect();

        
        //println!("{:?}", input);
    }

    //println!("{:?}", input);
}
