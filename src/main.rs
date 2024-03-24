use std::fs;
use std::io;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: program_name <filename>");
        std::process::exit(1);
    }

    let filename = &args[1];
    match count_lines(filename) {
        Ok(count) => println!("Number of lines in {} is {}",filename, count),
        Err(err) => eprintln!("Error: {}", err)
    }
}

fn count_lines (filename: &str) -> io:: Result<usize> {
    let content = fs::read_to_string(filename)?;
    Ok(content.lines().count())
}
