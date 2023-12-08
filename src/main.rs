use std::cmp;
use std::io::{self};
use std::str::FromStr;
use std::time::{UNIX_EPOCH, SystemTime};


/*
use rand::prelude::*;
*/
pub struct Gamefield {
    pub circle_pos: u16,
    pub cross_pos: u16,
    pub current: u8,
    cross_turn: bool,
    pub last_move: u8,
}

impl Gamefield {
    pub fn evaluate(&self, depth: u16 ) -> i32 {
        let win_state = binary_logic::check_win_state(&self.circle_pos, &self.cross_pos);
        if win_state == binary_logic::WinState::Draw {
            return 0;
        } else if win_state == binary_logic::WinState::Ongoing {
            return 0;
        }
        else if win_state == binary_logic::WinState::Circle {
            return 0xFF_FFFF + depth as i32;
        } else {
            return -0xFF_FFFFi32 - depth as i32;
        }
    }
    fn reset(&mut self) {
        self.current = 0;
    }
}

mod bin_logic;
use bin_logic::binary_logic;
impl Iterator for Gamefield {
    type Item = Gamefield;

    fn next(&mut self) -> Option<Gamefield> {
        
        if self.current > 8{
            return None;
        }


        if binary_logic::check_win_state(&self.circle_pos, &self.cross_pos) != binary_logic::WinState::Ongoing {
            return None;
        }

        while binary_logic::get_at_idx(self.current, &(self.cross_pos | self.circle_pos)) == 1 {
            self.current+=1;
            if self.current > 8 {
                return None;
            }
        }

        let mut res = Gamefield {
            current : 0,
            cross_turn : !self.cross_turn,
            last_move : self.current,
            ..*self
        };
        if self.cross_turn {

            binary_logic::place_at_idx(self.current, &mut res.cross_pos);
        } else {
            binary_logic::place_at_idx(self.current, &mut res.circle_pos);
        }
        self.current += 1;
        Some(res)

    }
}


fn minimax(position: &mut Gamefield, depth: u16, max_player: bool, mut alpha: Option<i32> , mut beta : Option::<i32> ) -> (i32, i32) {

if alpha == None {
    alpha = Some (i32::MIN);
}
if beta == None {
    beta = Some(i32::MAX);
}
    let position_eval = position.evaluate(depth);
    let mut iter = position.peekable();
    let has_items = iter.peek().is_some(); 
    if (depth == 0) | !has_items {
        return (position_eval, -1);
    }
    let mut best_idx: i32 = -1;
    position.reset();
    if max_player {
        let mut max_eval: i32 = i32::MIN;
        
        for mut field in position {
            let eval = minimax(&mut field, depth - 1, false, alpha, beta).0;
            if eval > max_eval {
                max_eval = eval;
                best_idx = field.last_move.into();      
            }
            
            alpha = cmp::max(alpha, Some( max_eval));

            if beta <= alpha {
                break;
            }
        }
        return (max_eval, best_idx);
    } 
    else 
    {
        let mut min_eval: i32 = i32::MAX;

        for mut field in position {
            let eval = minimax(&mut field, depth - 1, true, alpha, beta).0;
            if eval <= min_eval {
                min_eval = eval;
                best_idx = field.last_move.into();

            }            
            beta = cmp::min(beta,Some( min_eval));

            if beta < alpha {
                break;

            }
        }
        return (min_eval, best_idx);
    }

}





fn main() {
    println!("You can't beat me!! >:D");
    let mut current_field = Gamefield {
        circle_pos: 0,
        cross_pos: 0,
        current: 0,
        cross_turn: false,
        last_move: 69,
    };

    let first_move: bool;

    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) =>  {
            //println!("{}; {}",n.as_millis(),n.as_millis() % 2);
            first_move = (n.as_millis() % 2) == 1
        
        },
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
    if first_move {
        let move_pos: u8;
        match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(n) =>  move_pos = ((n.as_secs()) % 9) as u8,
            Err(_) => panic!("SystemTime before UNIX EPOCH!"),
        }


        //let mut rng = rand::thread_rng();
        binary_logic::place_at_idx(move_pos, &mut current_field.circle_pos);
        println!("I played the move {}, your turn", move_pos);




    } else {
        println!("You begin. Make a move!");
    }

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input_num: u8 = u8::from_str(input.trim()).unwrap();
    binary_logic::place_at_idx(input_num, &mut current_field.cross_pos);

    while binary_logic::check_win_state(&current_field.circle_pos, &current_field.cross_pos) == binary_logic::WinState::Ongoing {
        current_field.current = 0;

        let play = minimax(&mut current_field, 10, false, None, None).1;
        binary_logic::place_at_idx(play as u8, &mut current_field.circle_pos);
        match binary_logic::check_win_state(&current_field.circle_pos, &current_field.cross_pos) {
            binary_logic::WinState::Circle => {
                println!("I played {}, and hereby won, try again!", play);
                let _ = io::stdin().read_line(&mut input);
                return;
            }
            binary_logic::WinState::Draw => {
                println!("Draw, nice game!");
            }
            binary_logic::WinState::Ongoing => {
                println!("Nice move, you haven't lost yet. If you do, I'll take over the world >:D\nI played {}", play);
            }
            _ => {
                println!("Error!");
            }
        }

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input_num: u8 = u8::from_str(input.trim()).unwrap();
        binary_logic::place_at_idx(input_num, &mut current_field.cross_pos);



        

    }

    println!("I can't loose ?!");
    let _ =  io::stdin().read_line(&mut input);
}