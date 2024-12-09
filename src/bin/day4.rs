use std::fs::read_to_string;

fn main(){
    let mut matrix: Vec<Vec<char>> = Vec::new();
    let src = read_to_string("./input.txt").unwrap();
    for line in src.lines(){
        matrix.push(line.chars().collect());
    }
    let n = matrix.len();
    let m = matrix[0].len();
    let mut res = 0;
    for i in 0..n{
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
                if j-1 >= 0 && matrix[i][j-1] == 'M' && matrix[i][j-2] == 'A' && matrix[i][j-3] == 'S'{
                    res+=1;
                }
                /*
                Horizontally
                 */

                // up to down 
                
            }
        }
    }
}