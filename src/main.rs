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

#[allow(dead_code)]
fn day3a() {
    use std::{fs::File, io::Read};
    // Sample Input:
    //
    // 467..114..
    // ...*......
    // ..35..633.
    // ......#...
    // 617*......
    // .....+.58.
    // ..592.....
    // ......755.
    // ...$.*....
    // .664.598..
    //
    // Numbers that are adjacent to a symbol is to be added (side, top, bottom, diagona)

    // let input = "467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..";
    let mut input = String::new();
    let _ = File::open("inputs/3.txt")
        .unwrap()
        .read_to_string(&mut input);

    let lines = input.trim().split('\n').collect::<Vec<&str>>();
    let mut char_arr: Vec<Vec<char>> = Default::default();
    for line in lines {
        char_arr.push(line.trim().chars().collect());
    }

    let mut sum = 0;

    let mut adjacent = |num: u64, i: usize, start_idx: usize, end_idx: usize| {
        for x in start_idx..end_idx + 1 {
            if i != char_arr.len() - 1 {
                if char_arr[i + 1][x] != '.' && !char_arr[i + 1][x].is_digit(10) {
                    sum += num;
                }
            }
            if i != 0 {
                if char_arr[i - 1][x] != '.' && !char_arr[i - 1][x].is_digit(10) {
                    sum += num;
                }
            }
        }

        if char_arr[i][end_idx] != '.' && !char_arr[i][end_idx].is_digit(10) {
            sum += num;
        }
        if char_arr[i][start_idx] != '.' && !char_arr[i][start_idx].is_digit(10) {
            sum += num;
        }
    };

    for (i, row) in char_arr.iter().enumerate() {
        let mut num: u64 = 0;

        for (j, ch) in row.iter().enumerate() {
            if ch.is_digit(10) {
                num = (num * 10) + ch.to_digit(10).unwrap() as u64;
                if j == row.len() - 1 {
                    let start_idx: usize = if num > 99 {
                        j - 3
                    } else if num > 9 {
                        j - 2
                    } else {
                        j - 1
                    };
                    let end_idx: usize = j;
                    adjacent(num, i, start_idx, end_idx);
                    num = 0;
                }
            } else if num != 0 {
                let mut start_idx: usize = if num > 99 {
                    j - 3
                } else if num > 9 {
                    j - 2
                } else {
                    j - 1
                };
                let end_idx: usize = j;

                if start_idx > 0 {
                    start_idx -= 1;
                }
                adjacent(num, i, start_idx, end_idx);
                num = 0;
            }
        }
    }

    println!("{sum}");
}

#[allow(dead_code)]
fn day3b() {
    use std::collections::HashSet;
    use std::{fs::File, io::Read};
    // Sample Input:
    //
    // 467..114..
    // ...*......
    // ..35..633.
    // ......#...
    // 617*......
    // .....+.58.
    // ..592.....
    // ......755.
    // ...$.*....
    // .664.598..
    //
    // * is a gear if there are two numbers adjacent to it
    // The product of those two numbers of all gears added up is the solution

    // let input = "467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..";
    let mut input = String::new();
    let _ = File::open("inputs/3.txt")
        .unwrap()
        .read_to_string(&mut input);

    let lines = input.trim().split('\n').collect::<Vec<&str>>();
    let mut char_arr: Vec<Vec<char>> = Default::default();
    for line in lines {
        char_arr.push(line.trim().chars().collect());
    }

    let mut sum = 0;

    for (i, row) in char_arr.iter().enumerate() {
        for (j, ch) in row.iter().enumerate() {
            if *ch != '*' {
                continue;
            }
            let mut num_set: HashSet<(usize, usize)> = HashSet::new();
            for cr in [i - 1, i, i + 1] {
                for mut cc in [j - 1, j, j + 1] {
                    if cr >= char_arr.len()
                        || cc >= char_arr[0].len()
                        || !char_arr[cr][cc].is_ascii_digit()
                    {
                        continue;
                    }
                    while cc > 0 && char_arr[cr][cc - 1].is_ascii_digit() {
                        cc -= 1;
                    }
                    num_set.insert((cr, cc));
                }
            }
            if num_set.len() != 2 {
                continue;
            }

            let mut product = 1;
            for (cr, cc) in num_set.iter() {
                let mut num = 0;
                let mut c: usize = *cc;
                while c < char_arr[0].len() && char_arr[*cr][c].is_ascii_digit() {
                    num = (num * 10) + char_arr[*cr][c].to_digit(10).unwrap();
                    c += 1;
                }
                product *= num;
            }
            sum += product;
        }
    }

    println!("{sum}");
}

