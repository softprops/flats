# flats [![Build Status](https://travis-ci.org/softprops/flats.svg?branch=master)](https://travis-ci.org/softprops/flats) [![Coverage Status](https://coveralls.io/repos/github/softprops/flats/badge.svg)](https://coveralls.io/github/softprops/flats) [![crates.io](https://img.shields.io/crates/v/flats.svg)](https://crates.io/crates/flats) [![docs.rs](https://docs.rs/flats/badge.svg)](https://docs.rs/flats) [![Master API docs](https://img.shields.io/badge/docs-master-green.svg)](https://softprops.github.io/flats)

> 🥞 flattens nested structures into a flat single dimension map

## 📦 install

Add the following to your cargo project's `Cargo.toml` file.

```toml
[dependencies]
flats = "0.1"
```

## Usage

```rust
#[macro_use]
extern crate serde_json;
extern crate flats;

use std::collections::BTreeMap;
use flats::{flatten_value, Scalar};

fn main() {
  let flat: BTreeMap<String, Scalar> = flatten_value(
    json!({
      "name": "John Doe",
      "address": {
          "city": "nyc"
      },
      "phones": [
        "+44 1234567",
        "+44 2345678"
      ]
    })
 );

 let mut expected: BTreeMap<String, Scalar> = BTreeMap::new();
 expected.insert("name".into(), "John Doe".into());
 expected.insert("address.city".into(), "nyc".into());
 expected.insert("phones[0]".into(), "+44 1234567".into());
 expected.insert("phones[1]".into(), "+44 2345678".into());

 assert_eq!(expected, flat);
}
```

Doug Tangren (softprops) 2018
