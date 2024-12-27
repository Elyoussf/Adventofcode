use std::{collections::HashMap, fs::read_to_string, num};
fn main(){
    part_one();
}
#[derive(Hash,Eq,PartialEq)]
struct Position{
    x: i32,
    y: i32,
}
// impl Position{
//     fn equal(&self , e2 : Position) -> bool{
//         return self.x==e2.x && self.y==e2.y
//     }
// }
fn part_one(){
    
    
    
    let mut  Antennas_pos: HashMap<char,Vec<Position>> = HashMap::new();
    let mut n = 0;
    let mut m = 0;
    for (ln,line) in read_to_string("./input.txt").unwrap().lines().enumerate(){
        n +=1;
        for (i,pos) in line.chars().enumerate(){
            if ln == 0 {
                m+=1;
            }
            if pos.is_alphanumeric(){
                Antennas_pos
                .entry(pos)
                .and_modify(|v| v.push(Position{x:i as i32,y:ln as i32})).
                or_insert(vec![Position{x:i as i32,y:ln as i32}]);
            }
        }
        
    }


// At this stage i ahve a hashmap with the positions of the antennas 
// i can now start looking for antinodes



}

fn find_Antinodes_and_mark_them(Positions : Vec<Position>, antenna_label : char,mut seen_antinodes : HashMap<Position,bool>,n:usize,m:usize) -> i32{
    let n = Positions.len();
    let mut res = 0;
    // For every two antennas we have at most two antinode 
    // we need just to verify these two positions are they belong to te map 
    // the antenna below plus the transition beteen the two antennas
    // the antenna above minus the same transition to the other antenna  
    for i in 0..n-1{
        for j in i+1..n{
            let dist_x = (Positions[i].x-Positions[j].x).abs();
            let dist_y = (Positions[i].y-Positions[j].y).abs();
            let mut down;
            let mut up;
            let mut left ;
            let mut right ;
            if  Positions[i].x<Positions[j].x{
                up = &Positions[i];
                down = &Positions[j];
            }else{
                down = &Positions[i];
                up = &Positions[j];
            }
            if  Positions[i].y<Positions[j].y{
                left = &Positions[i];
                right = &Positions[j];
            }else{
                right = &Positions[i];
                left = &Positions[j];
            }
            if down == left{
                if (down.x + dist_x)  <n as i32 && down.y - dist_y >= 0{
                    match seen_antinodes.get(&Position{x:down.x + dist_x,y:down.y - dist_y}){

                        Some(ele) => continue,
                        None => {
                            res += 1;
                            seen_antinodes.entry(Position{x:down.x + dist_x,y:down.y - dist_y}).or_insert(true);
                        }
                    }
                }
                if (down.x + dist_x) <n && down.y 
            }
            
           
        }
    }
    return 0;

}