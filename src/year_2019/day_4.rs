#[allow(dead_code)]
pub fn part_1() {
    let input = 156218 ..= 652527;
    let count = input.into_iter()
        .filter(|&pw| valid_password(pw))
        .count();
    println!("Valid passwords: {}", count);
}

fn valid_password(password: u64) -> bool {
    let digits = get_digits(password);
    // condition 1
    if digits.len() != 6 {
        return false;
    }
    // condition 2
    let no_doubles = digits
        .windows(2)
        .all(|w| w[0] != w[1]);
    if no_doubles {
        return false;
    }
    // condition 3
    let count = digits
        .windows(2)
        .filter(|&w| { w[1] < w[0] })
        .count();
    let any_decreasing = count != 0;
    if any_decreasing {
        return false;
    }
    true
}

fn get_digits(num: u64) -> Vec<u8> {
    let mut digits: Vec<u8> = vec![];
    let mut current = num;
    while 0 < current {
        digits.push((current % 10) as u8);
        current = current / 10;
    }
    digits.into_iter().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_is_too_short() {
        assert_eq!(valid_password(11111), false);
    }

    #[test]
    fn input_is_too_long() {
        assert_eq!(valid_password(1111111), false);
    }

    #[test]
    fn valid_password_1() {
        assert_eq!(valid_password(223456), true);
    }

    #[test]
    fn valid_password_2() {
        assert_eq!(valid_password(127789), true);
    }

    #[test]
    fn example_4_1() {
        assert_eq!(valid_password(111111), true);
    }

    #[test]
    fn example_4_2() {
        assert_eq!(valid_password(223450), false);
    }

    #[test]
    fn example_4_3() {
        assert_eq!(valid_password(123789), false);
    }
}
