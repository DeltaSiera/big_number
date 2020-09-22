mod big_integer;

use std::io;
use big_integer::BigInt;
use std::str::FromStr;

fn main() {
    let number = "37514637146751465174321763417654176546546546512316584";
    let mut x = BigInt::new(number);
    let y = BigInt::new("675146751465716741675149768814977675146577164741");
    x = &x >> 3741;
    x = &x * &y;
    println!("{}", x);
}

pub fn get_input() -> String {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_goes_into_input_above) => {}
        Err(_no_updates_is_fine) => {}
    }
    input.trim().to_string()
}
