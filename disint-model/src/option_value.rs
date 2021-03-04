#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum OptionValue {
    String(String),
    Int(i32),
}

impl std::fmt::Display for OptionValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OptionValue::String(v) => v.fmt(f),
            OptionValue::Int(v) => v.fmt(f),
        }
    }
}

impl PartialEq<str> for OptionValue {
    fn eq(&self, other: &str) -> bool {
        match self {
            OptionValue::String(v) => v == other,
            _ => false,
        }
    }
}

impl PartialEq<OptionValue> for str {
    fn eq(&self, other: &OptionValue) -> bool {
        match other {
            OptionValue::String(v) => v == self,
            _ => false,
        }
    }
}

impl PartialEq<i32> for OptionValue {
    fn eq(&self, other: &i32) -> bool {
        match self {
            OptionValue::Int(v) => v == other,
            _ => false,
        }
    }
}

impl PartialEq<OptionValue> for i32 {
    fn eq(&self, other: &OptionValue) -> bool {
        match other {
            OptionValue::Int(v) => v == self,
            _ => false,
        }
    }
}

impl OptionValue {
    pub fn try_into_string(self) -> Result<String, Self> {
        match self {
            OptionValue::String(v) => Ok(v),
            other => Err(other),
        }
    }

    pub fn try_into_int(self) -> Result<i32, Self> {
        match self {
            OptionValue::Int(v) => Ok(v),
            other => Err(other),
        }
    }

    pub fn into_string(self) -> String {
        self.try_into_string().expect("given OptionValue is not String")
    }

    pub fn into_int(self) -> i32 {
        self.try_into_int().expect("given OptionValue is not Int")
    }

    pub fn try_as_str(&self) -> Option<&str> {
        match self {
            OptionValue::String(v) => Some(v),
            _ => None,
        }
    }

    pub fn try_as_int(&self) -> Option<i32> {
        match self {
            OptionValue::Int(v) => Some(*v),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &str {
        self.try_as_str().expect("given OptionValue is not String")
    }
}
