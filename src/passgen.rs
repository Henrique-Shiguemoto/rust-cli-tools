use std::env;
use rand::prelude::*;
use regex::Regex;

fn usage() {
    println!("USAGE: passgen <SUBCOMMANDS> (optional)<SUBSUBCOMMANDS>");
    println!("SUBCOMMANDS:");
    println!("\t-lX          : sets the password length to X (X in [1, 255]), default is 12");
    println!("\t-Dsc         : disable special symbols, enabled by default");
    println!("\t-Dn          : disable numbers, enabled by default");
    println!("\t-Dlc         : disable lower case characters, enabled by default");
    println!("\t-Duc         : disable upper case characters, enabled by default");
    println!("\nSUBSUBCOMMANDS:");
    println!("\t-qX          : sets the quantity of passwords generated to X (X in [1, 255]), default is 1");
    println!("\t-bString     : sets the beginning of the password to String (size has to be in [1, 255] and smaller than X from -lX parameter), empty string by default");
}

fn create_password(rng: &mut rand::rngs::ThreadRng, characters_allowed: Vec<char>, password_size: usize, left_string: Vec<char>) -> String {
    let mut chars: Vec<char> = (0..password_size).map(|_| *characters_allowed.choose(rng).unwrap()).collect();
    let overwritten_characters_count: usize = left_string.len();
    for i in 0..overwritten_characters_count {
        chars[i] = left_string[i];
    }
    return chars.into_iter().collect();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut rng = rand::thread_rng();

    let numbers_character_set: &str = "0123456789";
    let lower_case_character_set: &str = "abcdefghijklmnopqrsuvwxyz";
    let upper_case_character_set: &str = "ABCDEFGHIJKLMNOPQRSUVWXYZ";
    let special_character_set: &str = "!\";#$%&'()*+,-./:;<=>?@[]^_`{|}~";
    let all_characters: &str = "0123456789abcdefghijklmnopqrsuvwxyzABCDEFGHIJKLMNOPQRSUVWXYZ!\";#$%&'()*+,-./:;<=>?@[]^_`{|}~";

    if 1 <= args.len() || args.len() <= 9 {
        if args.len() == 1 {
            let password = create_password(&mut rng, all_characters.chars().collect(), 12, "".chars().collect());
            println!("Passwords:");
            println!("\t{password}");

            println!("For more settings: type passgen -help");
            return;
        }

        if args.len() == 2 {
            if args[1] == "-help" {
                usage();
                return;
            }
        }

        let character_set_allowed: String;
        let mut password_size: i32 = 12;
        let mut password_quantity: i32 = 1;
        let left_side_string: &str;
        let pattern_to_find_lx = Regex::new(r"^-l.*").unwrap();
        let pattern_to_find_qx = Regex::new(r"^-q.*").unwrap();
        let pattern_to_find_b_string = Regex::new(r"^-b.*").unwrap();

        let mut all_characters_filtered: String = String::from(all_characters);
        if args.contains(&String::from("-Dsc")) { all_characters_filtered = all_characters_filtered.chars().filter(|c| !special_character_set.contains(*c)).collect(); }
        if args.contains(&String::from("-Dn"))  { all_characters_filtered = all_characters_filtered.chars().filter(|c| !numbers_character_set.contains(*c)).collect(); }
        if args.contains(&String::from("-Dlc")) { all_characters_filtered = all_characters_filtered.chars().filter(|c| !lower_case_character_set.contains(*c)).collect(); }
        if args.contains(&String::from("-Duc")) { all_characters_filtered = all_characters_filtered.chars().filter(|c| !upper_case_character_set.contains(*c)).collect(); }
        character_set_allowed = all_characters_filtered;

        if character_set_allowed.len() == 0 {
            panic!("ERROR: You have to choose at least one character set.");
        }

        let lx_element: Option<&String> = args.iter().find(|s| pattern_to_find_lx.is_match(s));
        if lx_element.is_some() {
            let number_from_lx_as_string: String = lx_element.unwrap().strip_prefix("-l").expect("Invalid -lX parameter").to_string();
            let number_from_lx = number_from_lx_as_string.parse::<i32>().unwrap_or_else(|_error| { panic!("ERROR: Number passed to -lX parameter is invalid"); });
            password_size = number_from_lx;
            if password_size < 1 || password_size > 255 {
                panic!("ERROR: Number passed to -lX parameter is out of bounds (min = 1 and max = 255)");
            }
        }

        let qx_element: Option<&String> = args.iter().find(|s| pattern_to_find_qx.is_match(s));
        if qx_element.is_some() {
            let number_from_qx_as_string: String = qx_element.unwrap().strip_prefix("-q").expect("Invalid -qX parameter").to_string();
            let number_from_qx = number_from_qx_as_string.parse::<i32>().unwrap_or_else(|_error| { panic!("ERROR: Number passed to -qX parameter is invalid"); });
            password_quantity = number_from_qx;
            if password_quantity < 1 || password_quantity > 255 {
                panic!("ERROR: Number passed to -qX parameter is out of bounds (min = 1 and max = 255)");
            }
        }

        let b_string_element: Option<&String> = args.iter().find(|s| pattern_to_find_b_string.is_match(s));
        let mut string_from_b_string: String = String::from("");
        if b_string_element.is_some() {
            string_from_b_string = b_string_element.expect("Invalid -bString parameter").strip_prefix("-b").expect("Invalid -bString parameter").to_string();
            if (string_from_b_string.len() < 1 || string_from_b_string.len() > 255) && (string_from_b_string.len() < password_size as usize) {
                panic!("ERROR: Size of beginning subtring ({size}) is bigger than or equal to the size of the password {password_size}", size = string_from_b_string.len());
            }
        }
        left_side_string = &string_from_b_string;

        let mut passwords: Vec<String> = Vec::<String>::new();
        for _i in 0..password_quantity {
            let password_created: String = create_password(&mut rng, character_set_allowed.chars().collect(), password_size as usize, left_side_string.chars().collect());
            passwords.push(password_created);
        }

        println!("Passwords:");
        for pass in passwords { println!("\t{pass}"); }
    }
}