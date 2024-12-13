use std::{collections::HashMap, fs::read_to_string};
use regex::Regex;



fn part_one(){
    let re = Regex::new(r"^$").expect("error while creating regex");
  // There is two types of data : X|Y and x,x,x,x,x  they are separated by an empty line (look at the input)
    let mut order: HashMap<i32,Vec<i32>> = HashMap::new();
    // ele --> after_ele
    let mut split = false;
    let mut res = 0;
    for line in read_to_string("./input.txt").unwrap().split("\n"){
        if re.is_match(line) && !split{
            split = true;
            continue;
        }
        if !split { //order
            let mut x : i32 = 0;
            let mut y : i32 = 0;
            for (i,e ) in line.to_string().split("|").enumerate(){
                if i == 0{
                    x = e.parse().unwrap();
                }else {
                    y = e.parse().unwrap();
                }
            }
            order.entry(x).and_modify(|v| v.push(y)).or_insert(vec![y]);
        }else {
            // p1 , p2 , p3 ...

            
            let mut seen: HashMap<i32,bool> = HashMap::new();
            let size = line.split(",").filter(|_e| true).count();
            let mut mid = 0;
            let mut valid = true;
            // this loop will go over the pages register mid ; 
            // if it's valid  
            for (ord,page) in line.split(",").enumerate(){
                
                let val: i32 = page.parse().unwrap();
                
                if ord == size/2{
                    
                    mid = val;
                   
                }
                
                let v = order.get(&val).unwrap();
                for after in v{
                    if let Some(_) = seen.get(&after) { 
                        // means an element from the ones that should appear after "after" has been seen before
                        valid = false;
                        break;
                    }
                }
                seen.insert(val, true);

                
                
            }
            if valid{
                res+=mid;
            }
            

        }
        
    }
    println!("{}",res);



}

fn main(){
    part_one();
}