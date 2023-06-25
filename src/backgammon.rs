use base64::{engine::general_purpose, Engine as _};
use std::ops::Add;

#[derive(Clone, Debug)]
pub struct State {
    board: [i32; 24],
    bar: (i32, i32),
    off: (i32, i32),
    is_white: bool,
}

impl State {
    pub fn new() -> Self {
        State {
            board: [
                -2, 0, 0, 0, 0, 5, // Player on roll's home board (points 1-6)
                0, 3, 0, 0, 0, -5, // Player on roll's outer board (points 7-12)
                5, 0, 0, 0, -3, 0, // Player not on roll's outer board (points 13-18)
                -5, 0, 0, 0, 0, 2, // Player not on roll's home board (points 19-24)
            ],
            bar: (0, 0),
            off: (0, 0),
            is_white: true,
        }
    }

    fn bar(&self, player: bool) -> i32 {
        match player {
            true => self.bar.0,
            false => self.bar.1,
        }
    }

    fn off(&self, player: bool) -> i32 {
        match player {
            true => self.off.0,
            false => self.off.1,
        }
    }

    fn flip(&self) -> State {
        State {
            board: self
                .board
                .iter()
                .rev()
                .map(|&num| -num)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
            bar: (self.bar.1, self.bar.0),
            off: (self.off.1, self.off.0),
            is_white: !self.is_white,
        }
    }

    pub fn decode(key: [u8; 10]) -> State {
        let mut bit_index = 0;
        let mut board = [0i32; 24];

        let mut white_bar = 0;
        let mut black_bar = 0;
        let mut white_pieces = 0;
        let mut black_pieces = 0;

        for point in (0..24).rev() {
            while (key[bit_index / 8] >> (bit_index % 8)) & 1 == 1 {
                board[point] -= 1;
                black_pieces += 1;
                bit_index += 1;
            }
            bit_index += 1; // Appending a 0
        }

        while (key[bit_index / 8] >> (bit_index % 8)) & 1 == 1 {
            black_bar += 1;
            bit_index += 1;
        }

        bit_index += 1; // Appending a 0

        for point in 0..24 {
            while (key[bit_index / 8] >> (bit_index % 8)) & 1 == 1 {
                board[point] += 1;
                white_pieces += 1;
                bit_index += 1;
            }
            bit_index += 1; // Appending a 0
        }

        while (key[bit_index / 8] >> (bit_index % 8)) & 1 == 1 {
            white_bar += 1;
            bit_index += 1;
        }

        State {
            board: board,
            bar: (white_bar, black_bar),
            off: (15 - white_pieces - white_bar, 15 - black_pieces - black_bar),
            is_white: true,
        }
    }

    pub fn from_id(id: String) -> State {
        let key = general_purpose::STANDARD.decode(id.add("==")).unwrap();
        State::decode(key.try_into().unwrap())
    }

    pub fn encode(&self) -> [u8; 10] {
        let mut key = [0u8; 10];
        let mut bit_index = 0;

        let game = match self.is_white {
            true => self.clone(),
            false => self.flip(),
        };

        // Encoding the position for the player not on roll
        for point in (0..24).rev() {
            for _ in 0..-game.board[point] {
                key[bit_index / 8] |= 1 << (bit_index % 8);
                bit_index += 1; // Appending a 1
            }
            bit_index += 1; // Appending a 0
        }
        for _ in 0..game.bar.1 {
            key[bit_index / 8] |= 1 << (bit_index % 8);
            bit_index += 1; // Appending a 1
        }
        bit_index += 1; // Appending a 0

        // Encoding the position for the player on roll
        for point in 0..24 {
            for _ in 0..game.board[point] {
                key[bit_index / 8] |= 1 << (bit_index % 8);
                bit_index += 1; // Appending a 1
            }
            bit_index += 1; // Appending a 0
        }
        for _ in 0..game.bar.0 {
            key[bit_index / 8] |= 1 << (bit_index % 8);
            bit_index += 1; // Appending a 1
        }

        key
    }

    pub fn position_id(&self) -> String {
        let key = self.encode();
        let b64 = String::from(general_purpose::STANDARD.encode(&key));
        b64[..14].to_string()
    }

    pub fn display(&self) {
        println!("Position ID: {}", self.position_id());
        println!("+13-14-15-16-17-18------19-20-21-22-23-24-+");
        for row in 0..5 {
            print!("|");
            for point in 12..=23 {
                Self::print_point(self.board[point], row);

                if point == 17 {
                    print!("|");
                    Self::print_point(-self.bar.1, row);
                    print!("|");
                }
            }
            print!("|");
            Self::print_point(-self.off.1, row);
            println!();
        }
        println!("|                  |BAR|                  |OFF");
        for row in (0..5).rev() {
            print!("|");
            for point in (0..=11).rev() {
                if point == 5 {
                    print!("|");
                    Self::print_point(self.bar.0, row);
                    print!("|");
                }
                Self::print_point(self.board[point], row)
            }
            print!("|");
            Self::print_point(self.off.0, row);
            println!();
        }
        println!("+12-11-10--9--8--7-------6--5--4--3--2--1-+");
    }

    fn print_point(value: i32, row: i32) {
        match (value, row) {
            (val, 4) if val.abs() > 5 => print!(" {} ", val.abs()),
            (val, _) if val > row => print!(" X "),
            (val, _) if val < -row => print!(" O "),
            _ => print!("   "),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn start_id() {
        let game = super::State::new();
        let id = game.position_id();
        assert_eq!(id, "4HPwATDgc/ABMA");
    }

    #[test]
    fn matching_ids() {
        let pids = [
            "4HPwATDgc/ABMA", // starting position
            "jGfkASjg8wcBMA", // random position
            "zGbiIQgxH/AAWA", // X bar
            "zGbiIYCYD3gALA", // O off
        ];
        for pid in pids {
            let game = super::State::from_id(pid.to_string());
            assert_eq!(pid, game.position_id());
        }
    }
}
