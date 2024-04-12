use num_traits::Float;

/// Linear interpolation function using generics
///
/// This function performs linear interpolation between two points. It calculates
/// the value at a specified point between the two given points.
///
/// # Type Parameters
///
/// * `T: Float` - The type of the input values. This should be a floating point type.
///
/// # Parameters
///
/// * `a: T` - The start point for interpolation.
/// * `b: T` - The end point for interpolation.
/// * `t: T` - The interpolation factor. Should be in the range [0, 1].
///
/// # Returns
///
/// * `T` - The interpolated value.
#[allow(clippy::many_single_char_names, clippy::arithmetic_side_effects)]
pub fn linear<T: Float>(a: T, b: T, t: T) -> T {
    (a * (T::one() - t)) + (b * t)
}

/// Cubic hermite interpolation function using generics
///
/// This function performs cubic Hermite interpolation between two points, with
/// gradients defined at each point. This can create a smoother interpolation
/// compared to linear interpolation.
///
/// # Type Parameters
///
/// * `T: Float` - The type of the input values. This should be a floating point type.
///
/// # Parameters
///
/// * `p0: T` - The first point, used to calculate the gradient at `p1`.
/// * `p1: T` - The start point for interpolation.
/// * `p2: T` - The end point for interpolation.
/// * `p3: T` - The second point, used to calculate the gradient at `p2`.
/// * `t: T` - The interpolation factor. Should be in the range [0, 1].
///
/// # Returns
///
/// * `T` - The interpolated value.
///
/// # Panics
/// Panics if T cannot produce a value of 2.0 or 3.0.
#[allow(clippy::many_single_char_names, clippy::arithmetic_side_effects)]
pub fn hermite<T: Float>(p0: T, p1: T, p2: T, p3: T, t: T) -> T {
    let two = T::from(2.0).unwrap();
    let three = T::from(3.0).unwrap();

    let t2 = t * t;
    let t3 = t2 * t;

    // Calculate the gradients at p1 and p2
    let m0 = (p2 - p0) / two;
    let m1 = (p3 - p1) / two;

    // Calculate the Hermite basis functions
    let a = (two * t3) - (three * t2) + T::one();
    let b = t3 - (two * t2) + t;
    let c = (-two * t3) + (three * t2);
    let d = t3 - t2;

    // Calculate the interpolated value
    (a * p1) + (b * m0) + (c * p2) + (d * m1)
}

/// Enumeration of interpolation methods
pub enum Method {
    /// No interpolation
    None,
    /// Linear interpolation
    Linear,
    /// Cubic Hermite interpolation
    Cubic,
}
