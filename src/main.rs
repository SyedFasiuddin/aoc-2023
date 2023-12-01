fn day1a() {
    // Sample Input:
    // First and last digit as two digit number, then add all of them
    // 1abc2            12
    // pqr3stu8vwx      38
    // a1b2c3d4e5f      15
    // treb7uchet       77
    use std::{fs::File, io::Read};

    let mut str = String::new();
    let _ = File::open("inputs/1a.txt").unwrap().read_to_string(&mut str);
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
    day1a();
}
