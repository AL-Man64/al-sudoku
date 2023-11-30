use js_sys::Array;
use wasm_bindgen::{prelude::*, throw_str};

/// The Sudoku object.
///
/// Represents a game of sudoku, that can be further manipulated and
/// solved.
#[wasm_bindgen]
#[derive(Clone)]
pub struct Sudoku {
    fields: [u8; 81],
}

/// Public functions of the sudoku object, exposed to javascript.
#[wasm_bindgen]
impl Sudoku {
    /// Constructor for the Sudoku object.
    ///
    /// Takes an Array of numbers of length 81. The values in the array
    /// are limited to values 0-9, with 0 representing a blank field in a
    /// sudoku game. The constructor will throw an error if any of these
    /// requirements is not met.
    ///
    /// The elements are ordered from left to right, and from top to
    /// bottom in terms of the sudoku field.
    #[wasm_bindgen(constructor)]
    pub fn constructor(input: Array) -> Sudoku {
        // Initialization of the sudoku variable that will be returned out
        // of the constructor
        let mut sudoku = Sudoku { fields: [0; 81] };

        // So in the previous iteration of this code, in a strike of my
        // own brilliance, I made the decision to spend at least an hour
        // trying to figure out how to convert a JsValue into an Array,
        // while trying to avoid any deprecated functions, and not once
        // have I thought about having the Array be a function parameter.

        // I don't know man... Sounds like a skill issue to me

        // Oh, by all means, let's childishly make fun of myself, because
        // that's all you can do....

        // Iterates over the input array, converting it into a Vec<u8>
        let input = input
            .into_iter()
            .map(|val| val.as_f64().unwrap_throw() as u8)
            .collect::<Vec<u8>>();

        if input.len() != 81 {
            throw_str("`input` must be an Array of length 81, but it's of different length");
        }

        for (i, field) in input.iter().enumerate() {
            if !(1..=9).contains(field) {
                throw_str("Illegal value for a field, must be between 1 and 9");
            }
            sudoku.fields[i] = *field;
        }

        sudoku
    }

    /// Returns the fields within the Sudoku object
    pub fn get_fields(&self) -> Vec<u8> {
        self.fields.to_vec()
    }

    /// Checks the validity of the sudoku game represented by this object,
    /// returns *true* if valid or *false* otherwise.
    pub fn check_validity(&self) -> bool {
        self.fields
            .iter()
            .enumerate()
            .all(|(index, _)| self.check_field_validity(index))
    }

    /// Solves the game represented by this object and returns the result
    /// as a new (solved) Sudoku object.
    pub fn solve(&self) -> Sudoku {
        let mut solution = self.clone();
        Sudoku::solve_recursive(&mut solution, 0);
        solution
    }
}

/// Functions of the Sudoku object not exposed to javascript.
impl Sudoku {
    /// Checks the validity of a single field in the sudoku game
    /// represented by this object, returns *true* if valid or *false*
    /// otherwise.
    pub fn check_field_validity(&self, index: usize) -> bool {
        if self.fields[index] == 0 {
            return true;
        }

        // Check row
        let row = index / 9;
        for column in 0..9 {
            if self.fields[9 * row + column] == self.fields[index] {
                if 9 * row + column == index {
                    continue;
                }
                return false;
            }
        }

        // Check column
        let column = index % 9;
        for row in 0..9 {
            if self.fields[9 * row + column] == self.fields[index] {
                if 9 * row + column == index {
                    continue;
                }
                return false;
            }
        }

        // Check group
        let group_row = (index / 9) / 3;
        let group_column = (index % 9) / 3;
        for row in 0..3 {
            for column in 0..3 {
                if self.fields[27 * group_row + 9 * row + 3 * group_column + column]
                    == self.fields[index]
                {
                    if 27 * group_row + 9 * row + 3 * group_column + column == index {
                        continue;
                    }
                    return false;
                }
            }
        }

        true
    }

    /// Solves the given Sudoku object at a given index.
    ///
    /// This is a recursive function and will call itself on the next
    /// field when a good solution is reached.
    pub fn solve_recursive(sudoku: &mut Sudoku, index: usize) -> bool {
        if index >= 81 {
            return true;
        }

        if sudoku.fields[index] != 0 {
            return Sudoku::solve_recursive(sudoku, index + 1);
        }

        for i in 1..=9 {
            sudoku.fields[index] = i;
            if sudoku.check_field_validity(index) {
                if Sudoku::solve_recursive(sudoku, index + 1) {
                    return true;
                }
            }
        }

        sudoku.fields[index] = 0;
        false
    }
}
