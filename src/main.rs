extern crate rand;
use std::io;
// use rand::thread_rng;
// use rand::Rng;

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Copy, Clone)]
enum Token {
    X,
    O
}

/// gets a space's friendly string for printing
fn get_space_str(space: &Option<Token>, space_number: usize) -> String {
    match space {
        None => String::from(space_number.to_string()),
        Some(Token::O) => String::from("O"),
        Some(Token::X) => String::from("X"),
    }
}

struct Board {
    state: [Option<Token>; 9],
    current_player: Option<Token>
}

impl Board {
    fn set_current_player(&mut self, player: Option<Token>) {
        self.current_player = player;
    }

    // prints the current board state 
    fn print_board(&self) {
        println!("Current Player: {}", get_space_str(&self.current_player, 0));

        println!("-------------");
        for mut space_number in 0..3 {
            space_number = space_number * 3;

            println!("| {} | {} | {} |", 
                get_space_str(&self.state[space_number], space_number + 1), 
                get_space_str(&self.state[space_number + 1], space_number + 2), 
                get_space_str(&self.state[space_number + 2], space_number + 3)
            );

            println!("-------------");
        }
    }

    /// sets a space on the board to an X or O
    fn set_space(&mut self, space_number: usize) {
        self.state[space_number - 1] = self.current_player;
    }

    /// sets all spaces on board back to None
    // fn clear_board(&mut self) {
    //     for space in self.state.iter_mut() {
    //         *space = None;
    //     }
    // }

    /// checks board state to see if all spaces are full
    fn all_spaces_used(&self) -> bool {
        for space in self.state.iter() {
            match space {
                None => return false,
                Some(Token::O) => continue,
                Some(Token::X) => continue,
            }
        }

        true
    }

    // determines if a player has won 
    /*
    0 1 2 
    3 4 5
    6 7 8
    */
    /// determines if a player has won
    fn is_won(&self) -> bool {
        for mut space_number in 0..3 {
            // check column
            if self.state[space_number] != None &&  
                self.state[space_number] == self.state[space_number + 3] && 
                self.state[space_number] == self.state[space_number + 6] {
                // win
                return true;
            }
            // increase space_number here to chceck the next row
            space_number = space_number * 3;

            // check row
            if self.state[space_number] != None &&
                self.state[space_number] == self.state[space_number + 1] && 
                self.state[space_number] == self.state[space_number + 2]{
                // win
                return true;
            }            
        }

        // check the diagonals
        if (self.state[0] != None && self.state[0] == self.state[4] &&  self.state[0] == self.state[8]) || 
            (self.state[2] != None && self.state[2] == self.state[4] && self.state[2] == self.state[6]) {
            // win
            return true;
        }

        false
    }

    fn get_board_space_input(&self) -> usize {
        println!("Please enter a space number between 1 and 9");
    
        loop {
            let mut space_number = String::new();
    
            io::stdin()
                .read_line(&mut space_number)
                .expect("Failed to read line");
        
            let space_number: usize = match space_number.trim().parse() {
                Ok(sn) => {
                    if sn < 1 || sn > 9 {
                        println!("Please enter a number between 1 and 9");
                        continue;
                    } else {
                        sn
                    }
                },
                Err(_) => {
                    println!("Please enter a number between 1 and 9");
                    continue;
                }
            };

            // check to see if the space number is available
            if self.state[space_number - 1] != None {
                // unavailable - try again 
                continue;
            } else {
                return space_number;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_won_bottom_row() {
        let b = Board {
            state: [None, None, None, None, None, None, Some(Token::X), Some(Token::X), Some(Token::X)],
            current_player: Some(Token::X)
        };
        assert_eq!(b.is_won(), true);
    }

    #[test]
    fn test_is_won_diagonal() {
        let b = Board {
            state: [Some(Token::O), None, None, None, Some(Token::O), None, None, None, Some(Token::O)],
            current_player: Some(Token::X)
        };
        assert_eq!(b.is_won(), true);
    }

    #[test]
    fn test_all_spaces_used_not_used() {
        let b = Board {
            state: [Some(Token::O), None, None, None, Some(Token::O), None, None, None, Some(Token::O)],
            current_player: Some(Token::X)
        };
        assert_eq!(b.all_spaces_used(), false);
    }

    #[test]
    fn test_all_spaces_used_used() {
        let b = Board {
            state: [Some(Token::O), Some(Token::O), Some(Token::O), Some(Token::O), Some(Token::O), Some(Token::O), Some(Token::O), Some(Token::O), Some(Token::O)],
            current_player: Some(Token::X)
        };
        assert_eq!(b.all_spaces_used(), true);
    }
}

// fn flip_coin() -> u8 {
//     let mut rng = thread_rng();
//     let coin: u8 = rng.gen_range(0..2);
//     coin
// }

fn main() {
    println!("Let's play Tic Tac Toe!\nChoose a number on the board to place an X or O");

    // init the board with empty spaces and X as the current player. X goes firsst
    let mut b = Board {
        state: [None, None, None, None, None, None, None, None, None],
        current_player: Some(Token::X)
    };

    loop {
        // print the board at the start
        b.print_board();

        // get the player's input
        let player_input = b.get_board_space_input();

        // set that space on the board
        b.set_space(player_input);

        // check for win & tie states 
        if b.is_won() {
            // current player wins! exit
            b.print_board();
            println!("Player {} has won! WooHoo!", get_space_str(&b.current_player, 0));
            break;
        } else if b.all_spaces_used() {
            b.print_board();
            println!("Game is a tie!");
            break;
        }

        // swap the current player to the other token
        if b.current_player == Some(Token::O) {
            b.set_current_player(Some(Token::X));
        } else {
            b.set_current_player(Some(Token::O));
        }
    }
}
