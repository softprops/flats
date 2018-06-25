//! `flats` is a crate that transforms nested serde `Serialize` types into a one dimensional,
//! flat map keys and values.
//! Nested structures are represented as map keys that represent structual paths to values
//!
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[cfg(test)]
#[macro_use]
extern crate maplit;

// Std Lib
use std::collections::BTreeMap;

// Third Party
use serde::Serialize;
use serde_json::Value;

// Ours
mod scalar;
pub use scalar::Scalar;

/// Flattens nested structures into one dimentional map
pub fn flatten<S>(nested: S) -> serde_json::Result<BTreeMap<String, Scalar>>
where
    S: Serialize,
{
    Ok(flatten_value(serde_json::to_value(nested)?))
}

/// Flattens nested `serde_json::Value`  into one dimentional map
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
    fold(&mut BTreeMap::new(), value, None).clone()
}

#[cfg(test)]
mod tests {
    use super::{flatten, serde_json};

    #[test]
    fn flattens_nested_maps() {
        let result = flatten(hashmap! {
            "foo" => hashmap!{
                "bar" => hashmap! {
                    "baz" => 3
                }
            }
        }).unwrap();
        assert_eq!(
            result,
            btreemap! {
                String::from("foo.bar.baz") => 3.into()
            }
        )
    }

    #[test]
    fn flattens_to_serializable() {
        let result = flatten(hashmap! {
            "foo" => hashmap!{
                "bar" => hashmap! {
                    "baz" => 3
                }
            }
        }).unwrap();
        assert_eq!(
            serde_json::to_string(&result).unwrap(),
            r#"{"foo.bar.baz":3}"#
        )
    }
}
