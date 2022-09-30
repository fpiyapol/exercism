pub fn nth(n: u32) -> u32 {
    (2..).filter(|a| is_prime(*a)).nth(n as usize).unwrap()
}

fn is_prime(n: u32) -> bool {
    let n_sqrt = (n as f32).sqrt() as u32;
    !(2..=n_sqrt).any(|a| n % a == 0)
}
