#[allow(dead_code)]
fn day1a() {
    // Sample Input:
    // First and last digit as two digit number, then add all of them
    // 1abc2            12
    // pqr3stu8vwx      38
    // a1b2c3d4e5f      15
    // treb7uchet       77
    use std::{fs::File, io::Read};

    let mut str = String::new();
    let _ = File::open("inputs/1.txt").unwrap().read_to_string(&mut str);
    let mut final_num = 0;

    for st in str.split('\n') {
        let mut ch: Vec<char> = st.chars().collect();
        let mut num = 0;
        for c in &ch {
            if c.is_digit(10) {
                num = c.to_digit(10).unwrap() * 10;
                break;
            }
        }
        ch.reverse();
        for c in ch {
            if c.is_digit(10) {
                num = num + c.to_digit(10).unwrap();
                break;
            }
        }
        final_num = final_num + num;
    }

    println!("{final_num}");
}

fn day1b() {
    use regex::Regex;
    use std::{fs::File, io::Read};

    // Sample Input:
    // The digits can also be spelled out
    // two1nine
    // eightwothree
    // abcone2threexyz
    // xtwone3four
    // 4nineeightseven2
    // zoneight234
    // 7pqrstsixteen

    // one two three four five six seven eight nine

    // let mut str = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen".to_string();
    let mut str = String::new();
    let _ = File::open("inputs/1.txt").unwrap().read_to_string(&mut str);

    let rx = Regex::new(r"one|two|three|four|five|six|seven|eight|nine").unwrap();
    while let Some(mat) = rx.find(&str) {
        str = match mat.as_str() {
            "one" => rx.replace(&str, "1e").to_string(),
            "two" => rx.replace(&str, "2o").to_string(),
            "three" => rx.replace(&str, "3e").to_string(),
            "four" => rx.replace(&str, "4r").to_string(),
            "five" => rx.replace(&str, "5e").to_string(),
            "six" => rx.replace(&str, "6x").to_string(),
            "seven" => rx.replace(&str, "7n").to_string(),
            "eight" => rx.replace(&str, "8t").to_string(),
            "nine" => rx.replace(&str, "9e").to_string(),
            // Because two digits can be combined as such 'oneight'
            // Thanks reddit
            _ => unreachable!(),
        };
    }
    // println!("{str}");

    let mut final_num = 0;
    for st in str.split('\n') {
        let mut ch: Vec<char> = st.chars().collect();
        let mut num = 0;
        for c in &ch {
            if c.is_digit(10) {
                num = c.to_digit(10).unwrap() * 10;
                break;
            }
        }
        ch.reverse();
        for c in ch {
            if c.is_digit(10) {
                num = num + c.to_digit(10).unwrap();
                break;
            }
        }
        final_num = final_num + num;
    }

    println!("{final_num}");
}

fn main() {
    day1b();
}
