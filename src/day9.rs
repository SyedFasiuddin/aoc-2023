use std::{fs::File, io::Read};

fn main() {
    let _input = "0 3 6 9 12 15\n1 3 6 10 15 21\n10 13 16 21 30 45";
    let mut input = String::new();
    let _ = File::open("inputs/9.txt")
        .unwrap()
        .read_to_string(&mut input);

    let mut hists: Vec<Vec<i64>> = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        let mut h: Vec<i64> = Vec::new();
        for num in line.split_whitespace() {
            h.push(num.trim().parse().unwrap());
        }
        hists.push(h);
    }

    let mut sum = 0;
    for hist in hists.iter_mut() {
        let mut diff: Vec<i64> = hist.clone();
        let mut tmp: Vec<i64> = Vec::new();
        let mut last: Vec<i64> = Vec::new();
        let mut should_continue = true;

        while should_continue {
            for val in diff.windows(2) {
                match val {
                    [a, b] => tmp.push(b - a),
                    _ => unreachable!(),
                }
            }
            if tmp.iter().filter(|t| **t == 0).count() == tmp.len() {
                should_continue = false;
            }

            last.insert(0, tmp[0]);

            diff.clear();
            std::mem::swap(&mut diff, &mut tmp);
            tmp.clear();
        }

        let mut s: i64 = 0;
        for num in last.iter().skip(1) {
            s = num - s;
        }
        s = hist[0] - s;
        sum += s;
    }

    println!("{}", sum);
}
