mod big_integer;

use std::io;
use big_integer::BigInt;

fn main() {
    let number = "3213456459846213216541321654964321654684321321321321321654";
    let x = BigInt::new(number);

}

pub fn get_input() -> String {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_goes_into_input_above) => {}
        Err(_no_updates_is_fine) => {}
    }
    input.trim().to_string()
}
