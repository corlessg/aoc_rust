use std::fs::File;
use std::io::{self,BufRead};
use std::path::Path;



fn main() {

    //read the lines in
    //execute a function to look at the first and last values  
    
    if let Ok(mut lines) = read_lines("./data/data.txt") {
        let mut counter = 0;
        for line in lines {
            let unwrapped = line.unwrap().replace("\"","");
            let split: Vec<&str> = unwrapped.split(",").collect();
            let firstComp: Vec<&str> = split[0].split("-").collect();
            let secondComp: Vec<&str> = split[1].split("-").collect();

            //Version 1
            // if strToInt(firstComp[0]) <= strToInt(secondComp[0])
            //     && strToInt(firstComp[1]) >= strToInt(secondComp[1]){
            //     println!("{:?} 1 {:?}",firstComp[0],secondComp[0]);
            //     counter = counter + 1;
            // } else if strToInt(firstComp[0]) >= strToInt(secondComp[0]) 
            //     && strToInt(firstComp[1]) <= strToInt(secondComp[1]) {
            //     println!("{:?} 2 {:?}",firstComp[0],secondComp[0]);
            //     counter = counter + 1;
            // }
            
            //Version 2
            if strToInt(firstComp[0]) >= strToInt(secondComp[0]) 
                && strToInt(firstComp[0]) <= strToInt(secondComp[1]){
                counter = counter + 1;
            } else if strToInt(firstComp[1]) >= strToInt(secondComp[0]) 
                && strToInt(firstComp[1]) <= strToInt(secondComp[1]) {
                counter = counter + 1;
            } else if strToInt(secondComp[0]) >= strToInt(firstComp[0]) 
            && strToInt(secondComp[0]) <= strToInt(firstComp[1]){
            counter = counter + 1;
        } else if strToInt(secondComp[1]) >= strToInt(firstComp[0]) 
            && strToInt(secondComp[1]) <= strToInt(firstComp[1]) {
            counter = counter + 1;
        }
            

        };
        println!("{}",counter);
        
    }
}

fn strToInt(input: &str) -> i32 {
    input.parse::<i32>().unwrap()
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



