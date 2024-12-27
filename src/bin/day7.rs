use std::fs::read_to_string;


fn main(){
    part_one();
}


fn part_one(){
    let mut res: i64 = 0;
    for  line in read_to_string("./input.txt").unwrap().lines(){

        let data = line.trim().split(":").collect::<Vec<&str>>();
       
        let left = data[0].trim().parse::<i64>().unwrap();
        
        let mut values: Vec<i64> = Vec::new();
        
        for val in data[1].split_whitespace(){
            
           
            values.push(val.trim().parse::<i64>().unwrap());
        }

        if is_valid(left, &values, 0, 0){
            res+=left;
        }
    }
    println!("{}",res);
}
// two operators : + , *

fn is_valid(expected_result : i64 , values : &Vec<i64>,current_res :i64,index :usize) -> bool{
    if current_res > expected_result{
        return false;
    }
    if index == values.len(){
        return current_res == expected_result;
    }

    let current_val = values[index];

    let plus = is_valid(expected_result, &values, current_res+current_val, index+1);
    let mul = is_valid(expected_result, &values, current_res*current_val, index+1);

    return plus || mul;

}

