pub fn format_kred(koin: u64) -> String {
    let whole = koin / 100_000_000;
    let fraction = koin % 100_000_000;
    format!(
        "{}.{}",
        whole,
        format!("{:0>8}", fraction).trim_end_matches('0')
    )
}
