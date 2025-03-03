pub fn minimum_sum(num: i32) -> i32 {
    let first_digit = num / 1000;
    let second_digit = (num / 100) % 10;
    let third_digit = (num / 10) % 10;
    let fourth_digit = num % 10;

    let mut digits = vec![first_digit, second_digit, third_digit, fourth_digit];
    digits.sort();

    (digits[0] * 10 + digits[2])  + (digits[1] * 10 + digits[3])
}

#[cfg(test)]
mod test {
    use super::minimum_sum;

    #[test]
    fn cases() {
        assert_eq!(minimum_sum(2932), 52);
        assert_eq!(minimum_sum(4009), 13);
    }
}
