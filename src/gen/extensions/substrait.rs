// SPDX-License-Identifier: Apache-2.0
// Note that this file is auto-generated and auto-synced using `substrait-gen`.

/// Included source of [`functions_datetime`](https://github.com/substrait-io/substrait/raw/v0.77.0/extensions/functions_datetime.yaml).
const FUNCTIONS_DATETIME: &str = include_str!("../../../substrait/extensions/functions_datetime.yaml");

/// Included source of [`functions_comparison`](https://github.com/substrait-io/substrait/raw/v0.77.0/extensions/functions_comparison.yaml).
const FUNCTIONS_COMPARISON: &str = include_str!("../../../substrait/extensions/functions_comparison.yaml");

/// Included source of [`functions_geometry`](https://github.com/substrait-io/substrait/raw/v0.77.0/extensions/functions_geometry.yaml).
const FUNCTIONS_GEOMETRY: &str = include_str!("../../../substrait/extensions/functions_geometry.yaml");

/// Included source of [`functions_aggregate_approx`](https://github.com/substrait-io/substrait/raw/v0.77.0/extensions/functions_aggregate_approx.yaml).
const FUNCTIONS_AGGREGATE_APPROX: &str = include_str!("../../../substrait/extensions/functions_aggregate_approx.yaml");

/// Included source of [`functions_aggregate_decimal_output`](https://github.com/substrait-io/substrait/raw/v0.77.0/extensions/functions_aggregate_decimal_output.yaml).
const FUNCTIONS_AGGREGATE_DECIMAL_OUTPUT: &str = include_str!("../../../substrait/extensions/functions_aggregate_decimal_output.yaml");

/// Included source of [`functions_rounding_decimal`](https://github.com/substrait-io/substrait/raw/v0.77.0/extensions/functions_rounding_decimal.yaml).
const FUNCTIONS_ROUNDING_DECIMAL: &str = include_str!("../../../substrait/extensions/functions_rounding_decimal.yaml");

/// Included source of [`functions_logarithmic`](https://github.com/substrait-io/substrait/raw/v0.77.0/extensions/functions_logarithmic.yaml).
const FUNCTIONS_LOGARITHMIC: &str = include_str!("../../../substrait/extensions/functions_logarithmic.yaml");

/// Included source of [`unknown`](https://github.com/substrait-io/substrait/raw/v0.77.0/extensions/unknown.yaml).
const UNKNOWN: &str = include_str!("../../../substrait/extensions/unknown.yaml");

/// Included source of [`type_variations`](https://github.com/substrait-io/substrait/raw/v0.77.0/extensions/type_variations.yaml).
const TYPE_VARIATIONS: &str = include_str!("../../../substrait/extensions/type_variations.yaml");

/// Included source of [`functions_string`](https://github.com/substrait-io/substrait/raw/v0.77.0/extensions/functions_string.yaml).
const FUNCTIONS_STRING: &str = include_str!("../../../substrait/extensions/functions_string.yaml");

/// Included source of [`extension_types`](https://github.com/substrait-io/substrait/raw/v0.77.0/extensions/extension_types.yaml).
const EXTENSION_TYPES: &str = include_str!("../../../substrait/extensions/extension_types.yaml");

/// Included source of [`functions_arithmetic`](https://github.com/substrait-io/substrait/raw/v0.77.0/extensions/functions_arithmetic.yaml).
const FUNCTIONS_ARITHMETIC: &str = include_str!("../../../substrait/extensions/functions_arithmetic.yaml");

/// Included source of [`functions_rounding`](https://github.com/substrait-io/substrait/raw/v0.77.0/extensions/functions_rounding.yaml).
const FUNCTIONS_ROUNDING: &str = include_str!("../../../substrait/extensions/functions_rounding.yaml");

/// Included source of [`functions_aggregate_generic`](https://github.com/substrait-io/substrait/raw/v0.77.0/extensions/functions_aggregate_generic.yaml).
const FUNCTIONS_AGGREGATE_GENERIC: &str = include_str!("../../../substrait/extensions/functions_aggregate_generic.yaml");

/// Included source of [`functions_boolean`](https://github.com/substrait-io/substrait/raw/v0.77.0/extensions/functions_boolean.yaml).
const FUNCTIONS_BOOLEAN: &str = include_str!("../../../substrait/extensions/functions_boolean.yaml");

