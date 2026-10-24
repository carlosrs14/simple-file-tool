use  std::{env, fs::File, io};

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
    

}
