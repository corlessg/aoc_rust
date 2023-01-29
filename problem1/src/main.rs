use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

//read text
//two values, currentSum, maxSum
//return maxSum
fn main() {

    // //This will read the entire file into memory as a string... very bad for large data sets
    // let path = Path::new("./data/input.txt");
    // let display = path.display();

    // let mut file = match File::open(&path) {
    //     Err(why) => panic!("couldnt read {}: {}", display, why),
    //     Ok(file) => file,
    // };
    
    // let mut s = String::new();
    // match file.read_to_string(&mut s) {
    //     Err(why) => panic!("couldnt read {}: {}", display, why),
    //     Ok(_) => println!("{} contains \n {}", display, s),
    // }
    let mut current_sum: u64 = 0;
    let max_sum: u64;
    let mut curr_val: u64;
    let mut arr_val: [u64; 3] = [0,0,0];
    let mut i: usize;
    let mut j: usize;

    //Need to leverage an iterator
    if let Ok(lines) = read_lines("./data/input.txt"){
        for line in lines {
                if let Ok(line_str) = line {
                    if !line_str.is_empty() {
                        curr_val = line_str.parse().unwrap();

                        current_sum = current_sum + curr_val;
                    } 
                    else {
                        //create for loop for testing each part of the array, fill in if it changes
                        i = 0;
                        j = arr_val.len() - 1;
                        while i < arr_val.len(){
                            if arr_val[i] < current_sum{

                                while j > i{
                                    arr_val[j] = arr_val[j-1];
                                    j -= 1;
                                }
                                arr_val[i] = current_sum;
                                break;
                            }
                            i += 1;
                        }
                        // for x in arr_val.iter_mut(){
                            

                        //     if *x < current_sum{
                        //         * = *x;
                        //         *x = current_sum;
                        //         break;
                        //     }
                        //     i+=1;
                        // }
                        // println!("{}",current_sum);
                        current_sum = 0;
                    }
                };
        }
       }
       max_sum = arr_val[0] + arr_val[1] + arr_val[2];
       println!("{}",max_sum);
    }
    
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
