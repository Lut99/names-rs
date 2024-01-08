//  LIB.rs
//    by Lut99
//
//  Created:
//    05 Jan 2024, 16:11:42
//  Last edited:
//    08 Jan 2024, 10:08:58
//  Auto updated?
//    Yes
//
//  Description:
//!   A crate providing a set of names that can be used as examples or (random) suggestions.
//!   
//!   ```rust
//!   use names::three::{all, DAN, QIN};
//!   
//!   // They are all `&'static str` constants
//!   assert_eq!(DAN, "Dan");
//!   assert_eq!(QIN, "Qin");
//!   
//!   // Lists of all of them are also available, as static slices
//!   const names: &[&str] = all();
//!   assert_eq!(names[3], "EVE");
//!   ```
//!   
//!   
//!   # Installation
//!   To use this crate in one of your projects, simply add it to your `Cargo.toml` file under `[dependencies]`:
//!   ```toml
//!   names = { git = "https://github.com/Lut99/names-rs" }
//!   ```
//!   
//!   Optionally, you can commit yourself to a particular tag using the additional `tag`-property:
//!   ```toml
//!   names = { git = "https://github.com/Lut99/names-rs", tag = "v0.1.0" }
//!   ```
//!   
//!   Or, using `features`, you can enable additional [features](#features):
//!   ```toml
//!   names = { git = "https://github.com/Lut99/names-rs", features = ["rand"] }
//!   ```
//!   
//!   To use only the minimum few, supply `default-features = false`:
//!   ```toml
//!   names = { git = "https://github.com/Lut99/names-rs", default-features = false, features = ["three-uppercase"] }
//!   ```
//!   
//!   
//!   # Usage
//!   To use the names, simply refer to one of the constants that are defines in the crate's module.
//!   
//!   These are ordered by first the length (e.g., `three` for 3-character names) and then the case (e.g., `uppercase` for all-caps). Then, the names are available as either separate constants (e.g., `AMY`) or as a list of all names (the `all()`-function).
//!   
//!   For example, to access the three-letter, all-caps name `AMY`, use:
//!   ```rust
//!   use names::three::uppercase::AMY;
//!   
//!   assert_eq!(AMY, "AMY");
//!   ```
//!   
//!   Note that usualcase is the default, and can be reached immediately in the length module (e.g.,
//!   ```rust
//!   use names::three::AMY;
//!   
//!   assert_eq!(AMY, "Amy");
//!   ```
//!   )
//!   
//!   
//!   # Features
//!   ## Names
//!   All names are optional in this crate, controlled by which features are enabled. The following meta-features are present:
//!   - `all-names`: Enables _all_ names available.
//!   - `all-uppercase`: Enables all UPPERCASE names.
//!   - `all-usualcase`: Enables all Usualcase names.
//!   - `all-lowercase`: Enables all lowercase names.
//!   
//!   Then, there are specific features for lengths of names:
//!   - `three`: Enables all 3-character names.
//!   - `three-uppercase`: Enables all 3-character UPPERCASE names.
//!   - `three-usualcase`: Enables all 3-character Usualcase names.
//!   - `three-lowercase`: Enables all 3-character lowercase names.
//!   
//!   
//!   ## Additional functionality
//!   Aside from controlling which names are used, there are also features adding additional functionality.
//!   
//!   - `rand`: This feature enables functions to randomly select names.
//!     
//!     For every module level, a `rand()`-function becomes available that will return a random `&'static str` based on which names are present for that module with the current collection of features. Optionally, a custom `Rng` can be given to use different sources of randomness.
//!     
//!     ```rust
//!     use names::three::uppercase::rand;
//!   
//!     // Could print any of the `AMY`, `BOB`, `CHO`, ... names!
//!     println!("{}", rand());
//!     ```
//!   
//!   
//!   # Contribution
//!   If you would like to contribute to this crate, please feel free to create [an issue](https://github.com/Lut99/names-rs/issues) or [a pull request](https://github.com/Lut99/names-rs/pulls)! I am maintaining this crate as a hobby though, so I may not always immediately respond.
//!   
//!   
//!   # License
//!   This project is currently licensed under Apache 2.0. See [LICENSE](./LICENSE) for more details.
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
