// Add test for the module
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_hello() {
        assert_eq!(hello(), "Hello, world!");
    }
}

