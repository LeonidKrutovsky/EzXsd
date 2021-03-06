macro_rules! impl_from_string {
    ($type_name:ident) => {
        impl<T: Into<String>> From<T> for $type_name {
            fn from(s: T) -> Self {
                Self(s.into().into())
            }
        }
    };
}

macro_rules! impl_as_ref {
    ($type_name:ident) => {
        impl AsRef<str> for $type_name {
            fn as_ref(&self) -> &str {
                self.0.as_ref()
            }
        }
    };
}

macro_rules! impl_display {
    ($type_name:ident) => {
        use std::fmt;
        impl fmt::Display for $type_name {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{}", self.0)
            }
        }
    };
}
