use std::fs::read_to_string;
use regex::Regex;


fn main(){
    let mut memo = String::new();
    for line in read_to_string("./input.txt").unwrap().lines(){
        memo.push_str(line);

    }
    // mul(n,m) 9 characters
    
    
    let re = Regex::new(r"mul\([1-9][0-9]*,[1-9][0-9]*\)").unwrap();
    let mut res = 0;
  
    for possible_match in re.find_iter(&memo).map(|s| s.as_str()){
        println!("{}",possible_match);
        let input = Regex::new(r"[0-9][0-9]*").unwrap();
        let mut p = 1;
        for x in input.find_iter(possible_match).map(|s| s.as_str()){
            p = p * x.parse::<i32>().unwrap();
        }
        res += p;
    }
    println!("{}",res);

}


// use std::fs::read_to_string;
// use regex::Regex;

// fn main() {
//     // Read the input file
//     let memo = read_to_string("./input.txt").expect("Failed to read the file");
    
//     // Regex to match mut(n,m) where n and m are two-digit numbers
//     let re = Regex::new(r"mul\((\d{2}),(\d{2})\)").expect("Invalid regex");
    
//     let mut res = 0;

//     // Find matches and calculate the product
//     for caps in re.captures_iter(&memo) {
//         // Extract the captured groups
//         let n: i32 = caps[1].parse().expect("Failed to parse n");
//         let m: i32 = caps[2].parse().expect("Failed to parse m");
//         res += n * m;
//     }

//     println!("{}", res);
// }
