use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use std::collections::BinaryHeap;


// Sln Complexity
// Notation: 
// L -> Number of Lines in the File
// E -> Number of Elves in the File

// Optimal
// using a binary heap
// O(L)

// using a vector
// max(O(L), O(E * log E)

const  CAPACITY: usize = 3;

fn main() {
    let mut totals = BinaryHeap::new(); // fuckery -> Binary Heap w. Capacity does not enforce capacity?

    if let Ok(lines) = read_lines("./dec_1_input.txt") {
        
        let mut curr_elf = 0;
        for line in lines {
            if let Ok(ip) = line {
                if ip == "" {
                    if totals.len() < CAPACITY {
                        totals.push(-curr_elf);
                    }

                    else if let Some(curr_min) = totals.peek() {
                        if -1 * (*curr_min) < curr_elf {
                            totals.pop();
                            totals.push(-curr_elf);
                        }
                    }

                    curr_elf = 0;
                } else {
                    if let Ok(v) = ip.parse::<i32>() {
                        curr_elf += v;
                    }
                }
                
            }
        }
    }

    let top_3: Vec<i32> = totals.into_sorted_vec().iter().map(|x| -x).collect();
    println!("Top 3 {:?}", top_3);
    let top_3_total: i32 = top_3.iter().sum();

    // Sln 1
    println!("Sln 1 {}", top_3[0]);

    // Sln 2
    println!("Sln 2 {}", top_3_total);
    
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}