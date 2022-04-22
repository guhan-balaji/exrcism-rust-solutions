/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let code = code.replace(" ", "");
    if code.len() <= 1 || code.chars().filter(|c| !c.is_digit(10)).count() > 0 {
        false
    } else {
        code.chars()
            .rev()
            .collect::<String>()
            .char_indices()
            .map(|(i, c)| match i % 2 {
                0 => c.to_digit(10).unwrap(),
                _ => {
                    if c.to_digit(10).unwrap() * 2 > 9 {
                        (c.to_digit(10).unwrap() * 2) - 9
                    } else {
                        c.to_digit(10).unwrap() * 2
                    }
                }
            })
            .sum::<u32>()
            % 10
            == 0
    }
}
