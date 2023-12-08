pub mod binary_logic {
    #[inline(always)] //This is a simple function that should be inline to avoid unneccessary function calls
    pub fn place_at_idx(idx: u8, game_pos: &mut u16) { //This turns the idx bit of game_pos to on
        *game_pos |= 1 << idx;
    }
    #[inline(always)]
    pub fn zero_at_idx(idx: u8, game_pos: &mut u16) { //Turns the idx bit of game_pos off
        *game_pos &= !(1 << idx);
    }
    #[inline(always)]
    pub fn get_at_idx(idx: u8, game_pos: &u16) -> u16 { //Returns a 1 or 0 based what's in the current index
        *game_pos >> idx & 1
    }

    #[inline]
    pub fn check_for_win(game_pos: &u16) -> bool {
        for state in WIN_STATES {
            if (*game_pos & *state) == *state {
                return true;
            }
        }
        false
    }
    #[inline(never)]
    pub fn check_win_state(circle_pos: &u16, cross_pos: &u16) -> WinState {
        if check_for_win(circle_pos) {
            return WinState::Circle;
        } 
        else if check_for_win(cross_pos) {
            return WinState::Cross;
        }
        else if (*circle_pos & *cross_pos) == 0xFF { //Badically, every field is full
            return WinState::Draw;
        }
        WinState::Ongoing
    }
    #[derive(PartialEq)]
    pub enum WinState {
        Draw,
        Circle,
        Cross,
        Ongoing, //Nobody won or lost yet, but it hasn't been a draw
    }

    const WIN_STATES: &'static [u16] = &[

        0b111_000_000,

        0b000_111_000,

        0b000_000_111,

        0b100_100_100,

        0b010_010_010,

        0b001_001_001,

        0b100_010_001,

        0b001_010_100
    ];


    

}