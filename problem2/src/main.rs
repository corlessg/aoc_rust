use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn main() {
   
    if let Ok(lines) = read_lines("./data/data.txt"){

        let mut total_points: i32 = 0;

        for line in lines{ 
            if let Ok(line_str) = line {
                // Version 1 of Elf Test
                // let split_line: Vec<&str> = line_str.split(" ").collect();
                // let opponent: &str = split_line[0];
                // let user: &str = split_line[1];
                
                // total_points = total_points + get_score(user) + get_winner(user,opponent);
                
                //Version 2 of Elf Test
                let split_line: Vec<&str> = line_str.split(" ").collect();
                let opponent: &str = split_line[0];
                let outcome: &str = split_line[1];

                total_points = total_points + get_outcome_score(outcome) + get_choice(opponent,outcome);
            }
        };
        
        println!("{}",total_points);
    }    
}

//Values for stuff
//1 rock        A   X
//2 paper       B   Y
//3 scissors    c   Z


fn get_score(val: &str) -> i32{
    match val {
        "A" | "X" => 1,
        "B" | "Y" => 2,
        "C" | "Z" => 3,
        _ => todo!(),
    } 
}

fn get_outcome_score(val: &str) -> i32{
    match val {
        "X" => 0,
        "Y" => 3,
        "Z" => 6,
        _ => todo!(),
    }
}

// fn get_winner(player: &str, opponent: &str) -> i32 {
//     if player == "X" && opponent == "A" {
//         3
//     } else if player == "X" && opponent == "B" {
//         0
//     }
//     else if player == "X" && opponent == "C" {
//         6
//     }else if player == "Y" && opponent == "A" {
//         6
//     }else if player == "Y" && opponent == "B" {
//         3
//     }else if player == "Y" && opponent == "C" {
//         0
//     }else if player == "Z" && opponent == "A" {
//         0
//     }else if player == "Z" && opponent == "B" {
//         6
//     }else if player == "Z" && opponent == "C" {
//         3
//     } else
//     {0}
// }

fn get_choice(opponent: &str, outcome: &str) -> i32{
    if outcome == "Y" {
        // Tie, so return the score from original function as it will be same to opponent
        get_score(opponent)
    } else if outcome == "X"{
        //Loss
        match opponent {
            "A" => 3,
            "B" => 1,
            "C" => 2,
            _ => todo!(),
        }
    } else if outcome == "Z" {
        //Win
        match opponent {
            "A" => 2,
            "B" => 3,
            "C" => 1,
            _ => todo!(),
        }
    }else
    {0}
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}