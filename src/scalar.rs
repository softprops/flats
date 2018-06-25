//! Representations of scalar values
//!

// Std Lib
use std::fmt;

// Third Party
use serde_json;

/// Container for single, scalar values
#[derive(Debug, PartialEq, Serialize, Clone)]
#[serde(untagged)]
pub enum Scalar {
  Bool(bool),
  Number(serde_json::Number),
  String(String),
  Null,
}

impl fmt::Display for Scalar {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      Scalar::Bool(ref value) => write!(f, "{}", value),
      Scalar::Number(ref value) => write!(f, "{}", value),
      Scalar::String(ref value) => write!(f, "{}", value),
      Scalar::Null => f.write_str("null"),
    }
  }
}

macro_rules! from_integer {
    ($($ty:ident)*) => {
        $(
            impl From<$ty> for Scalar {
                fn from(n: $ty) -> Self {
                    Scalar::Number(n.into())
                }
            }
        )*
    };
}

from_integer! {
    i8 i16 i32 i64 isize
    u8 u16 u32 u64 usize
}

impl From<f32> for Scalar {
  fn from(f: f32) -> Self {
    From::from(f as f64)
  }
}

impl From<f64> for Scalar {
  fn from(f: f64) -> Self {
    serde_json::Number::from_f64(f).map_or(Scalar::Null, Scalar::Number)
  }
}

impl From<bool> for Scalar {
  fn from(f: bool) -> Self {
    Scalar::Bool(f)
  }
}

impl From<String> for Scalar {
  fn from(f: String) -> Self {
    Scalar::String(f)
  }
}

impl<'a> From<&'a str> for Scalar {
  fn from(f: &str) -> Self {
    Scalar::String(f.to_string())
  }
}

impl<'a> From<::std::borrow::Cow<'a, str>> for Scalar {
  fn from(f: ::std::borrow::Cow<'a, str>) -> Self {
    Scalar::String(f.into_owned())
  }
}

#[cfg(test)]
mod tests {
  use super::Scalar;
  #[test]
  fn null_display() {
    assert_eq!(format!("{}", Scalar::Null), "null")
  }

  #[test]
  fn string_display() {
    assert_eq!(format!("{}", Scalar::String("test".into())), "test")
  }

  #[test]
  fn number_display() {
    assert_eq!(format!("{}", Scalar::Number(2.into())), "2")
  }

  #[test]
  fn bool_display() {
    assert_eq!(format!("{}", Scalar::Bool(true)), "true")
  }
}
