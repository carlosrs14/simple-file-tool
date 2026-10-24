use  std::{env, fs::File, io::{self, BufRead}};

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        println!("Error: You must provide a file name");
        return;
    }
    
    let file_name: String = String::clone(&args[1]);
    let file: Result<File, io::Error> = File::open(file_name);

    if file.is_err() {
        println!("Error: cant not read that file");
        return;
    }

    let option = get_optional_arg(&args);
    if option == 0 {
        println!("Error: You must provide an option");
        return;
    }

    match args[option].as_str() {
        "--count-char" => {
            count_chars(file.unwrap());
        },
        "--count-lines" => {
            count_lines(file.unwrap());
        },
        "--grep" => {
            if option + 2 > args.len() {
                println!("Error: must provide a word to find");
            } else {
                grep(file.unwrap(), &args[option]);
            }
        },
        _ => {
            println!("Error: invalid option");
        }
    }

}

fn get_optional_arg(args: &Vec<String>) -> usize {
    let mut i: usize = 0;

    while i < args.len() {
        if args[i].starts_with("--") {
            return  i;
        }
        i += 1;
    }
    return 0;
}

fn count_lines(file: File) -> usize {
    let mut line_counter: usize = 0;

    let mut reader = io::BufReader::new(file);
    let mut line = String::new();
    loop {
        line.clear();
        match reader.read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => line_counter += 1,
            Err(e) => {
                print!("Error reading line: {}", e);
                break;
            }
        }
    }
    println!("This file have {} lines", line_counter);
    return line_counter;
}

fn count_chars(file: File) -> usize {
    
    let mut char_counter: usize = 0;

    let mut reader = io::BufReader::new(file);
    let mut line = String::new();
    loop {
        line.clear();
        match reader.read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => char_counter += line.len(),
            Err(e) => {
                print!("Error reading line: {}", e);
                break;
            }
        }
    }
    println!("This file have {} chars", char_counter);
    return char_counter;
}

fn grep(_file: File, _word: &String) {
    // TODO: maybe when good at regex
}