pub mod prelude;
pub mod traits;

#[cfg(test)]
mod tests {
    use crate::prelude::ToStr;

    #[test]
    fn t_to_str() {
        let result = String::from("test").to_str();
        // I know it's doesn't really work, just look at the type inference above.
        assert_eq!(result, "test");
    }
}
