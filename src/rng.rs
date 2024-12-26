// USAGE: 
use std::env;


fn random_float(min_range : f64, max_range : f64) {

}

fn random_double(min_range : f64, max_range : f64) {

}

fn random_integer(min_range : f64, max_range : f64) {

}

fn random_unsigned_integer(min_range : f64, max_range : f64) {

}

fn random_byte(min_range : f64, max_range : f64) {

}

fn random_short(min_range : f64, max_range : f64) {

}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

fn usage() {
    println!("USAGE: rng <SUBCOMMAND> [optional MIN_RANGE-MAX_RANGE]");
    println!("SUBCOMMANDS:");
    println!("\t-f : to return a random float");
    println!("\t-d : to return a random double");
    println!("\t-i : to return a random integer");
    println!("\t-u : to return a random unsigned integer");
    println!("\t-b : to return a random byte");
    println!("\t-s : to return a random short");
    println!("\tMIN_RANGE : a number to represent the lower inclusive bound for the RNG");
    println!("\tMAX_RANGE : a number to represent the upper inclusive bound for the RNG");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let rng = task_rng();
    print_type_of(rng);

    if args.len() < 2 { usage(); } 
    else if args.len() == 2 {
        let argument : &str = args[1].as_str();
        match argument {
            "-f" => { random_float(0.0, 100.0); },
            "-d" => { random_double(0.0, 100.0); },
            "-i" => { random_integer(0.0, 100.0); },
            "-u" => { random_unsigned_integer(0.0, 100.0); },
            "-b" => { random_byte(0.0, 100.0); },
            "-s" => { random_short(0.0, 100.0); },
            _ => { 
                println!("Unrecognized command: {argument}");
                usage();
            },
        }
    }
}