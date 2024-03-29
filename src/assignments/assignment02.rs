//! Assignment 2: Mastering common programming concepts (1/2).
//!
//! The primary goal of this assignment is to re-learn the common programming concepts in Rust, especially those in the Rust Book chapters 3 and 5.
//! Please make sure you're comfortable with the concepts to proceed on to the next assignments.
//!
//! You should fill out the `todo!()` placeholders in such a way that `/scripts/grade-02.sh` works fine.
//! See `assignment02_grade.rs` and `/scripts/grade-02.sh` for the test script.

use std::cmp;
use std::ops::{Add, Mul, Sub}; //"Add" Add& Div  Div,, Add  was deleted because "Unused Import" Error came up //same as above , result

const FAHRENHEIT_OFFSET: f64 = 32.0;
const FAHRENHEIT_SCALE: f64 = 5.0 / 9.0;

/// Converts Farenheit to Celcius temperature degree.
pub(crate) fn fahrenheit_to_celsius(degree: f64) -> f64 {
    degree.sub(FAHRENHEIT_OFFSET).mul(FAHRENHEIT_SCALE)
    //todo
}

/// Capitalizes English alphabets (leaving the other characters intact).
pub(crate) fn capitalize(input: String) -> String {
    input.to_ascii_uppercase()
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
        i += 1;
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
        i -= 1;
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
    } else if n == 1 {
        v.push(1);
        return v;
    }
    let mut v_n_0;
    let mut v_n_1 = vec![1, 1];
    let mut k: usize = 1;
    let mut i: usize;
    let n: usize = n.try_into().unwrap();
    while k <= n {
        v_n_0 = v_n_1;
        v_n_1 = vec![1];
        i = 1;
        while i < k {
            v_n_1.push(v_n_0[i - 1] + v_n_0[i]); //removed & &
                                                 //println!("v_n_1 push: {}", &v_n_0[i - 1] + &v_n_0[i]);
            i += 1;
            //println!(":i {}", i)
        }
        v_n_1.push(1);
        k += 1;
        //println!("k: {}", k);
        //println!("n: {}", n);
    }
    v_n_1

    //    while k <= n {
    //        v.push(chooses_calculation_recursive(n, k)); //todo: insert formula for calculating
    //println!("v.push:{}", chooses_calculation(n, k));
    //        k = k + 1;
    //println!("k: {}", k);
    //    }

    //todo!()
}

pub(crate) fn chooses_calculation_multiplicative(n: u64, k: u64) -> u64 {
    let mut res = 1;
    for i in 1..=k + 1 {
        res = res * (n - (k - i)) / i;
    }
    res
}

