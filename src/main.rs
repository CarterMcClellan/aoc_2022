use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut totals: Vec<i32> = Vec::new();

    if let Ok(lines) = read_lines("./dec_1_input.txt") {
        
        let mut curr_elf = 0;
        for line in lines {
            if let Ok(ip) = line {
                if ip == "" {
                    totals.push(curr_elf);
                    curr_elf = 0;
                } else {
                    let v = match ip.parse::<i32>() {
                        Ok(i) => i,
                        Err(_e) => panic!("failed to parse w. error {}", _e)
                    };

                    curr_elf += v;
                }
                
            }
        }
    }
    

    println!("Totals {:?}", totals);
    if let Some(max_val) = totals.iter().max() {
        println!("max total {}", max_val);

        // if this was good code we would error handle arrays of len < 3
        totals.sort();
        let top_3: i32 = totals.iter().rev().take(3).sum();
        println!("top 3 total {}", top_3);
    }
    
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}