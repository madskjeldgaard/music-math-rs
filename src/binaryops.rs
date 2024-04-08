use num_traits::Float;

/// Clip two values within a range
pub fn clip<T: PartialOrd>(value: T, min: T, max: T) -> T {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

/// Wrap a value within a range
pub fn wrap<T: Float>(value: T, min: T, max: T) -> T {
    let range = max.sub(min);
    if value < min {
        // max - (min - value) % range
        max.sub(min.sub(value)).rem(range)
    } else if value > max {
        min.add(value.sub(max).rem(range))
    } else {
        value
    }
}

/// Fold a value within a range
/// # Panics
/// If a type conversion fails.
pub fn fold<T: Float>(value: T, min: T, max: T) -> T {
    let range = max.sub(min);
    let two = T::from(2.0).expect("Could not convert 2.0 to T");
    let r = range.mul(two);
    let value = (value.sub(min).rem(r)).abs();

    // Compute return value
    min.add(if value > range { r.sub(value) } else { value })
}
