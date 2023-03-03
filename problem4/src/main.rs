use std::fs::File;
use std::io::{self,BufRead};
use std::path::Path;



fn main() {

    //read the lines in
    //execute a function to look at the first and last values  
    
    if let Ok(mut lines) = read_lines("./data/test.txt") {
        let mut counter = 0;
        for line in lines {
            let unwrapped = line.unwrap().replace("\"","");
            let split: Vec<&str> = unwrapped.split(",").collect();
            println!("{}",split[0]);
            // for x in split{
            //     let split2: Vec<&str> = split.split("-").collect();
            // };
        };
        
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>,{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}



// fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
// where P: AsRef<Path>, {
//     let file = File::open(filename)?;
//     Ok(io::BufReader::new(file).lines())
// }


