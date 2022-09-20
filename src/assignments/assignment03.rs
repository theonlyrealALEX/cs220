//! Assignment 3: Mastering common programming concepts (2/2).
//!
//! The primary goal of this assignment is to re-learn the common programming concepts in Rust, especially those in the Rust Book chapters 6, 7, 8, and 9.
//! Please make sure you're comfortable with the concepts to proceed on to the next assignments.
//!
//! You should fill out the `todo!()` placeholders in such a way that `/scripts/grade-03.sh` works fine.
//! See `assignment03_grade.rs` and `/scripts/grade-03.sh` for the test script.

use std::{
    collections::{HashMap, HashSet},
    //fs::read,
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
    match v {
        MyOption::MySome(v_some) => MyOption::MySome(f(v_some)),
        MyOption::MyNone => MyOption::MyNone,
    }
}

/// Maps the inner value if the given value is `MySome`, but with a different type of function; returns `MyNone` otherwise.
pub fn my_and_then<T, U, F: FnOnce(T) -> MyOption<U>>(v: MyOption<T>, f: F) -> MyOption<U> {
    match v {
        MyOption::MySome(v_some) => f(v_some),
        MyOption::MyNone => MyOption::MyNone,
    }
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
    if is_empty(values.len()) {
        return None;
    }
    values.sort();
    if is_even(&values) {
        return Some(calc_even(&values));
    }
    Some(calc_odd(&values))
}

