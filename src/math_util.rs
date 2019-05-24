
use cgmath::{VectorSpace, InnerSpace, BaseFloat};
use serde::{Deserialize, Deserializer};

/// Deserialize a vector and normalize it
///
/// Usage example:
/// ```ignore
/// #[derive(Deserialize)]
/// struct Car {
///     color: (f64, f64, f64),
///     #[serde(deserialize_with = "deserialize_normalized")]
///     direction: Vector3<f64>,
///     speed: f64,
/// }
/// ```
pub fn deserialize_normalized<'de, T, D>(deserializer: D) -> Result<T, D::Error>
    where
        T: InnerSpace + Deserialize<'de>,
        <T as VectorSpace>::Scalar: BaseFloat,
        D: Deserializer<'de>
{
    Ok(T::deserialize(deserializer)?.normalize())
}
