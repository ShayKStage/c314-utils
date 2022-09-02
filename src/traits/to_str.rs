/// trait ToStr containing function/method to_str that is used tp convert between a string and a static str
pub trait ToStr {
    /// Function/Method to_str takes in a string and returns a static str
    fn to_str(self) -> &'static str;
}

/// implementation of ToStr for String
impl ToStr for String {
    /// Implementation of to_str using a Box and leaking it
    fn to_str(self) -> &'static str {
        Box::leak(self.into_boxed_str())
    }
}
