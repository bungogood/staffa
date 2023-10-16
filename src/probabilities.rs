use std::fmt;

use bkgm::GameResult;

/// Sum of all six fields will always be 1.0
#[derive(PartialEq)]
pub struct Probabilities {
    pub win_normal: f32,
    pub win_gammon: f32,
    pub win_bg: f32,
    pub lose_normal: f32,
    pub lose_gammon: f32,
    pub lose_bg: f32,
}

impl fmt::Debug for Probabilities {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Probabilities: wn {:.2}%; wg {:.2}%; wb {:.2}%; ln {:.2}%; lg {:.2}%; lb {:.2}%",
            100.0 * self.win_normal,
            100.0 * self.win_gammon,
            100.0 * self.win_bg,
            100.0 * self.lose_normal,
            100.0 * self.lose_gammon,
            100.0 * self.lose_bg
        )
    }
}

impl Probabilities {
    /// Typically used from rollouts.
    /// The index within the array has to correspond to the discriminant of the `Probabilities` enum.
    /// Input integer values will be normalized so that the sum in the return value is 1.0
    pub fn new(results: &[u32; 6]) -> Self {
        let sum = results.iter().sum::<u32>() as f32;
        Probabilities {
            win_normal: results[GameResult::WinNormal as usize] as f32 / sum,
            win_gammon: results[GameResult::WinGammon as usize] as f32 / sum,
            win_bg: results[GameResult::WinBackgammon as usize] as f32 / sum,
            lose_normal: results[GameResult::LoseNormal as usize] as f32 / sum,
            lose_gammon: results[GameResult::LoseGammon as usize] as f32 / sum,
            lose_bg: results[GameResult::LoseBackgammon as usize] as f32 / sum,
        }
    }

    pub fn from_result(results: &GameResult) -> Self {
        match results {
            GameResult::WinNormal => Self {
                win_normal: 1.0,
                win_gammon: 0.0,
                win_bg: 0.0,
                lose_normal: 0.0,
                lose_gammon: 0.0,
                lose_bg: 0.0,
            },
            GameResult::WinGammon => Self {
                win_normal: 0.0,
                win_gammon: 1.0,
                win_bg: 0.0,
                lose_normal: 0.0,
                lose_gammon: 0.0,
                lose_bg: 0.0,
            },
            GameResult::WinBackgammon => Self {
                win_normal: 0.0,
                win_gammon: 0.0,
                win_bg: 1.0,
                lose_normal: 0.0,
                lose_gammon: 0.0,
                lose_bg: 0.0,
            },
            GameResult::LoseNormal => Self {
                win_normal: 0.0,
                win_gammon: 0.0,
                win_bg: 1.0,
                lose_normal: 1.0,
                lose_gammon: 0.0,
                lose_bg: 0.0,
            },
            GameResult::LoseGammon => Self {
                win_normal: 0.0,
                win_gammon: 0.0,
                win_bg: 0.0,
                lose_normal: 0.0,
                lose_gammon: 1.0,
                lose_bg: 0.0,
            },
            GameResult::LoseBackgammon => Self {
                win_normal: 0.0,
                win_gammon: 0.0,
                win_bg: 0.0,
                lose_normal: 0.0,
                lose_gammon: 0.0,
                lose_bg: 1.0,
            },
        }
    }

    pub fn normalized(&self) -> Self {
        let sum = self.to_vec().iter().sum::<f32>();
        Probabilities {
            win_normal: self.win_normal / sum,
            win_gammon: self.win_gammon / sum,
            win_bg: self.win_bg / sum,
            lose_normal: self.lose_normal / sum,
            lose_gammon: self.lose_gammon / sum,
            lose_bg: self.lose_bg / sum,
        }
    }

    pub fn flip(&self) -> Self {
        Self {
            win_normal: self.lose_normal,
            win_gammon: self.lose_gammon,
            win_bg: self.lose_bg,
            lose_normal: self.win_normal,
            lose_gammon: self.win_gammon,
            lose_bg: self.win_bg,
        }
    }

    /// Cubeless equity
    pub fn equity(&self) -> f32 {
        self.win_normal - self.lose_normal
            + 2.0 * (self.win_gammon - self.lose_gammon)
            + 3.0 * (self.win_bg - self.lose_bg)
    }

    pub fn to_vec(&self) -> Vec<f32> {
        Vec::from(self.to_slice())
    }

    pub fn to_slice(&self) -> [f32; 6] {
        [
            self.win_normal,
            self.win_gammon,
            self.win_bg,
            self.lose_normal,
            self.lose_gammon,
            self.lose_bg,
        ]
    }

    pub fn to_gnu(&self) -> [f32; 5] {
        let win_gammon = self.win_gammon + self.win_bg;
        let lose_gammon = self.lose_gammon + self.lose_bg;
        [
            self.win_normal + win_gammon,
            win_gammon,
            self.win_bg,
            lose_gammon,
            self.lose_bg,
        ]
    }
}

impl From<&ResultCounter> for Probabilities {
    /// Typically used from rollouts.
    fn from(value: &ResultCounter) -> Self {
        let sum = value.sum() as f32;
        Probabilities {
            win_normal: value.num_of(GameResult::WinNormal) as f32 / sum,
            win_gammon: value.num_of(GameResult::WinGammon) as f32 / sum,
            win_bg: value.num_of(GameResult::WinBackgammon) as f32 / sum,
            lose_normal: value.num_of(GameResult::LoseNormal) as f32 / sum,
            lose_gammon: value.num_of(GameResult::LoseGammon) as f32 / sum,
            lose_bg: value.num_of(GameResult::LoseBackgammon) as f32 / sum,
        }
    }
}

