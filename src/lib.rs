//!# c314-utils
//!
//!> Small Rust utilities
//!
//!## Usage
//!
//!Add the following to the [dependencies] section of your Cargo.toml file. 
//!
//!```toml
//!c314-utils = "0.1.2" 
//!``` 
//!
//!Now you can use it like this:
//!
//!```rust
//!use c314-utils::prelude::ToStr;
//!
//!fn main() {
//!    let converted_from_string_to_str = String::from("Hello World1").to_str();
//!    println!("{}", converted_from_string_to_str);
//!}
//!```
//!
//!Currently only contains ToStr, if you have any ideas, please create an issue at [github.com/314ShadePi/c314-utils](https://github.com/314ShadePi/c314-utils)
//!
//!I created this library to make [Yew](https://crates.io/crates/yew) do as I wanted

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
