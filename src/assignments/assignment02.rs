//! Assignment 2: Mastering common programming concepts (1/2).
//!
//! The primary goal of this assignment is to re-learn the common programming concepts in Rust, especially those in the Rust Book chapters 3 and 5.
//! Please make sure you're comfortable with the concepts to proceed on to the next assignments.
//!
//! You should fill out the `todo!()` placeholders in such a way that `/scripts/grade-02.sh` works fine.
//! See `assignment02_grade.rs` and `/scripts/grade-02.sh` for the test script.

use std::cmp;
use std::ops::{Mul, Sub}; //"Add" & Div  was deleted because "Unused Import" Error came up //same as above , result
const FAHRENHEIT_OFFSET: f64 = 32.0;
const FAHRENHEIT_SCALE: f64 = 5.0 / 9.0;

/// Converts Farenheit to Celcius temperature degree.
pub(crate) fn fahrenheit_to_celsius(degree: f64) -> f64 {
    degree.sub(FAHRENHEIT_OFFSET).mul(FAHRENHEIT_SCALE)
    //todo
}

/// Capitalizes English alphabets (leaving the other characters intact).
pub(crate) fn capitalize(input: String) -> String {
    input.to_uppercase()
    //todo!()
}

/// Returns the sum of the given array. (We assume the absence of integer overflow.)
pub(crate) fn sum_array(input: &[u64]) -> u64 {
    input.iter().sum()
    //todo!()
}

/// Given a non-negative integer, say `n`, return the smallest integer of the form `3^m` that's greater than or equal to `n`.
///
/// For instance, up3(6) = 9, up3(9) = 9, up3(10) = 27. (We assume the absence of integer overflow.)
pub(crate) fn up3(n: u64) -> u64 {
    let mut i = 0;
    while 3_u64.wrapping_pow(i) < n {
        i = i + 1;
    }
    3_u64.pow(i)
    //todo!()
}

/// Returns the greatest common divisor (GCD) of two non-negative integers. (We assume the absence of integer overflow.)
pub(crate) fn gcd(lhs: u64, rhs: u64) -> u64 {
    if cmp::min(lhs, rhs) == 0 {
        return cmp::max(lhs, rhs);
    } else if lhs == rhs {
        return lhs;
    }
    let mut i = cmp::min(lhs, rhs);
    while i <= cmp::min(lhs, rhs) {
        if cmp::min(lhs, rhs) % i == 0 && cmp::max(lhs, rhs) % i == 0 {
            return i;
        }
        i = i - 1;
    }
    1
}

/// Returns the array of nC0, nC1, nC2, ..., nCn, where nCk = n! / (k! * (n-k)!). (We assume the absence of integer overflow.)
///
/// Consult <https://en.wikipedia.org/wiki/Pascal%27s_triangle> for computation of binomial coefficients without integer overflow.
pub(crate) fn chooses(n: u64) -> Vec<u64> {
    let mut v = vec![];
    v.push(1);
    if n == 0 {
        return v;
    }
    let mut k = 1;
    while k <= n {
        v.push(chooses_calculation(n, k)); //todo: insert formula for calculating
        println!("v.push:{}", chooses_calculation(n, k));
        k = k + 1;
        println!("k: {}", k);
    }
    v
    //todo!()
}

pub(crate) fn chooses_calculation(n: u64, k: u64) -> u64 {
    println!("chooses_calculation: n: {}, k: {}", n, k);
    println!("division: {}", faculty(k).mul(faculty(n.sub(k))));
    faculty(n).wrapping_div(faculty(k).wrapping_mul(faculty(n.wrapping_sub(k))))
}

pub(crate) fn faculty(n: u64) -> u64 {
    let mut k: u64 = 1;
    let mut output: u64 = 1;
    if n == 0 {
        return 1;
    }
    while k <= n {
        //println!("faculty: {}{}{}", n, k, output);
        output = output.wrapping_mul(k);
        k = k + 1;
    }
    output
}

/// Returns the "zip" of two vectors.
///
/// For instance, `zip(vec![1, 2, 3], vec![4, 5])` equals to `vec![(1, 4), (2, 5)]`.
/// Here, `3` is ignored becaus ze it doesn't have a partner.
pub(crate) fn zip(lhs: Vec<u64>, rhs: Vec<u64>) -> Vec<(u64, u64)> {
    let mut i = 0;
    let mut outcome_zip: Vec<(u64, u64)> = vec![];
    if cmp::min(lhs.len(), rhs.len()) == 0 {
        return outcome_zip;
    }
    while i < cmp::min(lhs.len(), rhs.len()) {
        outcome_zip.push((lhs[i], rhs[i]));
        i = i + 1;
    }
    outcome_zip
    //todo!()
}

