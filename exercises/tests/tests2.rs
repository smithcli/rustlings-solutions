// tests2.rs
// This test has a problem with it -- make the test compile! Make the test
// pass! Make the test fail!
// Execute `rustlings hint tests2` or use the `hint` watch subcommand for a hint.

// I AM DONE

#[cfg(test)]
mod tests {
    #[test]
    fn you_can_assert_eq() {
        let x = 2;
        let y = 2;
        assert_eq!(x, y, "testing values x: {} and y: {}", x, y);
    }
}
