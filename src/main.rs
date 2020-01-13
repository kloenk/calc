use std::io::prelude::*;
use std::io;

use std::str::FromStr;

use calc;

fn main() {
    let mut input = String::new();
    loop {
        input.clear();
        print!(">> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).expect("foo");
        let input = input.trim();
        
        //for v in input.split_whitespace() {
        //    println!("{}", v);
        //}
        let input: Vec<calc::Operator> = input.split_whitespace().map(|v| { calc::Operator::from_str(v).unwrap() } ).collect();

        println!("{:?}", input);
    }

    //println!("{:?}", input);
}
