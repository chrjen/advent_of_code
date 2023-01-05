#[derive(Debug, Clone, Copy)]
pub enum KeypadMove {
    Up,
    Down,
    Right,
    Left,
}

impl KeypadMove {
    pub fn from_char(c: char) -> KeypadMove {
        match c {
            'U' | 'u' => KeypadMove::Up,
            'D' | 'd' => KeypadMove::Down,
            'L' | 'l' => KeypadMove::Left,
            'R' | 'r' => KeypadMove::Right,
            _ => panic!("'{c}' is not a valid KeypadMove"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum KeypadButton {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl KeypadButton {
    pub fn do_move(self, key_move: KeypadMove) -> Self {
        use KeypadButton::*;
        use KeypadMove::*;

        match (self, key_move) {
            (Two, Left) | (Four, Up) => One,
            (One, Right) | (Five, Up) | (Three, Left) => Two,
            (Two, Right) | (Six, Up) => Three,
            (One, Down) | (Five, Left) | (Seven, Up) => Four,
            (Two, Down) | (Six, Left) | (Eight, Up) | (Four, Right) => Five,
            (Three, Down) | (Five, Right) | (Nine, Up) => Six,
            (Four, Down) | (Eight, Left) => Seven,
            (Seven, Right) | (Five, Down) | (Nine, Left) => Eight,
            (Eight, Right) | (Six, Down) => Nine,

            (One, Up)
            | (One, Left)
            | (Two, Up)
            | (Three, Up)
            | (Three, Right)
            | (Four, Left)
            | (Six, Right)
            | (Seven, Left)
            | (Seven, Down)
            | (Eight, Down)
            | (Nine, Down)
            | (Nine, Right) => self,
        }
    }

    pub fn to_char(self) -> char {
        match self {
            KeypadButton::One => '1',
            KeypadButton::Two => '2',
            KeypadButton::Three => '3',
            KeypadButton::Four => '4',
            KeypadButton::Five => '5',
            KeypadButton::Six => '6',
            KeypadButton::Seven => '7',
            KeypadButton::Eight => '8',
            KeypadButton::Nine => '9',
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ExtendedKeypadButton {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Alpha,
    Beta,
    Charlie,
    Delta,
}

impl ExtendedKeypadButton {
    pub fn do_move(self, key_move: KeypadMove) -> Self {
        use ExtendedKeypadButton::*;
        use KeypadMove::*;

        match (self, key_move) {
            (Three, Up) => One,
            (Three, Left) | (Six, Up) => Two,
            (One, Down) | (Four, Left) | (Seven, Up) | (Two, Right) => Three,
            (Eight, Up) | (Three, Right) => Four,
            (Six, Left) => Five,
            (Two, Down) | (Seven, Left) | (Alpha, Up) | (Five, Right) => Six,
            (Three, Down) | (Eight, Left) | (Beta, Up) | (Six, Right) => Seven,
            (Four, Down) | (Nine, Left) | (Charlie, Up) | (Seven, Right) => Eight,
            (Eight, Right) => Nine,
            (Six, Down) | (Beta, Left) => Alpha,
            (Seven, Down) | (Charlie, Left) | (Delta, Up) | (Alpha, Right) => Beta,
            (Eight, Down) | (Beta, Right) => Charlie,
            (Beta, Down) => Delta,

            (One, Up)
            | (One, Right)
            | (One, Left)
            | (Two, Up)
            | (Two, Left)
            | (Four, Up)
            | (Four, Right)
            | (Five, Up)
            | (Five, Down)
            | (Five, Left)
            | (Nine, Up)
            | (Nine, Down)
            | (Nine, Right)
            | (Alpha, Down)
            | (Alpha, Left)
            | (Charlie, Down)
            | (Charlie, Right)
            | (Delta, Down)
            | (Delta, Right)
            | (Delta, Left) => self,
        }
    }

    pub fn to_char(self) -> char {
        use ExtendedKeypadButton::*;
        match self {
            One => '1',
            Two => '2',
            Three => '3',
            Four => '4',
            Five => '5',
            Six => '6',
            Seven => '7',
            Eight => '8',
            Nine => '9',
            Alpha => 'A',
            Beta => 'B',
            Charlie => 'C',
            Delta => 'D',
        }
    }
}
