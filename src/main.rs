use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;

fn main() {
    println!("Hello, aoc_2024_2!");

    if let Ok(lines) = read_lines("./src/input.txt") {

        let mut count_safe: i64 = 0;

        // Consumes the iterator, returns an ( Optional) String
        for line in lines.flatten() {
            // println!("{}", line);

            let input_vec: Vec<i64> = line.split_whitespace()
            .map(|x| x.parse().expect("Not an integer!"))
            .collect();

            let mut safe = check_safe(&input_vec);
            
            // Begin Part 2:
            let mut index_to_remove = 0;
            while ! safe && index_to_remove < input_vec.len() {

                let one_removed = produce_array_ommitting_one(&input_vec, index_to_remove);
                safe = check_safe(&one_removed);

                index_to_remove += 1;
            }
            // End Part 2:

            if safe {
                count_safe += 1;
            }
        }

        println!("count_safe {count_safe}");

    } else {
        if let Ok(path) = env::current_dir() {
            println!("Error reading lines, the current directory is {}", path.display());
        } else {
            println!("Error reading lines, and can't print the current directory");

        }
    }
}



fn produce_array_ommitting_one( input_vec: &Vec<i64>, index_to_remove: usize) -> Vec<i64> {
    let mut copy = input_vec.clone();
    copy.remove(index_to_remove);
    return copy;
}


fn check_safe( input_vec: &Vec<i64>) -> bool {

    let length = input_vec.len();
    let diff = input_vec[1]-input_vec[0];
    let negative = diff < 0;
    let mut safe = valid_diff(diff, negative);
    println!("start i 0 length {length} negative {negative} diff {diff} safe {safe}");
    if safe {
        let addressable_length = length - 1; // We're going start loop one after first and end one before last entry.
        for i in 1..addressable_length {
            let diff = input_vec[i+1] - input_vec[i];
            safe = valid_diff(diff, negative);
            if ! safe {
                break;
            }
            println!("i {i} length {length} negative {negative} diff {diff} safe {safe}");
        }
    }

    return safe;
}

fn valid_diff(diff: i64, negative: bool ) -> bool {
    if diff.abs() < 1 || diff.abs() > 3 {
        return false;
    }
    if negative && diff >= 0 {
        return false;
    }
    if ! negative && diff < 0 {
        return false;
    }
    return true;
}

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}