impl From<&[f32; 6]> for Probabilities {
    /// Typically used from rollouts.
    fn from(value: &[f32; 6]) -> Self {
        let sum = value.iter().sum::<f32>();
        Probabilities {
            win_normal: value[0] / sum,
            win_gammon: value[1] / sum,
            win_bg: value[2] / sum,
            lose_normal: value[3] / sum,
            lose_gammon: value[4] / sum,
            lose_bg: value[5] / sum,
        }
    }
}

impl From<&[f32; 5]> for Probabilities {
    /// Typically used from rollouts.
    fn from(value: &[f32; 5]) -> Self {
        let win_bg = value[2];
        let lose_bg = value[4];
        let win_gammon = value[1] - win_bg;
        let lose_gammon = value[3] - lose_bg;
        let win_normal = value[0] - value[1];
        let lose_normal = 1.0 - value[0] - value[3];

        Probabilities {
            win_normal,
            win_gammon,
            win_bg,
            lose_normal,
            lose_gammon,
            lose_bg,
        }
    }
}

#[derive(Default)]
pub struct ResultCounter {
    results: [u32; 6],
}

impl ResultCounter {
    /// Convenience method, mainly for tests
    pub fn new(
        win_normal: u32,
        win_gammon: u32,
        win_bg: u32,
        lose_normal: u32,
        lose_gammon: u32,
        lose_bg: u32,
    ) -> Self {
        let results = [
            win_normal,
            win_gammon,
            win_bg,
            lose_normal,
            lose_gammon,
            lose_bg,
        ];
        Self { results }
    }
    pub fn add(&mut self, result: GameResult) {
        self.results[result as usize] += 1;
    }

    pub fn add_results(&mut self, result: GameResult, amount: u32) {
        self.results[result as usize] += amount;
    }

    pub fn sum(&self) -> u32 {
        self.results.iter().sum::<u32>()
    }

    pub fn num_of(&self, result: GameResult) -> u32 {
        // This works because the enum has associated integer values (discriminant), starting with zero.
        self.results[result as usize]
    }

    pub fn combine(self, counter: &ResultCounter) -> Self {
        let mut results = self.results;
        for (self_value, counter_value) in results.iter_mut().zip(counter.results) {
            *self_value += counter_value;
        }
        Self { results }
    }
}

#[cfg(test)]
mod probabilities_tests {
    use crate::probabilities::Probabilities;

    #[test]
    fn new() {
        // sum of `results is 32, a power of 2. Makes fractions easier to handle.
        let results = [0_u32, 1, 3, 4, 8, 16];
        let probabilities = Probabilities::new(&results);
        assert_eq!(probabilities.win_normal, 0.0);
        assert_eq!(probabilities.win_gammon, 0.03125);
        assert_eq!(probabilities.win_bg, 0.09375);
        assert_eq!(probabilities.lose_normal, 0.125);
        assert_eq!(probabilities.lose_gammon, 0.25);
        assert_eq!(probabilities.lose_bg, 0.5);
    }

    #[test]
    fn equity_win_normal() {
        let probabilities = Probabilities {
            win_normal: 1.0,
            win_gammon: 0.0,
            win_bg: 0.0,
            lose_normal: 0.0,
            lose_gammon: 0.0,
            lose_bg: 0.0,
        };
        assert_eq!(probabilities.equity(), 1.0);
    }

    #[test]
    fn equity_win_gammon() {
        let probabilities = Probabilities {
            win_normal: 0.0,
            win_gammon: 1.0,
            win_bg: 0.0,
            lose_normal: 0.0,
            lose_gammon: 0.0,
            lose_bg: 0.0,
        };
        assert_eq!(probabilities.equity(), 2.0);
    }

    #[test]
    fn equity_win_bg() {
        let probabilities = Probabilities {
            win_normal: 0.0,
            win_gammon: 0.0,
            win_bg: 1.0,
            lose_normal: 0.0,
            lose_gammon: 0.0,
            lose_bg: 0.0,
        };
        assert_eq!(probabilities.equity(), 3.0);
    }

    #[test]
    fn equity_lose_normal() {
        let probabilities = Probabilities {
            win_normal: 0.0,
            win_gammon: 0.0,
            win_bg: 0.0,
            lose_normal: 1.0,
            lose_gammon: 0.0,
            lose_bg: 0.0,
        };
        assert_eq!(probabilities.equity(), -1.0);
    }

    #[test]
    fn equity_lose_gammon() {
        let probabilities = Probabilities {
            win_normal: 0.0,
            win_gammon: 0.0,
            win_bg: 0.0,
            lose_normal: 0.0,
            lose_gammon: 1.0,
            lose_bg: 0.0,
        };
        assert_eq!(probabilities.equity(), -2.0);
    }

    #[test]
    fn equity_lose_bg() {
        let probabilities = Probabilities {
            win_normal: 0.0,
            win_gammon: 0.0,
            win_bg: 0.0,
            lose_normal: 0.0,
            lose_gammon: 0.0,
            lose_bg: 1.0,
        };
        assert_eq!(probabilities.equity(), -3.0);
    }

    #[test]
    fn equity_balanced() {
        let probabilities = Probabilities {
            win_normal: 0.3,
            win_gammon: 0.1,
            win_bg: 0.1,
            lose_normal: 0.3,
            lose_gammon: 0.1,
            lose_bg: 0.1,
        };
        assert_eq!(probabilities.equity(), 0.0);
    }

    #[test]
    fn to_gnu() {
        let probabilities = Probabilities {
            win_normal: 0.3,
            win_gammon: 0.1,
            win_bg: 0.1,
            lose_normal: 0.3,
            lose_gammon: 0.1,
            lose_bg: 0.1,
        };
        let gv = probabilities.to_gnu();
        assert_eq!(probabilities, Probabilities::from(&gv));
    }
}
