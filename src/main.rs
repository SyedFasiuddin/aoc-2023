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

#[allow(dead_code)]
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

#[allow(dead_code)]
fn day2a() {
    use std::{fs::File, io::Read};
    // Sample Input:
    // Game ID: set 1; set 2; ...
    //
    // Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    // Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    // Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    // Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    // Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
    //
    // Bag loaded with: 12 red, 13 green, 14 blue
    // Give the sum of ID
    //
    // Game 3 is impossible as there are only 12 red balls in bag but he showed 20
    // Similarly Game 5 4 is impossible due to 15 blue balls
    // So 1 2 5 are possible and 1 + 2 + 5 = 8 is the answer

    // let input = r"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    // Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    // Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    // Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    // Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    let mut input = String::new();
    let _ = File::open("inputs/2.txt")
        .unwrap()
        .read_to_string(&mut input);

    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    let mut sum = 0;
    for game in input.trim().split('\n') {
        let mut good_set = true;
        // Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
        let (part1, part2) = game.trim().split_once(':').unwrap();
        let id: u64 = part1[5..].parse().unwrap();

        for sets in part2.trim().split(';') {
            // 6 red, 1 blue, 3 green
            for balls in sets.trim().split(',') {
                // 6 red
                // 1 blue ....
                let (num, ball_color) = balls.trim().split_once(' ').unwrap();
                match ball_color {
                    "red" => {
                        if num.parse::<u64>().unwrap() > max_red {
                            good_set = false;
                        }
                    }
                    "green" => {
                        if num.parse::<u64>().unwrap() > max_green {
                            good_set = false;
                        }
                    }
                    "blue" => {
                        if num.parse::<u64>().unwrap() > max_blue {
                            good_set = false;
                        }
                    }
                    _ => unreachable!(),
                }
            }
        }

        if good_set {
            sum += id;
        }
    }
    println!("{sum}");
}

#[allow(dead_code)]
fn day2b() {
    use std::cmp::max;
    use std::{fs::File, io::Read};
    // Sample Input:
    // Game ID: set 1; set 2; ...
    //
    // Bag loaded with: 12 red, 13 green, 14 blue
    // Give the sum of ID
    //
    // When is each game possible?
    // When the max number of balls of any color shown were present in the bag initially
    // Which means the max of each balls from all sets
    //
    // Given, power = product of that max
    // Solution sum of all powers

    let mut input = String::new();
    let _ = File::open("inputs/2.txt")
        .unwrap()
        .read_to_string(&mut input);

    let mut sum = 0;
    for game in input.trim().split('\n') {
        // Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
        let (_, part2) = game.trim().split_once(':').unwrap();

        let mut max_r = 0;
        let mut max_g = 0;
        let mut max_b = 0;

        for sets in part2.trim().split(';') {
            // 6 red, 1 blue, 3 green
            for balls in sets.trim().split(',') {
                // 6 red
                // 1 blue ....
                let (num, ball_color) = balls.trim().split_once(' ').unwrap();
                match ball_color {
                    "red" => max_r = max(max_r, num.parse::<u64>().unwrap()),
                    "green" => max_g = max(max_g, num.parse::<u64>().unwrap()),
                    "blue" => max_b = max(max_b, num.parse::<u64>().unwrap()),
                    _ => unreachable!(),
                }
            }
        }
        sum = sum + (max_r * max_g * max_b);
    }
    println!("{sum}");
}

fn main() {}
