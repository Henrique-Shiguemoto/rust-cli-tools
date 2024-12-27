use std::env;
use rand::prelude::*;

fn random_value<T>(mut rng: rand::rngs::ThreadRng, min_range: f64, max_range: f64) -> T
where T: num_traits::NumCast, {
    let value = (rng.gen::<f64>() * (max_range - min_range + 1.0)) + min_range;
    T::from(value).unwrap_or_else(|| panic!("Failed to cast value to target type"))
}

fn usage() {
    println!("USAGE: rng <SUBCOMMAND> (optional)[MIN_RANGE:MAX_RANGE]");
    println!("SUBCOMMANDS:");
    println!("\t-f : to return a random float");
    println!("\t-d : to return a random double");
    println!("\t-i : to return a random integer");
    println!("\t-u : to return a random unsigned integer");
    println!("\t-b : to return a random byte");
    println!("\tMIN_RANGE : a number to represent the lower inclusive bound for the RNG");
    println!("\tMAX_RANGE : a number to represent the upper inclusive bound for the RNG");
    println!("If (optional)[MIN_RANGE:MAX_RANGE] isn't passed, then the assumed range will be 0 - 255");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let rng = rand::thread_rng();

    if args.len() < 2 { 
        usage(); 
    } else if args.len() == 3 || args.len() == 2 {
        let argument : &str = args[1].as_str();
        let mut min_range : f64 = 0.0;
        let mut max_range : f64 = 255.0;

        if args.len() == 3 {
            let range_pair = String::from(args[2].clone());
            let range_vector = range_pair.trim_matches(|c| c == '[' || c == ']').split(":").collect::<Vec<&str>>();

            min_range = range_vector[0].parse::<f64>().unwrap_or_else(|error| { panic!("ERROR: {error}"); });
            max_range = range_vector[1].parse::<f64>().unwrap_or_else(|error| { panic!("ERROR: {error}"); });
            if min_range >= max_range { panic!("ERROR: MIN_RANGE ({min_range}) has to be smaller than MAX_RANGE ({max_range})\n"); }
        }

        match argument {
            "-f" => { let number : f32 = random_value(rng, min_range, max_range); println!("Your number is {number}"); },
            "-d" => { let number : f64 = random_value(rng, min_range, max_range); println!("Your number is {number}"); },
            "-i" => { let number : i32 = random_value(rng, min_range, max_range); println!("Your number is {number}"); },
            "-u" => { let number : u32 = random_value(rng, min_range, max_range); println!("Your number is {number}"); },
            "-b" => { let number : u8  = random_value(rng, min_range, max_range); println!("Your number is {number}"); },
            _    => { println!("Unrecognized command: {argument}"); usage(); },
        }
    }
}
