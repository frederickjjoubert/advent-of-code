use std::error;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const FILE_PATH: &str = "./input_2.txt";

fn main() {
    println!("Part 1!");
    let result = part_1(FILE_PATH);
    println!("Result: {}", result);
}

fn part_1(file_path: &str) -> u32 {
    let mut total: u32 = 0;
    match read_lines(file_path) {
        Ok(lines) => {
            for line in lines {
                match line {
                    Ok(line) => {
                        let mut value: String = String::from("");
                        println!("{}", line);
                        for c in line.chars() {
                            if c.is_digit(10) {
                                value.push(c);
                                break;
                            }
                        }
                        for c in line.chars().rev() {
                            if c.is_digit(10) {
                                value.push(c);
                                break;
                            }
                        }
                        match value.parse::<u32>() {
                            Ok(number) => {
                                println!("Parsed number: {}", number);
                                total += number;
                            }
                            Err(error) => {
                                println!("Failed to parse: {}", error);
                            }
                        }
                    }
                    Err(error) => {
                        println!("Failed to parse: {}", error);
                    }
                }
            }
        }
        Err(e) => {
            println!("Error reading file: {}", e);
        }
    }

    total
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        let result = part_1(FILE_PATH);
        assert_eq!(result, 142);
    }
}
