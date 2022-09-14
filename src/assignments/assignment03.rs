//! Assignment 3: Mastering common programming concepts (2/2).
//!
//! The primary goal of this assignment is to re-learn the common programming concepts in Rust, especially those in the Rust Book chapters 6, 7, 8, and 9.
//! Please make sure you're comfortable with the concepts to proceed on to the next assignments.
//!
//! You should fill out the `todo!()` placeholders in such a way that `/scripts/grade-03.sh` works fine.
//! See `assignment03_grade.rs` and `/scripts/grade-03.sh` for the test script.

use std::{
    collections::{HashMap, HashSet},
    ops::{Add, Div, Sub},
};

/// Day of week.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DayOfWeek {
    /// Sunday.
    Sun,
    /// Monday.
    Mon,
    /// Tuesday.
    Tue,
    /// Wednesday.
    Wed,
    /// Thursday.
    Thu,
    /// Friday.
    Fri,
    /// Saturday.
    Sat,
}

/// The next day of week.
///
/// `next_weekday(Thu)` is `Fri`; and `next_weekday(Fri)` is `Mon`.
pub fn next_weekday(day: DayOfWeek) -> DayOfWeek {
    match day {
        DayOfWeek::Mon => DayOfWeek::Tue,
        DayOfWeek::Tue => DayOfWeek::Wed,
        DayOfWeek::Wed => DayOfWeek::Thu,
        DayOfWeek::Thu => DayOfWeek::Fri,
        DayOfWeek::Fri => DayOfWeek::Mon,
        DayOfWeek::Sat => DayOfWeek::Mon,
        DayOfWeek::Sun => DayOfWeek::Mon,
    }
}

/// Custom option type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MyOption<T> {
    /// Some value of type `T`.
    MySome(T),
    /// No value.
    MyNone,
}

/// Maps the inner value if the given value is `MySome`; returns `MyNone` otherwise.
pub fn my_map<T, U, F: FnOnce(T) -> U>(v: MyOption<T>, f: F) -> MyOption<U> {
    todo!()
}

/// Maps the inner value if the given value is `MySome`, but with a different type of function; returns `MyNone` otherwise.
pub fn my_and_then<T, U, F: FnOnce(T) -> MyOption<U>>(v: MyOption<T>, f: F) -> MyOption<U> {
    todo!()
}

/// Given a list of integers, returns its median (when sorted, the value in the middle position).
///
/// For a data set `x` of `n` elements, the median can be defined as follows:
///
/// - If `n` is odd, the median is `(n+1)/2`-th smallest element of `x`.
/// - If `n` is even, the median is `(n/2)+1`-th smallest element of `x`.
///
/// For example, the following list of seven numbers,
///
/// ```
/// vec![1, 3, 3, 6, 7, 8, 9]
/// ```
///
/// has the median of 6, which is the fourth value. And for this data set of eight numbers,
///
/// ```
/// vec![1, 2, 3, 4, 5, 6, 8, 9]
/// ```
///
/// it has the median of 5, which is the fifth value.
///
/// Returns `None` if the list is empty.
pub fn median(mut values: Vec<isize>) -> Option<isize> {
    if values.len() == 0 {
        return None;
    }
    values.sort();
    if is_even(&values) {
        return Some(calc_even(&values));
    }
    Some(calc_odd(&values))
}
///Calculate Median for an Even Vector
fn calc_even(lhs: &Vec<isize>) -> isize {
    lhs[lhs.len().sub(1).div(2).add(1)]
}
///Calculate Median for an Odd Vector
fn calc_odd(lhs: &Vec<isize>) -> isize {
    lhs[lhs.len().sub(1).add(1).div(2)]
}
///Check if given Vector is Even
fn is_even(lhs: &Vec<isize>) -> bool {
    if lhs.len() % 2 == 0 {
        return true;
    }
    false
}

/// Given a list of integers, returns its smallest mode (the value that occurs most often; a hash map will be helpful here).
///
/// Returns `None` if the list is empty.
pub fn mode(values: Vec<isize>) -> Option<isize> {
    if values.len() == 0 {
        return None;
    }
    let mut res: HashMap<isize, u128> = HashMap::new();

    for i in values {
        if res.contains_key(&i) {
            *res.get_mut(&i).unwrap() += 1;
        } else {
            res.insert(i, 1);
        }
    }

    return get_hash_map_highest_value(res);
}
fn get_hash_map_highest_value(lhs: HashMap<isize, u128>) -> Option<isize> {
    let mut integ: Option<isize> = None;
    let mut count: u128 = 0;
    for (k, v) in lhs {
        if v > count {
            integ = Some(k);
            count = v;
        }
    }
    return integ;
}

/// Converts the given string to Pig Latin. Use the rules below to translate normal English into Pig Latin.
///
/// 1. If a word starts with a consonant and a vowel, move the first letter of the word at the end of the word and add "ay".
///
/// Example: "happy" -> "appyh" + "ay" -> "appyhay"
///
/// 2. If a word starts with multiple consonants, move them to the end of the word and add "ay".
///
/// Example: "string" -> "ingstr" + "ay" -> "ingstray"
///
/// 3. If a word starts with a vowel, add the word "hay" at the end of the word.
///
/// Example: "explain" -> "explain" + "hay" -> "explainhay"
///
/// Keep in mind the details about UTF-8 encoding!
///
/// You may assume the string only contains lowercase alphabets, and it contains at least one vowel.
pub fn piglatin(input: String) -> String {
    todo!()
}

/// Converts HR commands to the organization table.
///
/// If the commands are as follows:
///
/// ```
/// vec!["Add Amir to Engineering", "Add Sally to Sales", "Remove Jeehoon from Sales", "Move Amir from Engineering to Sales"]
/// ```
///
/// The return value should be:
///
/// ```
/// ["Sales" -> ["Amir", "Sally"]]
/// ```
///
/// - The result is a map from department to the list of its employees.
/// - An empty department should not appear in the result.
/// - There are three commands: "Add <person> to <department>", "Remove <person> from <department>", and "Move <person> from <department> to <department>".
/// - If a command is not executable, then it's ignored.
/// - There is no space in the name of the person and department.
///
/// See the test function for more details.
pub fn organize(commands: Vec<String>) -> HashMap<String, HashSet<String>> {
    todo!()
}
