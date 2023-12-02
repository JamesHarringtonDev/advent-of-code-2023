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

    let numbers = [
        "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
        "seven", "eight", "nine",
    ];

    let lines = read_file_line_by_line(&file_path);

    let mut total: i32 = 0;

    for line in lines {
        let mut first_index = line.len();
        let mut first_value = 0;

        let mut second_index = 0;
        let mut second_value = 0;

        for (index, element) in numbers.iter().enumerate() {
            if line.find(element) != None {
                let numerical_value = if index > 8 { index - 8 } else { index + 1 };
                let first_index_in_line = line.find(element).unwrap();
                let last_index_in_line = line.rfind(element).unwrap();

                if first_index_in_line <= first_index {
                    first_index = first_index_in_line;
                    first_value = numerical_value as i32;
                }

                if last_index_in_line >= second_index {
                    second_index = last_index_in_line;
                    second_value = numerical_value as i32;
                }
            }
        }

        println!("Total for {} = {}", line, 10 * first_value + second_value);

        total += 10 * first_value + second_value;
    }

    println!("Final Total: {}", total)
}
