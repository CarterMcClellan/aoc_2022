use std::{io::{self, BufRead}, fs::File, path::Path};

fn main() {
    let mut point_total = 0;
    let mut point_total_2 = 0;

    if let Ok(lines) = read_lines("./inputs/dec_2_input.txt") {
        for line in lines {
            if let Ok(turn) = line {
                let as_slice = &(*turn);
                // A -> Opponent Plays Rock
                // B -> Opponent Plays Paper
                // C -> Opponent Plays Scissors

                // X -> You Play Rock
                // Y -> You Play Paper
                // Z -> You Play Scissor

                // 1 for choosing Rock
                // 2 for choosing Paper
                // 3 for choosing Scissors

                // 0 for a loss
                // 3 for a draw
                // 6 for a win
                let score = match as_slice {
                    "A X" => 4, // Rock + Rock -> 1 + 3
                    "A Y" => 8, // Rock + Paper -> 2 + 6
                    "A Z" => 3, // Rock + Scissors -> 3 + 0

                    "B X" => 1, // Paper + Rock -> 1 + 0
                    "B Y" => 5, // Paper + Paper -> 2 + 3
                    "B Z" => 9, // Paper + Scissors -> 3 + 6

                    "C X" => 7, // Scissors + Rock -> 1 + 6
                    "C Y" => 2, // Scissors + Paper -> 2 + 0
                    "C Z" => 6, // Scissors + Scissors -> 3 + 3
                    _ => panic!("uh oh")
                };

                // Calculation 2

                // X -> You lose
                // Y -> You draw
                // Z -> You win

                let score2 = match as_slice {
                    "A X" => 3, // Rock + Scissors -> 3 + 0
                    "A Y" => 4, // Rock + Rock -> 1 + 3 
                    "A Z" => 8, // Rock + Paper -> 2 + 6

                    "B X" => 1, // Paper + Rock -> 1 + 0 
                    "B Y" => 5, // Paper + Paper -> 2 + 3 
                    "B Z" => 9, // Paper + Scissors -> 3 + 6

                    "C X" => 2, // Scissors + Paper -> 2 + 0
                    "C Y" => 6, // Scissors + Scissors -> 3 + 3
                    "C Z" => 7, // Scissors + Rock -> 1 + 6
                    _ => panic!("uh oh")
                };

                point_total += score;
                point_total_2 += score2;
            }
        }

        println!("Sln. 1 {}", point_total);
        println!("Sln. 2 {}", point_total_2);
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}