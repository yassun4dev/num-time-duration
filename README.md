# num-time-duration

This crate provides a convenient way to create `std::time::Duration` from numbers.

Example:

```rust
use num_time_duration::NumTimeDuration;

let now = std::time::SystemTime::now();
assert_eq!(now + 1.hours(), now + std::time::Duration::from_secs(3600));
assert_eq!(now + 1.days(), now + std::time::Duration::from_secs(86400));
assert_eq!(now - 1.weeks(), now - std::time::Duration::from_secs(604800));
```

## Usage

To use `num-time-duration`, add this to your `Cargo.toml`:

```toml
[dependencies]
num-time-duration = "0.1.0"
```
