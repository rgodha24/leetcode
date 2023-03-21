fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn can_place_flowers(mut flowerbed: Vec<i32>, n: i32) -> bool {
        flowerbed.push(0);
        flowerbed.push(1);
        let mut prev_one = -1 as i32;
        let mut count = 0;
        flowerbed.into_iter().enumerate().for_each(|(i, val)| {
            match val {
                0 => {
                    if i == 0 {
                        prev_one = -2;
                    }
                    // it's a 0, so keep on counting...
                }
                1 => {
                    // we found a one

                    // handle the first case
                    if prev_one != -1 {
                        let length = i as i32 - prev_one;
                        print!("{}: {}   ", length, (length as i32 - 2) / 2);
                        prev_one = i as i32;
                        count += (length as i32 - 2) / 2;
                    } else {
                        prev_one = i as i32;
                    }
                }
                _ => panic!("not 0 or 1"),
            };
        });

        n <= count
    }
    pub fn strong_password_checker_ii(password: String) -> bool {
        // 8+ long
        if password.len() < 8 {
            return false;
        };

        let mut last_val = String::from("");
        let mut digit_count = 0;
        let mut uppercase_count = 0;
        let mut lowercase_count = 0;
        let mut special_char_count = 0;
        let mut repeated_char_count = 0;
        for char in password.chars() {
            if !last_val.is_empty() {
                if char == last_val.as_bytes()[0].into() {
                    repeated_char_count += 1;
                }
            }
            if char.is_uppercase() {
                uppercase_count += 1;
            }
            if char.is_lowercase() {
                lowercase_count += 1;
            }
            if char.is_digit(10) {
                digit_count += 1;
            }
            if String::from("!@#$%^&*()-+").contains(char) {
                special_char_count += 1;
            }

            last_val = String::from(char);
        }

        digit_count >= 1
            && uppercase_count >= 1
            && lowercase_count >= 1
            && special_char_count >= 1
            && repeated_char_count == 0
    }
}
