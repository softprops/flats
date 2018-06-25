//! `flats` is a crate that transforms nested serde `Serialize` types into a one dimensional,
//! flat map keys and values.
//! Nested structures are represented as map keys that represent structual paths to values
//!
//! ```rust
//! #[macro_use]
//! extern crate serde_json;
//! extern crate flats;
//!
//! use std::collections::BTreeMap;
//! use flats::{flatten_value, Scalar};
//!
//! fn main() {
//!   let flat: BTreeMap<String, Scalar> = flatten_value(
//!     json!({
//!       "name": "John Doe",
//!       "address": {
//!           "city": "nyc"
//!       },
//!       "phones": [
//!         "+44 1234567",
//!         "+44 2345678"
//!       ]
//!     })
//!  );
//!
//!  let mut expected: BTreeMap<String, Scalar> = BTreeMap::new();
//!  expected.insert("name".into(), "John Doe".into());
//!  expected.insert("address.city".into(), "nyc".into());
//!  expected.insert("phones[0]".into(), "+44 1234567".into());
//!  expected.insert("phones[1]".into(), "+44 2345678".into());
//!
//!  assert_eq!(expected, flat);
//! }
//! ```

#![deny(missing_docs)]

extern crate serde;
#[macro_use]
extern crate serde_derive;
#[cfg(test)]
#[macro_use]
extern crate serde_json;
#[cfg(not(test))]
extern crate serde_json;

// Std Lib
use std::collections::BTreeMap;

// Third Party
use serde::Serialize;
use serde_json::Value;

// Ours
mod scalar;
pub use scalar::Scalar;

// re-export exposed type
pub use serde_json::Result;

/// Flattens nested structures into a one dimensional map
///
/// This first serializes the structure to a `serde_json::Value`
/// which may fail, hence the `Result` type.
pub fn flatten<S>(nested: S) -> Result<BTreeMap<String, Scalar>>
where
    S: Serialize,
{
    serde_json::to_value(nested).map(flatten_value)
}

/// Flattens nested `serde_json::Value` instances into a one dimensional map
pub fn flatten_value(value: Value) -> BTreeMap<String, Scalar> {
    fn fold<'a>(
        result: &'a mut BTreeMap<String, Scalar>,
        val: Value,
        path: Option<String>,
    ) -> &'a mut BTreeMap<String, Scalar> {
        match val {
            Value::Object(fields) => {
                for (k, v) in fields.into_iter() {
                    fold(
                        result,
                        v,
                        path.clone()
                            .map(|p| format!("{}.{}", p, k))
                            .or_else(|| Some(k.to_string())),
                    );
                }
            }
            Value::Array(v) => {
                for (idx, elem) in v.into_iter().enumerate().into_iter() {
                    fold(
                        result,
                        elem,
                        path.clone()
                            .map(|p| format!("{}[{}]", p, idx))
                            .or_else(|| Some(format!("[{}]", idx))),
                    );
                }
            }
            Value::Bool(scalar) => {
                result.insert(path.unwrap_or_default(), Scalar::Bool(scalar));
            }
            Value::String(scalar) => {
                result.insert(path.unwrap_or_default(), Scalar::String(scalar));
            }
            Value::Number(scalar) => {
                result.insert(path.unwrap_or_default(), Scalar::Number(scalar));
            }
            Value::Null => {
                result.insert(path.unwrap_or_default(), Scalar::Null);
            }
        };
        result
    }
    fold(&mut BTreeMap::new(), value, None).to_owned()
}

#[cfg(test)]
mod tests {
    use super::{flatten, serde_json, Scalar};
    use std::collections::BTreeMap;

    #[test]
    fn flattens_nested_maps() {
        let result = flatten(json!({
            "foo": {
                "bar": {
                    "baz": 3
                }
            }
        })).unwrap();
        let mut expected: BTreeMap<String, Scalar> = BTreeMap::new();
        expected.insert("foo.bar.baz".into(), 3.into());
        assert_eq!(result, expected)
    }

    #[test]
    fn flattens_to_serializable() {
        let result = flatten(json!({
            "foo": {
                "bar": {
                    "baz": 3
                }
            }
        })).unwrap();
        assert_eq!(
            serde_json::to_string(&result).unwrap(),
            r#"{"foo.bar.baz":3}"#
        )
    }
}