/// 2x2 matrix of the following configuration:
///
/// a, b
/// c, d
#[derive(Debug, Clone, Copy)]
struct Mat2 {
    a: u64,
    b: u64,
    c: u64,
    d: u64,
}

/// 2x1 matrix of the following configuration:
///
/// a
/// b
#[derive(Debug, Clone, Copy)]
struct Vec2 {
    a: u64,
    b: u64,
}

impl Mat2 {
    /// Creates an identity matrix.
    fn new() -> Self {
        Self {
            a: 1,
            b: 0,
            c: 0,
            d: 1,
        }
    }
}

impl Mul<Mat2> for Mat2 {
    type Output = Mat2;

    fn mul(self, rhs: Mat2) -> Self::Output {
        //self.mul(rhs)
        todo!()
    }
}

impl Mul<Vec2> for Mat2 {
    type Output = Vec2;
    fn mul(self, rhs: Vec2) -> Self::Output {
        todo!()
    }
}

impl Mat2 {
    /// Calculates the power of matrix.
    fn power(self, power: u64) -> Mat2 {
        todo!()
    }
}

impl Vec2 {
    /// Gets the upper value of vector.
    fn get_upper(self) -> u64 {
        //self[self.len()]
        todo!()
    }
}

/// The matrix used for calculating Fibonacci numbers.
const FIBONACCI_MAT: Mat2 = Mat2 {
    a: 1,
    b: 1,
    c: 1,
    d: 0,
};

/// The vector used for calculating Fibonacci numbers.
const FIBONACCI_VEC: Vec2 = Vec2 { a: 1, b: 0 };

/// Calculates the Fibonacci number. (We assume the absence of integer overflow.)
///
/// Consult <https://web.media.mit.edu/~holbrow/post/calculating-fibonacci-numbers-with-matrices-and-linear-algebra/> for matrix computation of Fibonacci numbers.
pub(crate) fn fibonacci(n: u64) -> u64 {
    (FIBONACCI_MAT.power(n) * FIBONACCI_VEC).get_upper()
}

/// Writes down the lyrics of "twelve days of christmas".
///
/// Hint: Google the song title for lyrics and look at the test code for the expected result.
pub(crate) fn twelve_days_of_christmas_lyrics() -> String {
    LYRICS.to_string()
    //todo!()
}
const LYRICS: &str = r#"On the first day of Christmas, my true love sent to me
A partridge in a pear tree

On the second day of Christmas, my true love sent to me
Two turtledoves
And a partridge in a pear tree

On the third day of Christmas, my true love sent to me
Three French hens
Two turtledoves
And a partridge in a pear tree

On the fourth day of Christmas, my true love sent to me
Four calling birds
Three French hens
Two turtledoves
And a partridge in a pear tree

On the fifth day of Christmas, my true love sent to me
Five gold rings (five golden rings)
Four calling birds
Three French hens
Two turtledoves
And a partridge in a pear tree

On the sixth day of Christmas, my true love sent to me
Six geese a-laying
Five gold rings (five golden rings)
Four calling birds
Three French hens
Two turtledoves
And a partridge in a pear tree

On the seventh day of Christmas, my true love sent to me
Seven swans a-swimming
Six geese a-laying
Five gold rings (five golden rings)
Four calling birds
Three French hens
Two turtledoves
And a partridge in a pear tree

On the eighth day of Christmas, my true love sent to me
Eight maids a-milking
Seven swans a-swimming
Six geese a-laying
Five gold rings (five golden rings)
Four calling birds
Three French hens
Two turtledoves
And a partridge in a pear tree

On the ninth day of Christmas, my true love sent to me
Nine ladies dancing
Eight maids a-milking
Seven swans a-swimming
Six geese a-laying
Five gold rings (five golden rings)
Four calling birds
Three French hens
Two turtledoves
And a partridge in a pear tree

On the tenth day of Christmas, my true love sent to me
Ten lords a-leaping
Nine ladies dancing
Eight maids a-milking
Seven swans a-swimming
Six geese a-laying
Five gold rings (five golden rings)
Four calling birds
Three French hens
Two turtledoves
And a partridge in a pear tree

On the eleventh day of Christmas, my true love sent to me
I sent eleven pipers piping
Ten lords a-leaping
Nine ladies dancing
Eight maids a-milking
Seven swans a-swimming
Six geese a-laying
Five gold rings (five golden rings)
Four calling birds
Three French hens
Two turtledoves
And a partridge in a pear tree

On the twelfth day of Christmas, my true love sent to me
Twelve drummers drumming
Eleven pipers piping
Ten lords a-leaping
Nine ladies dancing
Eight maids a-milking
Seven swans a-swimming
Six geese a-laying
Five gold rings (five golden rings)
Four calling birds
Three French hens
Two turtledoves
And a partridge in a pear tree

And a partridge in a pear tree
"#;
