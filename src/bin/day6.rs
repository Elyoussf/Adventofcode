use std::fs::read_to_string;
// assume that the guard cqn alzqts exit
fn part_one(){
    enum Direction {
        Up,
        Down,
        Right,
        Left,
        
    }
    let mut current_direction : Direction = Direction::Up;
    let mut mtx: Vec<Vec<char>> = Vec::new();
    let mut found_guard_pos = false;
    let (mut x,mut y) = (0,0); 
    for (l,line) in read_to_string("./input.txt").unwrap().lines().enumerate(){
        mtx.push(line.chars().collect());
        
        if found_guard_pos{
            continue;
        }
        let mut i:usize = 0;
        let mut it = line.chars();
        while let Some(c) = it.next(){
            println!("c : {}",c);
            if c == '^' || c == '>' || c == '<' || c == 'v' {
                x = l;
                y = i;
                found_guard_pos = true;
                match c {
                    '^' => current_direction = Direction::Up,
                    '>' => current_direction = Direction::Right,
                    '<' => current_direction = Direction::Left,
                    'v' => current_direction = Direction::Down,
                    _=>{},
                }
                break;
            }
            i+=1;

        }
    }

    // At this point we have a matrix of the positions as well as the position of guard ze just need to simulate the movement

    let n = mtx.len();
    let m = mtx[0].len();
    let mut visited_pos = vec![vec![false;m];n];

    
    
    

    visited_pos[x][y] = true ; // initial position 
    let mut res = 1;
    // This loop simulates the guard walk 
    while x<n  && y<m {
        if !visited_pos[x][y]{
            visited_pos[x][y] = true;
            res+=1;
        }
        
        match current_direction {
            Direction::Up =>{
                if x >=1 {
                    if mtx[x-1][y] == '#' {
                        current_direction = Direction::Right;
                    }else {
                        x=x-1;
                    }
                }else{
                    break;
                }
            },
            Direction::Right =>{
                if y+1 < m {
                    if mtx[x][y+1] == '#' {
                        current_direction = Direction::Down;
                    }else {
                        y = y+1;
                    }
                }else{
                    y = y+1;
                }
            },
            Direction::Down =>{
                if x+1 < n {
                    if mtx[x+1][y] == '#' {
                        current_direction = Direction::Left;
                    }else {
                        x +=1;
                    }
                }else{
                    x = x+1;
                }
            },
            Direction::Left =>{
                if y >=1 {
                    if mtx[x][y-1] == '#' {
                        current_direction = Direction::Up;
                    }else {
                        y = y -1;
                    }
                }else{
                    break;
                }
            },
            
        }        
    }

    println!("{}",res);
    
    

}



fn main(){
    part_one();
}