/// Included source of [`functions_arithmetic_decimal`](https://github.com/substrait-io/substrait/raw/v0.77.0/extensions/functions_arithmetic_decimal.yaml).
const FUNCTIONS_ARITHMETIC_DECIMAL: &str = include_str!("../../../substrait/extensions/functions_arithmetic_decimal.yaml");

/// Included source of [`functions_set`](https://github.com/substrait-io/substrait/raw/v0.77.0/extensions/functions_set.yaml).
const FUNCTIONS_SET: &str = include_str!("../../../substrait/extensions/functions_set.yaml");

use std::collections::HashMap;
use std::str::FromStr;
use std::sync::LazyLock;
use crate::text::simple_extensions::SimpleExtensions;
use crate::urn::Urn;

/// Map with Substrait core extensions. Maps Urns to included extensions.
pub static EXTENSIONS: LazyLock<HashMap<Urn, SimpleExtensions>> = LazyLock::new(|| {
    let mut map = HashMap::new();
    map.insert(Urn::from_str("extension:io.substrait:functions_datetime").expect("valid urn"), serde_yaml::from_str(FUNCTIONS_DATETIME).expect("a valid core extension"));
    map.insert(Urn::from_str("extension:io.substrait:functions_comparison").expect("valid urn"), serde_yaml::from_str(FUNCTIONS_COMPARISON).expect("a valid core extension"));
    map.insert(Urn::from_str("extension:io.substrait:functions_geometry").expect("valid urn"), serde_yaml::from_str(FUNCTIONS_GEOMETRY).expect("a valid core extension"));
    map.insert(Urn::from_str("extension:io.substrait:functions_aggregate_approx").expect("valid urn"), serde_yaml::from_str(FUNCTIONS_AGGREGATE_APPROX).expect("a valid core extension"));
    map.insert(Urn::from_str("extension:io.substrait:functions_aggregate_decimal_output").expect("valid urn"), serde_yaml::from_str(FUNCTIONS_AGGREGATE_DECIMAL_OUTPUT).expect("a valid core extension"));
    map.insert(Urn::from_str("extension:io.substrait:functions_rounding_decimal").expect("valid urn"), serde_yaml::from_str(FUNCTIONS_ROUNDING_DECIMAL).expect("a valid core extension"));
    map.insert(Urn::from_str("extension:io.substrait:functions_logarithmic").expect("valid urn"), serde_yaml::from_str(FUNCTIONS_LOGARITHMIC).expect("a valid core extension"));
    map.insert(Urn::from_str("extension:io.substrait:unknown").expect("valid urn"), serde_yaml::from_str(UNKNOWN).expect("a valid core extension"));
    map.insert(Urn::from_str("extension:io.substrait:type_variations").expect("valid urn"), serde_yaml::from_str(TYPE_VARIATIONS).expect("a valid core extension"));
    map.insert(Urn::from_str("extension:io.substrait:functions_string").expect("valid urn"), serde_yaml::from_str(FUNCTIONS_STRING).expect("a valid core extension"));
    map.insert(Urn::from_str("extension:io.substrait:extension_types").expect("valid urn"), serde_yaml::from_str(EXTENSION_TYPES).expect("a valid core extension"));
    map.insert(Urn::from_str("extension:io.substrait:functions_arithmetic").expect("valid urn"), serde_yaml::from_str(FUNCTIONS_ARITHMETIC).expect("a valid core extension"));
    map.insert(Urn::from_str("extension:io.substrait:functions_rounding").expect("valid urn"), serde_yaml::from_str(FUNCTIONS_ROUNDING).expect("a valid core extension"));
    map.insert(Urn::from_str("extension:io.substrait:functions_aggregate_generic").expect("valid urn"), serde_yaml::from_str(FUNCTIONS_AGGREGATE_GENERIC).expect("a valid core extension"));
    map.insert(Urn::from_str("extension:io.substrait:functions_boolean").expect("valid urn"), serde_yaml::from_str(FUNCTIONS_BOOLEAN).expect("a valid core extension"));
    map.insert(Urn::from_str("extension:io.substrait:functions_arithmetic_decimal").expect("valid urn"), serde_yaml::from_str(FUNCTIONS_ARITHMETIC_DECIMAL).expect("a valid core extension"));
    map.insert(Urn::from_str("extension:io.substrait:functions_set").expect("valid urn"), serde_yaml::from_str(FUNCTIONS_SET).expect("a valid core extension"));
    map
});