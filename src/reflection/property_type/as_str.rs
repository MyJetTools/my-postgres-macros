pub enum AsStr<'s> {
    AsStr(&'s str),
    AsString(String),
}

impl<'s> AsStr<'s> {
    pub fn create_as_str(src: &'s str) -> Self {
        Self::AsStr(src)
    }

    pub fn create_as_string(src: String) -> Self {
        Self::AsString(src)
    }

    pub fn as_str(&self) -> &str {
        match self {
            Self::AsStr(src) => src,
            Self::AsString(src) => src.as_str(),
        }
    }
}

impl<'s> std::fmt::Display for AsStr<'s> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AsStr::AsStr(src) => write!(f, "{}", src),
            AsStr::AsString(src) => write!(f, "{}", src),
        }
    }
}
