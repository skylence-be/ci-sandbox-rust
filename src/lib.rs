//! Tiny library so the CI pipeline has real code to format, lint, and test.

use anyhow::{Result, bail};

/// Adds two numbers, refusing results that overflow i32.
pub fn checked_add(a: i32, b: i32) -> Result<i32> {
    match a.checked_add(b) {
        Some(sum) => Ok(sum),
        None => bail!("{a} + {b} overflows i32"),
    }
}

#[cfg(test)]
mod tests {
    use super::checked_add;

    #[test]
    fn adds_small_numbers() {
        assert_eq!(checked_add(2, 3).unwrap(), 5);
    }

    #[test]
    fn rejects_overflow() {
        assert!(checked_add(i32::MAX, 1).is_err());
    }
}
