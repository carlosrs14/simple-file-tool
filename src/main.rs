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

    let mut line_counter: i32 = 0;

    let mut reader = io::BufReader::new(file.unwrap());
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
}