#[allow(dead_code)]
fn day4a() {
    use std::{fs::File, io::Read};
    // Sample Input:
    // Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    // Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
    // Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
    // Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
    // Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
    // Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
    //
    // Each card has winning numbers on left of '|' and on right you have your numbers
    // How many of right numbers match with left, you double the points of card starting with 1 (2^n)
    // Sum of all card points is the answer

    // let input = r"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    //               Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
    //               Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
    //               Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
    //               Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
    //               Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
    let mut input = String::new();
    let _ = File::open("inputs/4.txt")
        .unwrap()
        .read_to_string(&mut input);

    let mut sum = 0;

    for card in input.trim().split('\n') {
        let (_, nums) = card.trim().split_once(':').unwrap();
        let (left, right) = nums.trim().split_once('|').unwrap();

        let mut win_nums: Vec<usize> = Default::default();
        for num in left.trim().split(' ') {
            if num.is_empty() {
                continue;
            }
            win_nums.push(num.trim().parse::<usize>().unwrap());
        }

        let mut got_nums: Vec<usize> = Default::default();
        for num in right.trim().split(' ') {
            if num.is_empty() {
                continue;
            }
            got_nums.push(num.trim().parse::<usize>().unwrap());
        }

        let mut count = 0;
        for num in got_nums {
            if win_nums.contains(&num) {
                count += 1;
            }
        }
        if count == 0 {
            continue;
        }
        sum += u64::pow(2, count - 1);
    }

    println!("{sum}");
}

#[allow(dead_code)]
fn day4b() {
    use std::{fs::File, io::Read};
    // Sample Input:
    // Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    // Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
    // Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
    // Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
    // Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
    // Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
    //
    // The total number of wins from one card will give that many subsequent cards
    // So the total of all cards you have is the ans

    // let input = r"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    //               Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
    //               Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
    //               Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
    //               Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
    //               Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
    let mut input = String::new();
    let _ = File::open("inputs/4.txt")
        .unwrap()
        .read_to_string(&mut input);

    let lines: Vec<_> = input.trim().split('\n').collect();
    let mut cards: Vec<u64> = vec![1; lines.len()];
    for card in lines {
        let (crd, nums) = card.trim().split_once(':').unwrap();
        let (left, right) = nums.trim().split_once('|').unwrap();

        let (_, card_num) = crd.trim().split_once(' ').unwrap();
        let card_num = card_num.trim().parse::<usize>().unwrap();

        let mut win_nums: Vec<usize> = Default::default();
        for num in left.trim().split(' ') {
            if num.is_empty() {
                continue;
            }
            win_nums.push(num.trim().parse::<usize>().unwrap());
        }

        let mut got_nums: Vec<usize> = Default::default();
        for num in right.trim().split(' ') {
            if num.is_empty() {
                continue;
            }
            got_nums.push(num.trim().parse::<usize>().unwrap());
        }

        let mut count = 0;
        for num in got_nums {
            if win_nums.contains(&num) {
                count += 1;
            }
        }

        for c in card_num..card_num + count as usize {
            cards[c] += cards[card_num - 1];
        }
    }

    println!("{}", cards.iter().sum::<u64>());
}

#[allow(dead_code)]
fn day5a() {
    use std::{fs::File, io::Read};
    //     let input = r"seeds: 79 14 55 13
    //
    // seed-to-soil map:
    // 50 98 2
    // 52 50 48
    //
    // soil-to-fertilizer map:
    // 0 15 37
    // 37 52 2
    // 39 0 15
    //
    // fertilizer-to-water map:
    // 49 53 8
    // 0 11 42
    // 42 0 7
    // 57 7 4
    //
    // water-to-light map:
    // 88 18 7
    // 18 25 70
    //
    // light-to-temperature map:
    // 45 77 23
    // 81 45 19
    // 68 64 13
    //
    // temperature-to-humidity map:
    // 0 69 1
    // 1 0 69
    //
    // humidity-to-location map:
    // 60 56 37
    // 56 93 4";

    let mut input = String::new();
    let _ = File::open("inputs/5.txt")
        .unwrap()
        .read_to_string(&mut input);

    let mut seeds: Vec<u64> = Vec::new();
    let (_, seeds_str) = input.split('\n').nth(0).unwrap().split_once(':').unwrap();
    seeds_str
        .trim()
        .split(' ')
        .for_each(|num| seeds.push(num.parse().unwrap()));

    let map_stuff: Vec<_> = input.split("\n\n").skip(1).collect();
    for curr_map in map_stuff {
        let (_, map_stuff) = curr_map.trim().split_once(':').unwrap();

        let mut ranges: Vec<Vec<u64>> = Default::default();
        for line in map_stuff.trim().split('\n') {
            let mut nums: Vec<u64> = Vec::new();
            line.trim()
                .split(' ')
                .for_each(|num| nums.push(num.parse().unwrap()));
            if nums.len() != 3 {
                panic!("Shit went wrong!");
            }
            ranges.push(nums);
        }

        let mut new = Vec::new();
        for seed in &seeds {
            let mut broke = false;
            for range in &ranges {
                let (dst, src, rng) = (range[0], range[1], range[2]);
                if *seed >= src && *seed < src + rng {
                    new.push(dst + (seed - src));
                    broke = true;
                }
            }
            if !broke {
                new.push(*seed);
            }
        }
        seeds = new;
    }
    println!("{}", seeds.iter().min().unwrap());
}

