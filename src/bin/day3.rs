use std::fs::read_to_string;
use regex::Regex;


// fn main(){
//     let mut memo = String::new();
//     for line in read_to_string("./input.txt").unwrap().lines(){
//         memo.push_str(line);

//     }
//     // mul(n,m) 9 characters
    
    
//     let re = Regex::new(r"mul\([1-9][0-9]*,[1-9][0-9]*\)").unwrap();
//     let mut res = 0;
  
//     for possible_match in re.find_iter(&memo).map(|s| s.as_str()){
//         println!("{}",possible_match);
//         let input = Regex::new(r"[0-9][0-9]*").unwrap();
//         let mut p = 1;
//         for x in input.find_iter(possible_match).map(|s| s.as_str()){
//             p = p * x.parse::<i32>().unwrap();
//         }
//         res += p;
//     }
//     println!("{}",res);

// }




//part 2:

fn main(){
    let mut memo = String::new();
    for line in read_to_string("./input.txt").unwrap().lines(){
        memo.push_str(line);

    }
    // mul(n,m) 9 characters
    
    
    let re = Regex::new(r"mul\([1-9][0-9]*,[1-9][0-9]*\)|do\(\)|don't\(\)").unwrap();
    let mut res = 0;
    let mut enabled = true;
    for possible_match in re.find_iter(&memo).map(|s| s.as_str()){
        println!("{}",possible_match);
        if possible_match.chars().nth(2).unwrap() == '('{ // if it's do()
            enabled = true;
            continue;
        }
        
        if possible_match.chars().nth(2).unwrap() == 'n' {
            enabled = false;
            continue;
        }
        if !enabled{
            continue;
        }
        //Here i am sure that ot is the pattern mul(x,y)

        let input = Regex::new(r"[0-9][0-9]*").unwrap();
        let mut p = 1;
        for x in input.find_iter(possible_match).map(|s| s.as_str()){
            p = p * x.parse::<i32>().unwrap();
        }
        res += p;
    }
    println!("{}",res);

}
