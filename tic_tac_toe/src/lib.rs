pub fn tic_tac_toe(table: [[char; 3]; 3]) -> String {
    let mut x_won = false;
    let mut o_won = false;
    if diagonals('X', table ) || horizontal('X', table) || vertical('X', table) {
       x_won = true
    } 
    if diagonals('O', table ) || horizontal('O', table) || vertical('O', table) {
       o_won = true
    }
    if x_won == o_won {
        return "tie".to_string();
    }
    if x_won {
        return  "player X won".to_string();
    }

    return  "player O won".to_string();
    


}

pub fn diagonals(player: char, table: [[char; 3]; 3]) -> bool {
           // let's check if the first one has the value
           // they can be both 
           let mut first_diagonal = true ;
           let mut second_diagonal= true ;

           for i in 0..table.len() {
                if table[i][i]!= player {
                    first_diagonal = false;
                    break;
                }
           }

           for i in 0..table.len(){
            if table[i][table.len()-1-i]!=player {
                second_diagonal = false;
                break;
            }
           }

        return  first_diagonal|| second_diagonal

}

pub fn horizontal(player: char, table: [[char; 3]; 3]) -> bool  {
    for i in 0..table.len() {
    let mut not_found = true;
    for j in 0..table[i].len() {
        if table[i][j]!= player {
            not_found = false;
            break
        }
    }
    if not_found {
        return true;
    }
    
   }
   false
   
}

pub fn vertical(player: char, table: [[char; 3]; 3]) -> bool {
    let mut j = 0;
    while j < table.len() {
        let mut not_found = true;
        for i in 0..table.len() {
            if table[i][j]!= player {
            not_found = false;
            break;
            }
        }
        if not_found {
            return true;
        }
        j+=1;
    }

    false
   
}