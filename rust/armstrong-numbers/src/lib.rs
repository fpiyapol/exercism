pub fn is_armstrong_number(num: u32) -> bool {
    let digits = (num as f64).log10().ceil() as u32;

    let sum = (0..digits)
        .map(|n| (num / 10_u32.pow(n) % 10).pow(digits))
        .sum::<u32>();

    num == sum
}
