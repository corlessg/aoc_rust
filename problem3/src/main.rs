use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use ascii_converter::*;


fn main() {

    let mut total_val: u32 = 0;

    if let Ok(lines) = read_lines("./data/data.txt") {       
        
        for line in lines{
            
            if let Ok(line_str) = line {

                let (lin1, lin2) = line_str.split_at(line_str.len()/2);
                // println!("{} and {}",lin1,lin2)
                let vec1 = string_to_decimals(lin1).unwrap();
                let vec2 = string_to_decimals(lin2).unwrap();
                let same: u8 = compareArrays(vec1,vec2);

                let mut val: u32 = 0;
                if same >= 97 {
                    val = (same as u32) - 96;
                } else {
                    val = (same as u32) - 64 + 26;
                }
                total_val = total_val + val;


            }
            
        }

        println!("{}",total_val);
    };

}

fn compareArrays(vec1: Vec<u8>, vec2: Vec<u8>) -> u8{
    for x in vec1.iter().enumerate(){
        for y in vec2.iter().enumerate(){
            if x.1 == y.1 {
                return *x.1;
            }                
        }
    }
    return 0;
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// fn stringMap(input: &str, !<Vec>)


// convert ascii to number 
// a-z -> ascii - 96
// A-Z -> ascii - 64 + 26
/*
Count the len
Cut in half to two strings
Iterate through strings to find match
Perhaps use ascii for quick counter of alphabet values
Sum for each row
*/