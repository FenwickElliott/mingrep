use std::env;

fn main() {

    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        println!("fatal: minigrep requries at least 2 arguments");
        std::process::exit(1);
    }

    let target_file = &args[1];
    let search_term = &args[2];

    println!("target file: {}", target_file);
    println!("search term: {}", search_term);

    let file_content = match std::fs::read_to_string(target_file) {
        Ok(cont) => cont,
        Err(_) => {
            println!("fatal: error reading file");
            std::process::exit(1);
        }
    };

    println!("{}", file_content);
}
 