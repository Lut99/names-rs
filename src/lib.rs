//  LIB.rs
//    by Lut99
//
//  Created:
//    05 Jan 2024, 16:11:42
//  Last edited:
//    08 Jan 2024, 10:04:58
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
/// Selects a random name out of \*all\* available names (including UPPERCASE, Usualcase and lowercase).
///
/// Note that this dependent on which features are given. For example, if only the `three`-feature is given,
/// this will only return three-letter names.
///
/// # Returns
/// A [`&'static str`](str) that refers to a random constant in this crate.
///
/// # Example
/// ```rust
/// use names::rand;
///
/// // Could print any of the `AMY`, `Bob`, `cho`, ... names!
/// println!("{}", rand());
/// ```
#[cfg(all(feature = "rand", any(feature = "three-uppercase", feature = "three-usualcase", feature = "three-lowercase")))]
#[inline]
pub fn rand() -> &'static str { rand_with(&mut rand::thread_rng()) }

/// Selects a random name out of \*all\* available names (including UPPERCASE, Usualcase and lowercase) using a provided Random Number Generator (RNG).
///
/// Note that this dependent on which features are given. For example, if only the `three`-feature is given,
/// this will only return three-letter names.
///
/// # Arguments
/// - `rng`: The random number generator (as an [`Rng`]) to use.
///
/// # Returns
/// A [`&'static str`](str) that refers to a random constant in this crate.
///
/// # Example
/// ```rust
/// use names::rand_with;
/// use rand::thread_rng;
///
/// // Could print any of the `AMY`, `Bob`, `cho`, ... names!
/// println!("{}", rand_with(&mut thread_rng()));
/// ```
#[cfg(all(feature = "rand", any(feature = "three-uppercase", feature = "three-usualcase", feature = "three-lowercase")))]
#[inline]
pub fn rand_with(rng: &mut impl rand::Rng) -> &'static str {
    use rand::seq::IteratorRandom as _;

    // Construct a list of slices to take into account
    let mut sets: Vec<&'static [&'static str]> = vec![];
    #[cfg(feature = "three-uppercase")]
    sets.push(three::uppercase::all());
    #[cfg(feature = "three-usualcase")]
    sets.push(three::usualcase::all());
    #[cfg(feature = "three-lowercase")]
    sets.push(three::lowercase::all());

    // Take a random iterator sample on the flattened iterator
    // SAFETY: The unwrap() is OK because we guarantee statically the slices are populated, and the feature gates assert that at least one set is present.
    sets.into_iter().map(|s| s.iter()).flatten().choose(rng).unwrap()
}



/// Selects a random name out of \*all\* available UPPERCASE names.
///
/// Note that this dependent on which features are given. For example, if only the `three`-feature is given,
/// this will only return three-letter names.
///
/// # Returns
/// A [`&'static str`](str) that refers to a random constant in this crate.
///
/// # Example
/// ```rust
/// use names::rand_uppercase;
///
/// // Could print any of the `AMY`, `BOB`, `CHO`, ... names!
/// println!("{}", rand_uppercase());
/// ```
#[cfg(all(feature = "rand", any(feature = "three-uppercase")))]
#[inline]
pub fn rand_uppercase() -> &'static str { rand_uppercase_with(&mut rand::thread_rng()) }

/// Selects a random name out of \*all\* available UPPERCASE names using a provided Random Number Generator (RNG).
///
/// Note that this dependent on which features are given. For example, if only the `three`-feature is given,
/// this will only return three-letter names.
///
/// # Arguments
/// - `rng`: The random number generator (as an [`Rng`]) to use.
///
/// # Returns
/// A [`&'static str`](str) that refers to a random constant in this crate.
///
/// # Example
/// ```rust
/// use names::rand_uppercase_with;
/// use rand::thread_rng;
///
/// // Could print any of the `AMY`, `BOB`, `CHO`, ... names!
/// println!("{}", rand_uppercase_with(&mut thread_rng()));
/// ```
#[cfg(all(feature = "rand", any(feature = "three-uppercase")))]
#[inline]
pub fn rand_uppercase_with(rng: &mut (impl ?Sized + rand::Rng)) -> &'static str {
    use rand::seq::SliceRandom as _;

    // For now, only one option, so ez
    three::uppercase::all().choose(rng).unwrap()
}



/// Selects a random name out of \*all\* available Usualcase names.
///
/// Note that this dependent on which features are given. For example, if only the `three`-feature is given,
/// this will only return three-letter names.
///
/// # Returns
/// A [`&'static str`](str) that refers to a random constant in this crate.
///
/// # Example
/// ```rust
/// use names::rand_usualcase;
///
/// // Could print any of the `Amy`, `Bob`, `Cho`, ... names!
/// println!("{}", rand_usualcase());
/// ```
#[cfg(all(feature = "rand", any(feature = "three-usualcase")))]
#[inline]
pub fn rand_usualcase() -> &'static str { rand_usualcase_with(&mut rand::thread_rng()) }

/// Selects a random name out of \*all\* available Usualcase names using a provided Random Number Generator (RNG).
///
/// Note that this dependent on which features are given. For example, if only the `three`-feature is given,
/// this will only return three-letter names.
///
/// # Arguments
/// - `rng`: The random number generator (as an [`Rng`]) to use.
///
/// # Returns
/// A [`&'static str`](str) that refers to a random constant in this crate.
///
/// # Example
/// ```rust
/// use names::rand_usualcase_with;
/// use rand::thread_rng;
///
/// // Could print any of the `Amy`, `Bob`, `Cho`, ... names!
/// println!("{}", rand_usualcase_with(&mut thread_rng()));
/// ```
#[cfg(all(feature = "rand", any(feature = "three-usualcase")))]
#[inline]
pub fn rand_usualcase_with(rng: &mut (impl ?Sized + rand::Rng)) -> &'static str {
    use rand::seq::SliceRandom as _;

    // For now, only one option, so ez
    three::usualcase::all().choose(rng).unwrap()
}



/// Selects a random name out of \*all\* available lowercase names.
///
/// Note that this dependent on which features are given. For example, if only the `three`-feature is given,
/// this will only return three-letter names.
///
/// # Returns
/// A [`&'static str`](str) that refers to a random constant in this crate.
///
/// # Example
/// ```rust
/// use names::rand_lowercase;
///
/// // Could print any of the `amy`, `bob`, `cho`, ... names!
/// println!("{}", rand_lowercase());
/// ```
#[cfg(all(feature = "rand", any(feature = "three-lowercase")))]
#[inline]
pub fn rand_lowercase() -> &'static str { rand_lowercase_with(&mut rand::thread_rng()) }

/// Selects a random name out of \*all\* available lowercase names using a provided Random Number Generator (RNG).
///
/// Note that this dependent on which features are given. For example, if only the `three`-feature is given,
/// this will only return three-letter names.
///
/// # Arguments
/// - `rng`: The random number generator (as an [`Rng`]) to use.
///
/// # Returns
/// A [`&'static str`](str) that refers to a random constant in this crate.
///
/// # Example
/// ```rust
/// use names::rand_lowercase_with;
/// use rand::thread_rng;
///
/// // Could print any of the `amy`, `bob`, `cho`, ... names!
/// println!("{}", rand_lowercase_with(&mut thread_rng()));
/// ```
#[cfg(all(feature = "rand", any(feature = "three-lowercase")))]
#[inline]
pub fn rand_lowercase_with(rng: &mut (impl ?Sized + rand::Rng)) -> &'static str {
    use rand::seq::SliceRandom as _;

    // For now, only one option, so ez
    three::lowercase::all().choose(rng).unwrap()
}