fn is_empty(u: usize) -> bool {
    if u == 0 {
        return true;
    }
    false
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
    if is_empty(values.len()) {
        return None;
    }
    let mut map = HashMap::new();

    for i in values {
        let count = map.entry(i).or_insert(0);
        *count += 1;
    }
    //println!("{:?}", map);
    get_hash_map_highest_value(map)
}
///Returns Hashmap-Key with highest associated u128 value
fn get_hash_map_highest_value(lhs: HashMap<isize, u128>) -> Option<isize> {
    let mut integ: Option<isize> = None;
    let mut count: u128 = 0;

    for (k, v) in lhs {
        if v > count {
            integ = Some(k);
            count = v;
        } else if v == count && Some(k) < integ {
            integ = Some(k)
        }
    }
    integ
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
pub fn piglatin(mut input: String) -> String {
    let string_type: PigLatinType;
    if char_is_vowel(input.chars().next().unwrap()) {
        string_type = PigLatinType::Vowel;
    } else if !char_is_vowel(input.chars().next().unwrap())
        && !char_is_vowel(input.chars().nth(1).unwrap())
    {
        string_type = PigLatinType::ConsonantConsonant;
    } else {
        string_type = PigLatinType::ConsonantVowel;
    }

    match string_type {
        PigLatinType::ConsonantVowel => add_ay_to_end(move_first_to_end(input)),
        PigLatinType::ConsonantConsonant => {
            let mut j = 0;
            for i in input.chars() {
                if char_is_vowel(i) {
                    break;
                }
                j += 1;
            }
            let mut k = 0;
            while k < j {
                input = move_first_to_end(input);
                k += 1
            }
            add_ay_to_end(input)
        }
        PigLatinType::Vowel => add_hay_to_end(input),
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PigLatinType {
    ConsonantVowel,
    ConsonantConsonant,
    Vowel,
}

#[test]
fn test_move_first_to_end() {
    println!("{}", move_first_to_end("abcdefg".to_string()));
}

///Move First letter to the end of a String
fn move_first_to_end(input: String) -> String {
    let mut res = input;
    res.push(res.chars().next().unwrap());
    res[1..res.len()].to_string()
}

///Add "hay" to end of String
fn add_hay_to_end(input: String) -> String {
    input + "hay"
}

///Add "ay" to end of String
fn add_ay_to_end(input: String) -> String {
    input + "ay"
}

///checks if first char in a string is a lowercase vowel char (a,e,i,o,u)
fn char_is_vowel(input: char) -> bool {
    if input == 'a' {
        return true;
    }
    if input == 'e' {
        return true;
    }
    if input == 'i' {
        return true;
    }
    if input == 'o' {
        return true;
    }
    if input == 'u' {
        return true;
    }
    false
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
    let mut res: HashMap<String, HashSet<String>> = HashMap::new();

    for i in commands {
        let command = i.split_whitespace().next().unwrap_or("");
        let vec_input: Vec<&str> = i.split_whitespace().collect();

        if command == "Add" {
            println!("Add:");
            let person: String = vec_input[1].to_string();
            let department = vec_input[3].to_string();

            let tmp = add_person_to_department(person, department, &mut res);
            /*for (key, value) in &res {
                println!("KEY_ADD:{}", key);
            }*/
        } else if command == "Remove" {
            println!("Remove:");

            let person: String = vec_input[1].to_string();
            let department = vec_input[3].to_string();

            let tmp = remove_person_from_department(person, department, &mut res);
        } else if command == "Move" {
            let person: String = vec_input[1].to_string();
            let person_remove = person.clone();
            let add_to_department = vec_input[5].to_string();
            let remove_from_department = vec_input[3].to_string();
            println!(
                "person: {}, add_dept: {}, remove_dept: {}",
                &person, &add_to_department, &remove_from_department
            );

            if is_in_department(&person, &remove_from_department, &res) {
                //add move logic
                let tmp = add_person_to_department(person, add_to_department, &mut res);
                let tmp =
                    remove_person_from_department(person_remove, remove_from_department, &mut res);
            }
        }
    }
    res
}

fn is_in_department(
    person: &String,
    department: &String,
    res: &HashMap<String, HashSet<String>>,
) -> bool {
    match res.get(department) {
        Some(dpt) => dpt.contains(person),
        None => false,
    }
}

///remove person from department no matter if it exists or not && delete empty key's
fn remove_person_from_department(
    person: String,
    department: String,
    res: &mut HashMap<String, HashSet<String>>,
) -> &mut HashMap<String, HashSet<String>> {
    println!(
        "called remove_person_from_department with person: {}, department: {}",
        &person, &department
    );
    if res.contains_key(&department) {
        match res.get(&department) {
            Some(dpt) => {
                let mut tmp_person_hashset = dpt.clone();
                let tmp = tmp_person_hashset.remove(&person);
                let tmp = res.remove(&department);
                if tmp_person_hashset.is_empty() {
                    return res;
                }
                let dpt = department;
                let tmp = res.insert(dpt, tmp_person_hashset);
            }
            None => (),
        }
    }
    res
}

///add a person into the hashset for a department key; or_insert -> only inserts if person its not already present;
fn add_person_to_department(
    person: String,
    department: String,
    res: &mut HashMap<String, HashSet<String>>,
) -> &mut HashMap<String, HashSet<String>> {
    println!(
        "called add_person_to_department with person: {}, department: {}",
        &person, &department
    );
    /*
        let mut person_hs = HashSet::<String>::new();
        let tmp = person_hs.insert(person);
        let tmp = res.get_mut(&department).get_or_insert(&mut person_hs);
    */
    let person_tmp = person.clone();
    match res.get(&department) {
        Some(dpt) => {
            println!("res.get(dept) is SOME");
            let mut tmp_person_hashset = dpt.clone();
            let tmp = tmp_person_hashset.insert(person);
            println!("was something inserted into the hashSET: {tmp}");

            let dpt = department;
            let tmp = res.insert(dpt, tmp_person_hashset);

            //let tmp = res.entry(department).or_insert(tmp_person_hashset);

            /*println!(
                "was something inserted into the HashMAP: {:?}",
                tmp.get(&person_tmp)
            );*/
        }
        None => {
            println!("res.get(dept) is NONE");
            let mut tmp_person_hashset = HashSet::new();
            let tmp = tmp_person_hashset.insert(person);
            let tmp = res.entry(department).or_insert(tmp_person_hashset);
        }
    }

    res
}

///old organize
pub fn organize_old(commands: Vec<String>) -> HashMap<String, HashSet<String>> {
    let mut res: HashMap<String, HashSet<String>> = HashMap::new();
    for i in commands {
        let command = i.split_whitespace().next().unwrap_or("");
        let vec_input: Vec<&str> = i.split_whitespace().collect();
        if command == "Add" {
            let person: String = vec_input[1].to_string();
            let person_tmp = person.clone();
            let to_department = vec_input[3].to_string();
            let to_department_tmp = to_department.clone();
            if res.contains_key(&to_department) {
                println!("Contains_key initiated");
                match res.get(&to_department) {
                    Some(dpt) => {
                        let mut tmp_person_hashset = dpt.clone();
                        let tmp = tmp_person_hashset.insert(person);
                        let tmp = res.entry(to_department).or_insert(tmp_person_hashset);
                    }
                    None => {
                        let mut tmp_person_hashset = HashSet::new();
                        let tmp = tmp_person_hashset.insert(person);
                        let tmp = res.entry(to_department).or_insert(tmp_person_hashset);
                    }
                }
            } //else if !res.contains_key(&to_department) {
            let mut tmp_person_hashset = HashSet::new();
            let tmp = tmp_person_hashset.insert(person_tmp);
            let tmp_person_hashset_2 = tmp_person_hashset.clone();
            let tmp = res.insert(to_department_tmp, tmp_person_hashset_2);
            //}
        } else if command == "Remove" {
            let person: String = vec_input[1].to_string();
            let to_department = vec_input[3].to_string();
            if res.contains_key(&to_department) {
                match res.get(&to_department) {
                    Some(dpt) => {
                        let mut tmp_person_hashset = dpt.clone();
                        let tmp = tmp_person_hashset.remove(&person);
                        let tmp = res.remove(&to_department);
                        if tmp_person_hashset.is_empty() {
                            break;
                        }
                        let dpt = to_department;
                        let tmp = res.insert(dpt, tmp_person_hashset);
                    }
                    None => (),
                }
            }
        } else if command == "Move" {
            println!("move");
            //Add:
            let person: String = vec_input[1].to_string();
            let person_tmp = person.clone();
            let to_department = vec_input[5].to_string();
            let to_department_tmp = to_department.clone();
            //if res.contains_key(&to_department) {
            match res.get(&to_department) {
                Some(dpt) => {
                    let mut tmp_person_hashset = dpt.clone();
                    let tmp = tmp_person_hashset.insert(person);
                    let tmp = res.entry(to_department).or_insert(tmp_person_hashset);
                    for (key, value) in &res {
                        println!("KEY_MOVE:{}", key);
                    }
                }
                None => {
                    let mut tmp_person_hashset = HashSet::new();
                    let tmp = tmp_person_hashset.insert(person);
                    let tmp = res.entry(to_department).or_insert(tmp_person_hashset);
                }
            }
            //} else {
            let mut tmp_person_hashset = HashSet::new();
            let tmp = tmp_person_hashset.insert(person_tmp);
            let tmp = res.insert(to_department_tmp, tmp_person_hashset);
            //}

            //Remove:

            let person: String = vec_input[1].to_string();
            let to_department = vec_input[3].to_string();
            if res.contains_key(&to_department) {
                match res.get(&to_department) {
                    Some(dpt) => {
                        let mut tmp_person_hashset = dpt.clone();
                        let tmp = tmp_person_hashset.remove(&person);
                        let tmp = res.remove(&to_department);
                        if tmp_person_hashset.is_empty() {
                            break;
                        }
                        let dpt = to_department;
                        let tmp = res.insert(dpt, tmp_person_hashset);
                    }
                    None => (),
                }
            }

            for (key, value) in &res {
                println!("KEY_MOVE:{}", key);
            }
        }
    }

    //todo: implement deletion of empty values
    res
}

#[test]
fn test_read_command() {
    //let test = read_command("Add Mustafa to Yourmum".to_string());
}
/*
fn read_command(input: String) -> Command {
    let command = input.split_whitespace().next().unwrap_or("");
    let vec_input: Vec<&str> = input.split_whitespace().collect();
    if command == "Add" {
        return Command {
            person: vec_input[1].to_string(),
            from_department: None,
            to_department: Some(vec_input[3].to_string()),
            command: Some(vec_input[0].to_string()),
        };
    } else if command == "Remove" {
        return Command {
            person: Some(vec_input[1].to_string()),
            from_department: Some(vec_input[3].to_string()),
            to_department: None,
            command: Some(vec_input[0].to_string()),
        };
    } else if command == "Move" {
        return Command {
            person: Some(vec_input[1].to_string()),
            from_department: Some(vec_input[3].to_string()),
            to_department: Some(vec_input[5].to_string()),
            command: Some(vec_input[0].to_string()),
        };
    }
    return Command {
        person: None,
        from_department: None,
        to_department: None,
        command: None,
    };
}

#[derive()]
struct Command {
    person: String,
    from_department: String,
    to_department: String,
    command: String,
}

*/
