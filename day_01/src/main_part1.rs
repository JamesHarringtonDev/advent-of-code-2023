use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn read_file_line_by_line(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let lines = read_file_line_by_line(&file_path);

    let mut total = 0;

    for line in lines {
        let a = String::from(line);
        let characters: Vec<_> = a.chars().collect();

        let mut found_digita: Vec<char> = Vec::new();

        for char in characters {
            if char.is_numeric() {
                found_digita.push(char)
            }
        }

        let number_string: i32 = 10 * ((found_digita[0].to_string()).parse::<i32>().unwrap())
            + ((found_digita[found_digita.len() - 1].to_string())
                .parse::<i32>()
                .unwrap());

        total += number_string;
    }

    println!("Final Total: {}", total)
}
