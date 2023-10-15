use bkgm::Dice;

pub trait DiceGen {
    /// Returns dice
    fn roll(&mut self) -> Dice;
}

pub struct FastrandDice {
    generator: fastrand::Rng,
}

impl DiceGen for FastrandDice {
    /// Returns random dice
    fn roll(&mut self) -> Dice {
        let random = self.generator.usize(0..36);
        let die1 = random / 6 + 1;
        let die2 = random % 6 + 1;
        Dice::new(die1, die2)
    }
}

impl FastrandDice {
    #[allow(dead_code)]
    pub fn new() -> FastrandDice {
        FastrandDice {
            generator: fastrand::Rng::new(),
        }
    }

    #[allow(dead_code)]
    pub fn with_seed(seed: u64) -> FastrandDice {
        FastrandDice {
            generator: fastrand::Rng::with_seed(seed),
        }
    }
}

#[cfg(test)]
/// Use this for unit tests where you want to control the dice.
pub(crate) struct DiceGenMock {
    dice: Vec<Dice>,
    no_calls: usize, // how often `roll` has been called.
}

#[cfg(test)]
impl DiceGen for DiceGenMock {
    fn roll(&mut self) -> Dice {
        let dice = self.dice[self.no_calls];
        self.no_calls += 1;
        dice
    }
}

#[cfg(test)]
impl DiceGenMock {
    pub(crate) fn new(dice: &[Dice]) -> DiceGenMock {
        DiceGenMock {
            dice: dice.to_vec(),
            no_calls: 0,
        }
    }

    pub(crate) fn assert_all_dice_were_used(&self) {
        assert_eq!(
            self.dice.len(),
            self.no_calls,
            "Not all dice of the mock have been used"
        );
    }
}

#[cfg(test)]
mod fastrand_dice_tests {
    use crate::dice::{Dice, DiceGen, FastrandDice};
    use std::cmp::Ordering;

    #[test]
    fn all_numbers_are_occurring() {
        // Given
        let mut gen = FastrandDice::with_seed(123);
        let mut count = [[0_u32; 6]; 6];
        // When
        for _ in 0..360_000 {
            let dice = gen.roll();
            match dice {
                Dice::Double(die) => {
                    count[die - 1][die - 1] += 1;
                }
                Dice::Regular(dice) => {
                    count[dice.big - 1][dice.small - 1] += 1;
                }
            }
        }

        // Check regular rolls
        for i in 0..6 {
            for j in 0..6 {
                let count = count[i][j];
                match i.cmp(&j) {
                    Ordering::Less => {
                        // This makes sure that dice.big is not smaller than dice.small
                        assert_eq!(count, 0);
                    }
                    Ordering::Equal => {
                        assert!(count > 9_000 && count < 11_000);
                    }
                    Ordering::Greater => {
                        assert!(count > 18_000 && count < 22_000);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod dice_gen_mock_tests {
    use crate::dice::{Dice, DiceGen, DiceGenMock};

    #[test]
    // #[should_panic]
    fn roll_returns_given_dice() {
        let mut dice_gen = DiceGenMock::new(&[Dice::new(3, 2), Dice::new(1, 6)]);
        assert_eq!(dice_gen.roll(), Dice::new(3, 2));
        assert_eq!(dice_gen.roll(), Dice::new(1, 6));
    }

    #[test]
    // #[should_panic]
    #[should_panic(expected = "Not all dice of the mock have been used")]
    fn assert_all_dice_were_used() {
        let mut dice_gen = DiceGenMock::new(&[Dice::new(3, 2), Dice::new(1, 6)]);
        dice_gen.roll();
        dice_gen.assert_all_dice_were_used();
    }

    #[test]
    #[should_panic(expected = "index out of bounds: the len is 1 but the index is 1")]
    fn panics_when_roll_is_called_too_often() {
        let mut dice_gen = DiceGenMock::new(&[Dice::new(3, 2)]);
        dice_gen.roll();
        dice_gen.roll();
    }
}
