pub trait ToStr {
    fn to_str(self) -> &'static str;
}

impl ToStr for String {
    fn to_str(self) -> &'static str {
        Box::leak(self.into_boxed_str())
    }
}