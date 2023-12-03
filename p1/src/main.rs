fn main() {
    let input = String::from(include_str!("input.txt"));
    println!("Solution 1: {}", solve1(&input));
    println!("Solution 2: {}", solve2(&input));
}

fn solve1(input: &String) -> usize {
    let mut sum = 0;
    for line in input.split('\n') {
        let mut first_digit_found = false;
        let mut first_digit: usize = 0;
        let mut last_digit: usize = 0;
        for ch in line.chars() {
            if ch <= '9' && ch >= '0' {
                if !first_digit_found {
                    first_digit_found = true;
                    first_digit = ch as usize - '0' as usize;
                }
                last_digit = ch as usize - '0' as usize;
            }
        }
        if !first_digit_found {
            continue;
        }
        sum += 10 * first_digit + last_digit;
    }

    sum
}

static NUMBER_STRINGS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn solve2(input: &String) -> usize {
    let mut sum = 0;
    for line in input.split('\n') {
        let mut first_digit_found = false;
        let mut first_digit: usize = 0;
        let mut last_digit: usize = 0;
        let line_chars: Vec<char> = line.chars().collect();
        for mut index in 0..line_chars.len() {
            let ch = line_chars[index];
            if ch <= '9' && ch >= '0' {
                if !first_digit_found {
                    first_digit_found = true;
                    first_digit = ch as usize - '0' as usize;
                }
                last_digit = ch as usize - '0' as usize;
            } else {
                'numstr_label: for (numstr_val, numstr) in NUMBER_STRINGS.iter().enumerate() {
                    if numstr.len() + index > line_chars.len() {
                        continue;
                    }
                    for (numstr_char_idx, numstr_char) in numstr.chars().enumerate() {
                        if numstr_char != line_chars[index + numstr_char_idx] {
                            continue 'numstr_label;
                        }
                    }
                    // found the string in NUMBER_STRINGS evaluating at numstr_val
                    if !first_digit_found {
                        first_digit_found = true;
                        first_digit = numstr_val;
                    }
                    last_digit = numstr_val;
                    index += numstr.len();
                }
            }
        }
        if !first_digit_found {
            continue;
        }
        sum += 10 * first_digit + last_digit;
    }

    sum
}
