//  LIB.rs
//    by Lut99
//
//  Created:
//    05 Jan 2024, 16:11:42
//  Last edited:
//    05 Jan 2024, 16:51:40
//  Auto updated?
//    Yes
//
//  Description:
//!   A crate providing a set of names that can be used as example names.
//

// Declare modules
#[cfg(any(feature = "three-uppercase", feature = "three-usualcase", feature = "three-lowercase"))]
pub mod three;


/***** LIBRARY *****/
/// Selects a random name out of \*all\* available names.
///
/// Note that this dependent on which features are given. For example, if only the `three`-feature is given,
/// this will only return three-letter names.
///
/// # Returns
/// A [`&'static str`](str) that refers to a random constant in this crate.
///
/// # Example
/// ```rust
/// use names::three::uppercase::rand;
///
/// // Could print any of the `AMY`, `BOB`, `CHO`, ... names!
/// println!(rand());
/// ```
#[cfg(any(feature = "three-uppercase", feature = "three-usualcase", feature = "three-lowercase"))]
#[inline]
pub fn rand() -> &'static str { rand_of(rand::thread_rng()) }
