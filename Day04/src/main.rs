fn number_to_digits(n: u32) -> Vec<u32> {
    n.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect()
}

fn is_password_valid(pass: u32) -> bool {
    let digits = number_to_digits(pass);
    if digits.len() == 6 {
        let mut pair_found = false;
        let mut decrease = false;
        for i in 0..(digits.len() - 1) {
            if digits[i] > digits[i + 1] {
                decrease = true;
                break;
            }
            if digits[i] == digits[i + 1]
                && (i == 0 || digits[i] != digits[i - 1])
                && (i == digits.len() - 2 || digits[i] != digits[i + 2])
            {
                pair_found = true;
            }
        }
        pair_found && !decrease
    } else {
        false
    }
}

fn main() {
    let mut valid_passwords_count = 0;
    for i in 178416..=676461 {
        if is_password_valid(i) {
            valid_passwords_count += 1;
        }
    }
    println!("Number of valid passwords: {}", valid_passwords_count);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn password_validity() {
        assert!(is_password_valid(111111));
        assert!(!is_password_valid(1111111));
        assert!(!is_password_valid(223450));
        assert!(!is_password_valid(123789));
    }
}
