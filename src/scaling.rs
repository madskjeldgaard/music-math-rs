use crate::binaryops::clip;
use num_traits::Float;

/// Map an input value from one range to another.
/// It performs clipping on the input value in the range of `from_min` and `from_max`.
/// Using generics, this function can map any type that implements the `Into` trait.
///
/// Note: If the denominator is zero, the function will return `to_min` to avoid division by zero.
#[inline]
#[must_use]
pub fn linlin<T: Float>(value: T, from_min: T, from_max: T, to_min: T, to_max: T) -> T {
    let denominator = from_max.sub(from_min);

    if denominator == T::zero() {
        return to_min;
    }

    let value = clip(value, from_min, from_max);
    let nominator = value.sub(from_min);

    nominator
        .mul(to_max.sub(to_min))
        .div(denominator)
        .add(to_min)
}

/// Map an input value from one range to another with an exponential curve. It performs clipping on
/// the input value in the range of `from_min` and `from_max`.
/// Using generics, this function can map any type that implements the `Float` trait.
///
/// Note: If the denominator is zero, the function will return `to_min` to avoid division by zero.
#[inline]
#[must_use]
pub fn linexp<T: Float>(
    value: T,
    from_min: T,
    from_max: T,
    to_min: T,
    to_max: T,
    exponent: T,
) -> T {
    let denominator = from_max.sub(from_min);
    if denominator == T::zero() {
        to_min
    } else {
        let value = clip(value, from_min, from_max);
        let value = value.sub(from_min).div(denominator).powf(exponent);

        value.mul(to_max.sub(to_min)).add(to_min)
    }
}

/// Convert a decibel value to amplitude.
/// # Panics
/// If a type conversion fails.
#[inline]
#[must_use]
pub fn dbamp<T: Float>(db: T) -> T {
    let twenty = T::from(20.0).expect("Could not convert 20.0 to T");
    let ten = T::from(10.0).expect("Could not convert 10.0 to T");
    let n = db.div(twenty);

    ten.powf(n)
}

/// Convert an amplitude value to decibels.
/// # Panics
/// If a type conversion fails.
#[inline]
#[must_use]
pub fn ampdb<T: Float>(amp: T) -> T {
    let twenty = T::from(20.0).expect("Could not convert 20.0 to T");
    twenty.mul(amp.log10())
}
