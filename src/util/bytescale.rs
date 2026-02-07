pub fn petabytes_to_bytes(units: u64) -> u128 {
    const PB: u128 = 1_000_000_000_000_000_000;
    PB * units as u128
}

#[cfg(test)]
mod tests {
    use super::petabytes_to_bytes;

    #[test]
    fn converts_correctly() {
        assert_eq!(petabytes_to_bytes(1), 1_000_000_000_000_000_000);
        assert_eq!(petabytes_to_bytes(2), 2_000_000_000_000_000_000);
    }
}
