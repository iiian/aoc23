use regex::Regex;

pub fn puzzle1(raw: &str) -> Result<u64, Box<dyn std::error::Error>> {
    let lines = raw.lines();

    let mut sum = 0;
    for line in lines {
        let first_digit = line
            .chars()
            .find(|c| c.is_ascii_digit())
            .unwrap()
            .to_digit(10)
            .unwrap() as u64;
        let last_digit = line
            .chars()
            .rev()
            .find(|c| c.is_ascii_digit())
            .unwrap()
            .to_digit(10)
            .unwrap() as u64;

        sum += first_digit * 10 + last_digit;
    }

    Ok(sum)
}

pub fn puzzle2(raw: &str) -> Result<u64, Box<dyn std::error::Error>> {
    let lines = raw.lines();

    let mut sum = 0;
    // let re = Regex::new("(one|two|three|four|five|six|seven|eight|nine|[0-9])").unwrap();
    for line in lines {
        let mut nums: Vec<u64> = vec![];

        let mut i = 0;
        let line_bytes = line.as_bytes();
        while i < line.len() {
            if line_bytes[i].is_ascii_digit() {
                nums.push((line_bytes[i] - b'0') as u64);
            }
            // three character nums
            if i + 2 < line.len() {
                match &line[i..=i + 2] {
                    "one" => nums.push(1),
                    "two" => nums.push(2),
                    "six" => nums.push(6),
                    _ => {}
                };
                if matches!(&line[i..=i + 2], "one" | "two" | "six") {
                    i += 2;
                    continue;
                }
            }
            // four character nums
            if i + 3 < line.len() {
                match &line[i..=i + 3] {
                    "four" => nums.push(4),
                    "five" => nums.push(5),
                    "nine" => nums.push(9),
                    _ => {}
                };
                if matches!(&line[i..=i + 3], "four" | "five" | "nine") {
                    i += 3;
                    if i + 3 < line.len() {
                        match &line[i..=i + 3] {
                            "four" => nums.push(4),
                            "five" => nums.push(5),
                            "nine" => nums.push(9),
                            _ => {}
                        };
                        if matches!(&line[i..=i + 2], "four" | "five" | "nine") {
                            i += 3;
                        }
                        continue;
                    }
                }
            }
            // five character nums
            if i + 4 < line.len() {
                match &line[i..=i + 4] {
                    "three" => nums.push(3),
                    "seven" => nums.push(7),
                    "eight" => nums.push(8),
                    _ => {}
                };
                if matches!(&line[i..=i + 4], "three" | "seven" | "eight") {
                    i += 4;
                    continue;
                }
            }
            i += 1;
        }

        let a = nums[0];
        let b = nums[nums.len() - 1];
        println!("{:?} : [{} ... {}]", line, a, b);

        let num = 10 * a + b;
        sum += num;
    }

    Ok(sum)
}

fn to_digit(capture: &str) -> u64 {
    match capture {
        // Match word representations of digits
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        x => x.parse().unwrap(),
    }
}

#[test]
fn test_puzzle1() {
    let input = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#;

    assert_eq!(puzzle1(input).unwrap(), 142);
}

#[test]
fn test_puzzle2() {
    let input = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#;

    assert_eq!(puzzle2(input).unwrap(), 281);

    let input = r#"threeight"#;

    assert_eq!(puzzle2(input).unwrap(), 38);
}
