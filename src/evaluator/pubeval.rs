use std::marker::PhantomData;

use bkgm::State;

use super::Evaluator;

/// https://bkgm.com/rgb/rgb.cgi?view+610

// Backgammon move-selection evaluation function
// for benchmark comparisons.  Computes a linear
// evaluation function:  Score = W * X, where X is
// an input vector encoding the board state (using
// a raw encoding of the number of men at each location),
// and W is a weight vector.  Separate weight vectors
// are used for racing positions and contact positions.
// Makes lots of obvious mistakes, but provides a
// decent level of play for benchmarking purposes.
// Provided as a public service to the backgammon
// programming community by Gerry Tesauro, IBM Research.
// (e-mail: tesauro@watson.ibm.com)
//
// The following inputs are needed for this routine:
//
// race   is an integer variable which should be set
// based on the INITIAL position BEFORE the move.
// Set race=1 if the position is a race (i.e. no contact)
// and 0 if the position is a contact position.
//
// pos[]  is an integer array of dimension 28 which
// should represent a legal final board state after
// the move. Elements 1-24 correspond to board locations
// 1-24 from computer's point of view, i.e. computer's
// men move in the negative direction from 24 to 1, and
// opponent's men move in the positive direction from
// 1 to 24. Computer's men are represented by positive
// integers, and opponent's men are represented by negative
// integers. Element 25 represents computer's men on the
// bar (positive integer), and element 0 represents opponent's
// men on the bar (negative integer). Element 26 represents
// computer's men off the board (positive integer), and
// element 27 represents opponent's men off the board
// (negative integer).
//
// Also, be sure to call rdwts() at the start of your
// program to read in the weight values. Happy hacking]

pub struct PubEval<G: State> {
    phantom: PhantomData<G>,
}

impl<G: State> Evaluator<G> for PubEval<G> {
    fn eval(&self, pos: &G) -> f32 {
        0.0
    }

    fn best_position(&self, pos: &G, dice: &bkgm::Dice) -> G {
        // let race = pos.is_race();
        let race = false;
        let positions = pos.possible_positions(dice);
        *positions
            .iter()
            .map(|pos| (pos, self.eval(pos)))
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .unwrap()
            .0
    }
}

impl<G: State> PubEval<G> {
    pub fn new() -> Self {
        Self {
            phantom: PhantomData,
        }
    }

    fn best_position(position: &G) {}

    fn pubeval(position: &G, race: bool) -> f32 {
        if position.x_off() == 15 {
            return 9999999.9;
        }

        let inputs = Self::to_inputs(position);

        let mut score = 0.0;
        if race {
            for i in 0..122 {
                score += RACE_WEIGHTS[i] * inputs[i];
            }
        } else {
            for i in 0..122 {
                score += CONTACT_WEIGHTS[i] * inputs[i];
            }
        }
        score
    }

    fn to_inputs(position: &G) -> [f32; 122] {
        let mut inputs = [0.0; 122];

        for point in 1..=24 {
            let jmp = point - 1;
            let pips = position.pip(25 - point);
            if pips != 0 {
                if pips == -1 {
                    inputs[5 * jmp + 0] = 1.0;
                }
                if pips == 1 {
                    inputs[5 * jmp + 0] = 1.0;
                }
                if pips >= 2 {
                    inputs[5 * jmp + 0] = 1.0;
                }
                if pips == 3 {
                    inputs[5 * jmp + 0] = 1.0;
                }
                if pips >= 4 {
                    inputs[5 * jmp + 0] = ((pips - 3) as f32) / 2.0;
                }
            }
        }

        inputs[120] = -(position.o_bar() as f32) / 2.0;
        inputs[121] = (position.x_off() as f32) / 15.0;
        inputs
    }
}

/// --- WT.race ---
const RACE_WEIGHTS: [f32; 122] = [
    0.00000, -0.17160, 0.27010, 0.29906, -0.08471, 0.00000, -1.40375, -1.05121, 0.07217, -0.01351,
    0.00000, -1.29506, -2.16183, 0.13246, -1.03508, 0.00000, -2.29847, -2.34631, 0.17253, 0.08302,
    0.00000, -1.27266, -2.87401, -0.07456, -0.34240, 0.00000, -1.34640, -2.46556, -0.13022,
    -0.01591, 0.00000, 0.27448, 0.60015, 0.48302, 0.25236, 0.00000, 0.39521, 0.68178, 0.05281,
    0.09266, 0.00000, 0.24855, -0.06844, -0.37646, 0.05685, 0.00000, 0.17405, 0.00430, 0.74427,
    0.00576, 0.00000, 0.12392, 0.31202, -0.91035, -0.16270, 0.00000, 0.01418, -0.10839, -0.02781,
    -0.88035, 0.00000, 1.07274, 2.00366, 1.16242, 0.22520, 0.00000, 0.85631, 1.06349, 1.49549,
    0.18966, 0.00000, 0.37183, -0.50352, -0.14818, 0.12039, 0.00000, 0.13681, 0.13978, 1.11245,
    -0.12707, 0.00000, -0.22082, 0.20178, -0.06285, -0.52728, 0.00000, -0.13597, -0.19412,
    -0.09308, -1.26062, 0.00000, 3.05454, 5.16874, 1.50680, 5.35000, 0.00000, 2.19605, 3.85390,
    0.88296, 2.30052, 0.00000, 0.92321, 1.08744, -0.11696, -0.78560, 0.00000, -0.09795, -0.83050,
    -1.09167, -4.94251, 0.00000, -1.00316, -3.66465, -2.56906, -9.67677, 0.00000, -2.77982,
    -7.26713, -3.40177, -12.32252, 0.00000, 3.42040,
];

/// --- WT.cntc ---
const CONTACT_WEIGHTS: [f32; 122] = [
    0.25696, -0.66937, -1.66135, -2.02487, -2.53398, -0.16092, -1.11725, -1.06654, -0.92830,
    -1.99558, -1.10388, -0.80802, 0.09856, -0.62086, -1.27999, -0.59220, -0.73667, 0.89032,
    -0.38933, -1.59847, -1.50197, -0.60966, 1.56166, -0.47389, -1.80390, -0.83425, -0.97741,
    -1.41371, 0.24500, 0.10970, -1.36476, -1.05572, 1.15420, 0.11069, -0.38319, -0.74816, -0.59244,
    0.81116, -0.39511, 0.11424, -0.73169, -0.56074, 1.09792, 0.15977, 0.13786, -1.18435, -0.43363,
    1.06169, -0.21329, 0.04798, -0.94373, -0.22982, 1.22737, -0.13099, -0.06295, -0.75882,
    -0.13658, 1.78389, 0.30416, 0.36797, -0.69851, 0.13003, 1.23070, 0.40868, -0.21081, -0.64073,
    0.31061, 1.59554, 0.65718, 0.25429, -0.80789, 0.08240, 1.78964, 0.54304, 0.41174, -1.06161,
    0.07851, 2.01451, 0.49786, 0.91936, -0.90750, 0.05941, 1.83120, 0.58722, 1.28777, -0.83711,
    -0.33248, 2.64983, 0.52698, 0.82132, -0.58897, -1.18223, 3.35809, 0.62017, 0.57353, -0.07276,
    -0.36214, 4.37655, 0.45481, 0.21746, 0.10504, -0.61977, 3.54001, 0.04612, -0.18108, 0.63211,
    -0.87046, 2.47673, -0.48016, -1.27157, 0.86505, -1.11342, 1.24612, -0.82385, -2.77082, 1.23606,
    -1.59529, 0.10438, -1.30206, -4.11520, 5.62596, -2.75800,
];
