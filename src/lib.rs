//! This crate provides a convenient way to create `std::time::Duration` from numbers.
//!
//! Example:
//!
//! ```rust
//! use num_time_duration::NumTimeDuration;
//!
//! let now = std::time::SystemTime::now();
//! assert_eq!(now + 1.hours(), now + std::time::Duration::from_secs(3600));
//! assert_eq!(now + 1.days(), now + std::time::Duration::from_secs(86400));
//! assert_eq!(now - 1.weeks(), now - std::time::Duration::from_secs(604800));
//! ```

use std::time::Duration;

pub trait NumTimeDuration: num_traits::PrimInt {
    /// Creates a new Duration from the specified number of nanoseconds.
    ///
    /// ```rust
    /// use num_time_duration::NumTimeDuration;
    /// use std::time::Duration;
    ///
    /// assert_eq!(1.nanoseconds(), Duration::from_nanos(1));
    /// ```
    fn nanoseconds(&self) -> Duration;

    /// Creates a new Duration from the specified number of microseconds.
    ///
    /// ```rust
    /// use num_time_duration::NumTimeDuration;
    /// use std::time::Duration;
    ///
    /// assert_eq!(1.microseconds(), Duration::from_micros(1));
    /// ```
    fn microseconds(&self) -> Duration;

    /// Creates a new Duration from the specified number of milliseconds.
    ///
    /// ```rust
    /// use num_time_duration::NumTimeDuration;
    /// use std::time::Duration;
    ///
    /// assert_eq!(1.milliseconds(), Duration::from_millis(1));
    /// ```
    fn milliseconds(&self) -> Duration;

    /// Creates a new Duration from the specified number of seconds.
    ///
    /// ```rust
    /// use num_time_duration::NumTimeDuration;
    /// use std::time::Duration;
    ///
    /// assert_eq!(1.seconds(), Duration::from_secs(1));
    /// ```
    fn seconds(&self) -> Duration;

    /// Creates a new Duration from the specified number of minutes.
    ///
    /// ```rust
    /// use num_time_duration::NumTimeDuration;
    /// use std::time::Duration;
    ///
    /// assert_eq!(1.minutes(), Duration::from_secs(60));
    /// ```
    fn minutes(&self) -> Duration;

    /// Creates a new Duration from the specified number of hours.
    ///
    /// ```rust
    /// use num_time_duration::NumTimeDuration;
    /// use std::time::Duration;
    ///
    /// assert_eq!(1.hours(), Duration::from_secs(3600));
    /// ```
    fn hours(&self) -> Duration;

    /// Creates a new Duration from the specified number of days.
    ///
    /// ```rust
    /// use num_time_duration::NumTimeDuration;
    /// use std::time::Duration;
    ///
    /// assert_eq!(1.days(), Duration::from_secs(86400));
    /// ```
    fn days(&self) -> Duration;

    /// Creates a new Duration from the specified number of weeks.
    ///
    /// ```rust
    /// use num_time_duration::NumTimeDuration;
    /// use std::time::Duration;
    ///
    /// assert_eq!(1.weeks(), Duration::from_secs(604800));
    /// ```
    fn weeks(&self) -> Duration;
}

impl NumTimeDuration for i32 {
    fn nanoseconds(&self) -> Duration {
        Duration::from_nanos(*self as u64)
    }

    fn microseconds(&self) -> Duration {
        Duration::from_micros(*self as u64)
    }

    fn milliseconds(&self) -> Duration {
        Duration::from_millis(*self as u64)
    }

    fn seconds(&self) -> Duration {
        Duration::from_secs(*self as u64)
    }

    fn minutes(&self) -> Duration {
        let seconds: Duration = self.seconds();
        60 * seconds
    }

    fn hours(&self) -> Duration {
        let minutes: Duration = self.minutes();
        60 * minutes
    }

    fn days(&self) -> Duration {
        let hours: Duration = self.hours();
        24 * hours
    }

    fn weeks(&self) -> Duration {
        let days: Duration = self.days();
        7 * days
    }
}
