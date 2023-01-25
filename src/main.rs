mod day1;
use std::env;
fn main() {
    let args = read_arguments();
    println!("CMD Arguments: {:?}",args);
    day1::run(&args.get(1).unwrap())
}

fn read_arguments() -> Vec<String> {
    env::args().collect::<Vec<String>>()
}


