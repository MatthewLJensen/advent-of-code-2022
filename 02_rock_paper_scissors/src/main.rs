use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

// this whole program would be a lot cleaner if I put the chars into an array and then just did array math to determine corresponding winner, score, etc.

fn main() {
    // Create a path to the desired file
    let path = Path::new("input.txt");
    let display = path.display();

    let file = match File::open(&path) {
        Err(why) => panic!(
            "couldn't open {}: {}",
            display,
            <dyn Error>::to_string(&why)
        ),
        Ok(file) => file,
    };

    let reader = BufReader::new(file);
    let lines = reader.lines();

    let mut total_score = 0;
    for l in lines {
        let line = l.unwrap().trim().to_string();
        // println!("play: {}", line);
        // println!("win score: {}", determine_win_score(line.chars().nth(0).unwrap(), line.chars().nth(2).unwrap()));
        // println!("play score: {}", determine_play_score(line.chars().nth(2).unwrap()));
        total_score +=
            calculate_round_score(line.chars().nth(0).unwrap(), determine_strat(line.chars().nth(0).unwrap(), line.chars().nth(2).unwrap()));
    }

    println!("Total: {}", total_score);
}

fn calculate_round_score(opponent: char, strat: char) -> i32 {
    return determine_win_score(opponent, strat) + determine_play_score(strat);
}

fn determine_win_score(opponent: char, strat: char) -> i32 {
    if opponent == 'A' {
        if strat == 'Y' {
            return 6;
        } else if strat == 'X' {
            return 3;
        } else {
            return 0;
        }
    }
    if opponent == 'B' {
        if strat == 'Z' {
            return 6;
        } else if strat == 'Y' {
            return 3;
        } else {
            return 0;
        }
    }
    if opponent == 'C' {
        if strat == 'X' {
            return 6;
        } else if strat == 'Z' {
            return 3;
        } else {
            return 0;
        }
    }
    0 // shouldn't get here
}

fn determine_play_score(strat: char) -> i32 {
    if strat == 'X' {
        return 1;
    } else if strat == 'Y' {
        return 2;
    } else {
        return 3;
    }
}

fn determine_strat(opponent: char, desire: char) -> char {
    //X: lose
    //Y: draw
    //Z: win

    if opponent == 'A' {
        if desire == 'X' {
            return 'Z';
        } else if desire == 'Y' {
            return 'X';
        } else {
            return 'Y';
        }
    } else if opponent == 'B' {
        if desire == 'X' {
            return 'X';
        } else if desire == 'Y' {
            return 'Y';
        } else {
            return 'Z';
        }
    } else {
        // opponene == 'C'
        if desire == 'X' {
            return 'Y';
        } else if desire == 'Y' {
            return 'Z';
        } else {
            return 'X';
        }
    }
}