#[allow(dead_code)]
fn day5b() {
    use std::cmp::{max, min};
    use std::{fs::File, io::Read};

    // let input = r"seeds: 79 14 55 13
    //
    // seed-to-soil map:
    // 50 98 2
    // 52 50 48
    //
    // soil-to-fertilizer map:
    // 0 15 37
    // 37 52 2
    // 39 0 15
    //
    // fertilizer-to-water map:
    // 49 53 8
    // 0 11 42
    // 42 0 7
    // 57 7 4
    //
    // water-to-light map:
    // 88 18 7
    // 18 25 70
    //
    // light-to-temperature map:
    // 45 77 23
    // 81 45 19
    // 68 64 13
    //
    // temperature-to-humidity map:
    // 0 69 1
    // 1 0 69
    //
    // humidity-to-location map:
    // 60 56 37
    // 56 93 4";

    let mut input = String::new();
    let _ = File::open("inputs/5.txt")
        .unwrap()
        .read_to_string(&mut input);

    let mut seeds: Vec<(u64, u64)> = Vec::new();
    let (_, seeds_str) = input.split('\n').nth(0).unwrap().split_once(':').unwrap();

    let s = seeds_str.trim().split(' ').collect::<Vec<_>>();
    if s.len() % 2 != 0 {
        panic!("Shit went wrong!");
    }
    for (idx, _) in s.iter().enumerate() {
        if idx % 2 != 0 {
            continue;
        }
        seeds.push((
            s[idx].parse().unwrap(),
            s[idx].parse::<u64>().unwrap() + s[idx + 1].parse::<u64>().unwrap(),
        ));
    }

    let map_stuff: Vec<_> = input.split("\n\n").skip(1).collect();
    for curr_map in map_stuff {
        let (_, map_stuff) = curr_map.trim().split_once(':').unwrap();

        let mut ranges: Vec<Vec<u64>> = Default::default();
        for line in map_stuff.trim().split('\n') {
            let mut nums: Vec<u64> = Vec::new();
            line.trim()
                .split(' ')
                .for_each(|num| nums.push(num.parse().unwrap()));
            if nums.len() != 3 {
                panic!("Shit went wrong!");
            }
            ranges.push(nums);
        }

        let mut new = Vec::new();
        while let Some((start, end)) = seeds.pop() {
            let mut broke = false;
            for range in &ranges {
                let (dst, src, rng) = (range[0], range[1], range[2]);
                let overlap_start = max(start, src);
                let overlap_end = min(end, src + rng);
                if overlap_start < overlap_end {
                    new.push((overlap_start - src + dst, overlap_end - src + dst));
                    if overlap_start > start {
                        seeds.push((start, overlap_start));
                    }
                    if end > overlap_end {
                        seeds.push((overlap_end, end));
                    }
                    broke = true;
                    break;
                }
            }
            if !broke {
                new.push((start, end));
            }
        }
        seeds = new;
    }
    seeds.sort();
    println!("{:?}", seeds[0]);
}

#[allow(dead_code)]
fn day6a() {
    use std::{fs::File, io::Read};
    // Sample Input:
    // Time:      7  15   30
    // Distance:  9  40  200
    //
    // Go more distance than given distance in each race
    // 1ms = 1mm/s speed increase
    // find how many ways you can go more than recorded distance in that time

    // let input = "Time:      7  15   30\nDistance:  9  40  200";
    let mut input = String::new();
    let _ = File::open("inputs/6.txt")
        .unwrap()
        .read_to_string(&mut input);

    let (time, distance) = input.split_once('\n').unwrap();

    let (_, time_str) = time.trim().split_once(':').unwrap();
    let mut time: Vec<u64> = Vec::new();
    time_str.trim().split(' ').for_each(|num| {
        if !num.is_empty() {
            time.push(num.parse().unwrap())
        }
    });

    let (_, distance_str) = distance.trim().split_once(':').unwrap();
    let mut distance: Vec<u64> = Vec::new();
    distance_str.trim().split(' ').for_each(|num| {
        if !num.is_empty() {
            distance.push(num.parse().unwrap())
        }
    });

    let mut product = 1;
    for (t, d) in time.iter().zip(distance) {
        let mut count = 0;
        for x in 1..*t {
            if x * (t - x) > d {
                count += 1;
            }
        }
        product *= count;
    }

    println!("{product}");
}

fn main() {}
