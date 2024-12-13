use std::fs::read_to_string;
// The part one:
fn part_one(){
    let mut matrix: Vec<Vec<char>> = Vec::new();
    let src = read_to_string("./input.txt").unwrap();
    for line in src.lines(){
        matrix.push(line.chars().collect());
    }
    let n = matrix.len();
    let m = matrix[0].len();
    let mut res = 0;
    for i  in 0..n{
        for j in 0..m{
            // find x search diaglonally , backwards , vertically , horizontally XMAS
            if matrix[i][j] == 'X'{
                /* 
                 vertically:
                */
                // left to right:
                if j+3 < m &&  matrix[i][j+1] == 'M' && matrix[i][j+2] == 'A' && matrix[i][j+3] == 'S'{
                    res +=1;
                }

                //right to left (backwards)
                if j >= 3 && matrix[i][j-1] == 'M' && matrix[i][j-2] == 'A' && matrix[i][j-3] == 'S'{
                    res+=1;
                }
                /*
                Horizontally
                 */
               
                // up to down 
                if i+3 < n &&  matrix[i+1][j] == 'M' && matrix[i+2][j] == 'A' && matrix[i+3][j] == 'S'{
                    res +=1;
                }
                // down to up
                if i  >= 3 && matrix[i-1][j] == 'M' && matrix[i-2][j] == 'A' && matrix[i-3][j] == 'S'{
                    res+=1;
                }

                /*Diagonally */
                // main diagonal
                // up to down 
                if i+3 < n && j+3 < m && matrix[i+1][j+1] == 'M' && matrix[i+2][j+2] == 'A' && matrix[i+3][j+3] == 'S'  
                  {
                      res+=1;
                }
                // down up 
                if i  >= 3 && j as i32-3>=0 && matrix[i-1][j-1] == 'M' && matrix[i-2][j-2] == 'A' && matrix[i-3][j-3] == 'S'  {
                    res+=1;
                }

                //second diagonal 
                // up to down (j is decreasing m i is increasing)
                if i+3 <n && j  >= 3 && matrix[i+1][j-1] == 'M' && matrix[i+2][j-2] =='A' && matrix[i+3][j-3] == 'S'{
                    res+=1;
            }
                // down to up (i is decreasing j is increasing)
                if i  >=3 && j+3 < n && matrix[i-1][j+1] == 'M' && matrix[i-2][j+2] == 'A' && matrix[i-3][j+3] == 'S'{
                    res+=1;
            } 
                
        }
    }
   
    }
    println!("{}",res);
}

fn main(){
    part_two();
}



fn part_two(){
    let mut matrix: Vec<Vec<char>> = Vec::new();
    let src = read_to_string("./input.txt").unwrap();
    for line in src.lines(){
        matrix.push(line.chars().collect());
    }

    /*
    M.S
    .A.
    M.S
     */
    let n = matrix.len();
    let m = matrix[0].len();
    let mut res = 0;
    for i in 0..n{
        for j in 0..m{
            if matrix[i][j] == 'A'{
                let regular_diagonal = (i >= 1 && j >= 1 && matrix[i-1][j-1] == 'M' ) && (i <n-1 && j < m-1 && matrix[i+1][j+1] == 'S');
                let non_regular_diagnal = (i >= 1 && j >= 1 && matrix[i-1][j-1] == 'S' ) && (i <n-1 && j < m-1 && matrix[i+1][j+1] == 'M');

                let diagonal = regular_diagonal || non_regular_diagnal; // valid MAS (forzard or backward)

                let regular_anti_diagonal = (i >= 1 && j <m- 1 && matrix[i-1][j+1] == 'M' ) && (i <n-1 && j >= 1 && matrix[i+1][j-1] == 'S');
                let non_regular_anti_diagonal = (i >= 1 && j <m- 1 && matrix[i-1][j+1] == 'S' ) && (i <n-1 && j >= 1 && matrix[i+1][j-1] == 'M');

                let anti_diagnal = regular_anti_diagonal || non_regular_anti_diagonal;

                if anti_diagnal && diagonal {
                    res+=1;
                }
            }
        }

    }
    println!("{}",res);
}