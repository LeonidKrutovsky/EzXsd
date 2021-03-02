macro_rules! impl_from_str {
    ($type_name:ident) => {
        use std::str::FromStr;
        impl FromStr for $type_name {
            type Err = String;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Self::parse(s)
            }
        }
    };
}

macro_rules! impl_from_string {
    ($type_name:ident) => {
        impl From<String> for $type_name {
            fn from(s: String) -> Self {
                Self::create(s)
            }
        }
    };
}