pub(crate) fn chooses_calculation_recursive(n: u64, k: u64) -> u64 {
    //println!("chooses_calculation: n: {}, k: {}", n, k);
    //println!("division: {}", faculty(k).mul(faculty(n.sub(k))));
    //faculty(n).wrapping_div(faculty(k).wrapping_mul(faculty(n.wrapping_sub(k))))
    //    if k == 0 {
    //       return 1;
    //    } else if k == n {
    //        return 1;
    //    }
    //    return chooses_calculation(n - 1, k) + chooses_calculation(n - 1, k - 1);
    //let mut i = 1;
    //let mut result_chooses_calculation = 1;
    //let mut multiplicator_chooses;
    //while i <= k {
    //  multiplicator_chooses = n.sub(k.sub(i));
    //result_chooses_calculation = result_chooses_calculation.mul(multiplicator_chooses);
    //result_chooses_calculation = result_chooses_calculation.div(i);
    //   i = i + 1;
    // }
    //result_chooses_calculation
    //let mut k_choose = k;
    //  if k > n.sub(k) {
    //      k_choose = n.sub(k);
    //  }
    //  let mut result_n_chooses_k: u64 = 1;
    // let mut i = 0;
    //// while i < k_choose {
    //     result_n_chooses_k = result_n_chooses_k.mul(n.sub(i));
    //    result_n_chooses_k = result_n_chooses_k.div(i + 1);
    //    i = i + 1;
    // }
    // result_n_chooses_k

    //I know this is copied but i can't solve it otherwise
    //
    // if k == 0 {
    //     return 1;
    // } else if k == n {
    //      return 1;
    //  }

    //  let mut res = 1;
    // println!("k: {k}");
    // println!("n-k: {}", n.sub(k));
    // let k = cmp::min(k, n.sub(k));
    // println!("k': {k}");
    // println!("n:{n}");
    // let mut i = k;
    //
    // while i >= 1 {
    //     res = (res * (n - i)) / (i + 1);
    //      i = i-1;
    //  }
    //  res

    //Recursive approach:
    if k == 0 || n == k {
        return 1;
    }
    chooses_calculation_recursive(n.sub(1), k)
        .add(chooses_calculation_recursive(n.sub(1), k.sub(1)))
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
        k += 1;
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
        i += 1;
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
    //Tested and verified that it works
    fn mul(self, rhs: Mat2) -> Self::Output {
        let mut res = Mat2::new();
        res.a = self.a * rhs.a + self.c * rhs.b;
        res.b = self.b * rhs.a + self.d * rhs.b;
        res.c = self.a * rhs.c + self.c * rhs.d;
        res.d = self.b * rhs.c + self.d * rhs.d;
        res
        //todo!()
    }
}
#[test]
fn test_mul_mat2_x_mat2() {
    let mut lhs: Mat2 = Mat2::new();
    lhs.a = 199;
    lhs.b = 290;
    lhs.c = 435;
    lhs.d = 634;
    let mut rhs: Mat2 = Mat2::new();
    rhs.a = 1;
    rhs.b = 2;
    rhs.c = 3;
    rhs.d = 4;

    println!("a:{}", lhs.mul(rhs).a);
    println!("b:{}", lhs.mul(rhs).b);
    println!("c:{}", lhs.mul(rhs).c);
    println!("d:{}", lhs.mul(rhs).d);

    println!("{}", lhs.a,);
}

impl Mul<Vec2> for Mat2 {
    type Output = Vec2;
    fn mul(self, rhs: Vec2) -> Self::Output {
        let mut res = rhs;
        res.a = 0;
        res.b = 0;
        res.a = self.a * rhs.a + self.c * rhs.b;
        res.b = self.b * rhs.a + self.d * rhs.b;
        res
        //todo!()
    }
}

#[test]
fn test_mul_mat2_x_vec2() {
    let mut lhs: Mat2 = Mat2::new();
    lhs.a = 2;
    lhs.b = 3;
    lhs.c = 1;
    lhs.d = 2;

    let rhs: Vec2 = Vec2 { a: (0), b: (0) };

    println!("{}", lhs.mul(rhs).a);
    println!("{}", lhs.mul(rhs).b);
}

impl Mat2 {
    /// Calculates the power of matrix.
    fn power(self, power: u64) -> Mat2 {
        let mut res = Mat2::new();
        if power == 0 {
            return res;
        }
        if power == 1 {
            return self;
        }

        let mut i = 1;
        res = self;
        while i < power {
            res = res.mul(self);
            i += 1;
        }
        res

        // res = self;
        // while i <= power {
        //    res = res.mul(res);
        //
        //    println!("________________");
        //    println!("{}  {}", res.a, res.c);
        //    println!("{}  {}", res.b, res.d);
        //     println!("i: {}", i);
        //     println!("________________");
        //     i = i + 1;
        // }
        //res
        //todo!()
    }
}
#[test]
fn test_power() {
    let mut lhs: Mat2 = Mat2::new();
    lhs.a = 1;
    lhs.b = 2;
    lhs.c = 4;
    lhs.d = 4;
    let n = 4;
    let res = lhs.power(n);
    println!("_RESULT_n:{}______________", n);
    println!("{}  {}", res.a, res.c);
    println!("{}  {}", res.b, res.d);
    println!("________________");
}
impl Vec2 {
    /// Gets the upper value of vector.
    fn get_upper(self) -> u64 {
        self.a
        //todo!()
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
pub fn fibonacci(n: u64) -> u64 {
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
