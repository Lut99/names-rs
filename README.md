# names-rs
A crate providing a set of names that can be used as examples or (random) suggestions.

```rust
use names::three::{all, DAN, QIN};

// They are all `&'static str` constants
assert_eq!(DAN, "Dan");
assert_eq!(QIN, "Qin");

// Lists of all of them are also available, as static slices
const names: &[&str] = all();
assert_eq!(names[3], "EVE");
```


## Installation
To use this crate in one of your projects, simply add it to your `Cargo.toml` file under `[dependencies]`:
```toml
names = { git = "https://github.com/Lut99/names-rs" }
```

Optionally, you can commit yourself to a particular tag using the additional `tag`-property:
```toml
names = { git = "https://github.com/Lut99/names-rs", tag = "v0.1.0" }
```

Or, using `features`, you can enable additional [features](#features):
```toml
names = { git = "https://github.com/Lut99/names-rs", features = ["rand"] }
```

To use only the minimum few, supply `default-features = false`:
```toml
names = { git = "https://github.com/Lut99/names-rs", default-features = false, features = ["three-uppercase"] }
```


## Usage
To use the names, simply refer to one of the constants that are defines in the crate's module.

These are ordered by first the length (e.g., `three` for 3-character names) and then the case (e.g., `uppercase` for all-caps). Then, the names are available as either separate constants (e.g., `AMY`) or as a list of all names (the `all()`-function).

For example, to access the three-letter, all-caps name `AMY`, use:
```rust
use names::three::uppercase::AMY;

assert_eq!(AMY, "AMY");
```

Note that usualcase is the default, and can be reached immediately in the length module (e.g.,
```rust
use names::three::AMY;

assert_eq!(AMY, "Amy");
```
)


## Features
### Names
All names are optional in this crate, controlled by which features are enabled. The following meta-features are present:
- `all-names`: Enables _all_ names available.
- `all-uppercase`: Enables all UPPERCASE names.
- `all-usualcase`: Enables all Usualcase names.
- `all-lowercase`: Enables all lowercase names.

Then, there are specific features for lengths of names:
- `three`: Enables all 3-character names.
- `three-uppercase`: Enables all 3-character UPPERCASE names.
- `three-usualcase`: Enables all 3-character Usualcase names.
- `three-lowercase`: Enables all 3-character lowercase names.


### Additional functionality
Aside from controlling which names are used, there are also features adding additional functionality.

- `rand`: This feature enables functions to randomly select names.
  
  For every module level, a `rand()`-function becomes available that will return a random `&'static str` based on which names are present for that module with the current collection of features. Optionally, a custom `Rng` can be given to use different sources of randomness.
  
  ```rust
  use names::three::uppercase::rand;

  // Could print any of the `AMY`, `BOB`, `CHO`, ... names!
  println!(rand());
  ```
