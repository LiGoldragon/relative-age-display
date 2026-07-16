//! Human-readable relative-age display.
//!
//! A [`RelativeAge`] renders an elapsed time span for a human reader as its
//! single most-significant unit, valued with exactly two decimal places, using
//! one uniform format for every unit: `42.17 seconds`, `17.80 minutes`,
//! `3.42 days`, `2.50 years`.
//!
//! The unit ladder climbs from seconds through years. Seconds are shown only up
//! to three minutes of elapsed time; past that the display promotes to minutes,
//! then hours, days, weeks, months, and finally years, each unit carrying the
//! span until it would read as an unreasonably large multiple and the next unit
//! takes over. The ladder is a single ordered table with no special cases: the
//! seconds-only-up-to-three-minutes rule is just the first row's upper bound.
//!
//! This is deliberately a shared library so every surface that shows a record's
//! age — lane and table observations, status output, dashboards — renders time
//! the same way.

use std::{fmt, time::Duration};

/// An elapsed time span carrying its human relative-age rendering.
///
/// Construct one from a [`Duration`] or from a raw nanosecond count (the shape
/// stored ages arrive in), then render it through [`fmt::Display`]. The value is
/// always the single most-significant unit with two decimal places.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RelativeAge {
    span: Duration,
}

impl RelativeAge {
    /// A relative age from an elapsed [`Duration`].
    pub const fn from_duration(span: Duration) -> Self {
        Self { span }
    }

    /// A relative age from a raw elapsed nanosecond count. Stored ages are kept
    /// as nanoseconds, so this is the ordinary construction path for them.
    pub const fn from_nanoseconds(nanoseconds: u64) -> Self {
        Self {
            span: Duration::from_nanos(nanoseconds),
        }
    }

    /// The elapsed span between two instants on the same clock, saturating to
    /// zero when `later` precedes `earlier` (clock skew reads as no elapsed
    /// age rather than a negative one).
    pub fn elapsed_between(earlier: Duration, later: Duration) -> Self {
        Self {
            span: later.saturating_sub(earlier),
        }
    }

    /// The underlying elapsed span.
    pub const fn span(self) -> Duration {
        self.span
    }

    /// The unit this span renders in, chosen as the first ladder rung whose
    /// upper bound the span falls below.
    fn unit(self) -> &'static AgeUnit {
        let elapsed_seconds = self.span.as_secs_f64();
        AGE_UNIT_LADDER
            .iter()
            .find(|unit| elapsed_seconds < unit.upper_bound_seconds)
            .unwrap_or(&AGE_UNIT_LADDER[AGE_UNIT_LADDER.len() - 1])
    }
}

impl fmt::Display for RelativeAge {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let unit = self.unit();
        let value = self.span.as_secs_f64() / unit.seconds_per_unit;
        write!(formatter, "{value:.2} {}", unit.label)
    }
}

/// One rung of the relative-age unit ladder: its plural label, how many seconds
/// one of its units spans, and the elapsed span (exclusive, in seconds) at or
/// above which the display promotes to the next coarser unit.
struct AgeUnit {
    label: &'static str,
    seconds_per_unit: f64,
    upper_bound_seconds: f64,
}

const SECONDS_PER_MINUTE: f64 = 60.0;
const SECONDS_PER_HOUR: f64 = 60.0 * SECONDS_PER_MINUTE;
const SECONDS_PER_DAY: f64 = 24.0 * SECONDS_PER_HOUR;
const SECONDS_PER_WEEK: f64 = 7.0 * SECONDS_PER_DAY;
/// The mean Gregorian month: 365.2425 days / 12.
const SECONDS_PER_MONTH: f64 = 2_629_746.0;
/// The mean Gregorian year: 365.2425 days.
const SECONDS_PER_YEAR: f64 = 31_556_952.0;

/// The ordered unit ladder. Each rung is used while the elapsed span is below
/// its `upper_bound_seconds`; the final rung (years) is the open-ended catch-all.
///
/// Chosen breakpoints, uniform "single most-significant unit" rendering:
/// - seconds below 3 minutes (the psyche's explicit seconds cutoff),
/// - minutes below 90 minutes,
/// - hours below 36 hours,
/// - days below 14 days (2 weeks),
/// - weeks below 60 days (~8.6 weeks),
/// - months below 24 months (2 years),
/// - years thereafter.
const AGE_UNIT_LADDER: [AgeUnit; 7] = [
    AgeUnit {
        label: "seconds",
        seconds_per_unit: 1.0,
        upper_bound_seconds: 3.0 * SECONDS_PER_MINUTE,
    },
    AgeUnit {
        label: "minutes",
        seconds_per_unit: SECONDS_PER_MINUTE,
        upper_bound_seconds: 90.0 * SECONDS_PER_MINUTE,
    },
    AgeUnit {
        label: "hours",
        seconds_per_unit: SECONDS_PER_HOUR,
        upper_bound_seconds: 36.0 * SECONDS_PER_HOUR,
    },
    AgeUnit {
        label: "days",
        seconds_per_unit: SECONDS_PER_DAY,
        upper_bound_seconds: 14.0 * SECONDS_PER_DAY,
    },
    AgeUnit {
        label: "weeks",
        seconds_per_unit: SECONDS_PER_WEEK,
        upper_bound_seconds: 60.0 * SECONDS_PER_DAY,
    },
    AgeUnit {
        label: "months",
        seconds_per_unit: SECONDS_PER_MONTH,
        upper_bound_seconds: 24.0 * SECONDS_PER_MONTH,
    },
    AgeUnit {
        label: "years",
        seconds_per_unit: SECONDS_PER_YEAR,
        upper_bound_seconds: f64::INFINITY,
    },
];
