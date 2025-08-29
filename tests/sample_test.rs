#[cfg(test)]
mod tests {
    #[test]
    fn test_addition() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_greeting() {
        let greeting = "Hello, Rust learning!";
        assert_eq!(greeting, "Hello, Rust learning!");
    }
}