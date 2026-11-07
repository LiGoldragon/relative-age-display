//! Human-readable relative-age presentation over typed Datomic values.

use std::{fmt, time::Duration};

use datom_codec::{Decimal, Decimating};

#[rustfmt::skip]
pub mod generated;

pub use generated::relative_age_display::{HumanReadableMagnitude, HumanReadableTime};

/// An elapsed time span that projects its most-significant unit for a human reader.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RelativeAge {
    span: Duration,
}

impl RelativeAge {
    pub const fn from_duration(span: Duration) -> Self {
        Self { span }
    }

    pub const fn from_nanoseconds(nanoseconds: u64) -> Self {
        Self {
            span: Duration::from_nanos(nanoseconds),
        }
    }

    pub fn elapsed_between(earlier: Duration, later: Duration) -> Self {
        Self {
            span: later.saturating_sub(earlier),
        }
    }

    pub const fn span(self) -> Duration {
        self.span
    }

    /// Convert this elapsed span into the closed Datomic value whose variant
    /// retains the selected unit and whose payload is a finite decimal.
    pub fn into_human_readable_time(self) -> HumanReadableTime {
        let unit = self.unit();
        unit.unit.into_human_readable_time(quantized_magnitude(
            self.span.as_secs_f64() / unit.seconds_per_unit,
        ))
    }

    fn unit(self) -> AgeUnit {
        let elapsed_seconds = self.span.as_secs_f64();
        *AGE_UNIT_LADDER
            .iter()
            .find(|unit| elapsed_seconds < unit.upper_bound_seconds)
            .unwrap_or(&AGE_UNIT_LADDER[AGE_UNIT_LADDER.len() - 1])
    }
}

impl fmt::Display for RelativeAge {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.into_human_readable_time().fmt(formatter)
    }
}

impl fmt::Display for HumanReadableTime {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (magnitude, unit) = match self {
            Self::Seconds(magnitude) => (magnitude, "seconds"),
            Self::Minutes(magnitude) => (magnitude, "minutes"),
            Self::Hours(magnitude) => (magnitude, "hours"),
            Self::Days(magnitude) => (magnitude, "days"),
            Self::Weeks(magnitude) => (magnitude, "weeks"),
            Self::Months(magnitude) => (magnitude, "months"),
            Self::Years(magnitude) => (magnitude, "years"),
        };
        write!(formatter, "{:.2} {unit}", magnitude.0.float())
    }
}

fn quantized_magnitude(value: f64) -> HumanReadableMagnitude {
    let rendered = format!("{value:.2}");
    let quantized: f64 = rendered
        .parse()
        .expect("fixed two-decimal rendering always parses as f64");
    HumanReadableMagnitude(
        Decimal::try_from(quantized)
            .expect("a finite Duration produces a finite two-decimal magnitude"),
    )
}

#[derive(Clone, Copy)]
struct AgeUnit {
    unit: HumanTimeUnit,
    seconds_per_unit: f64,
    upper_bound_seconds: f64,
}

#[derive(Clone, Copy)]
enum HumanTimeUnit {
    Seconds,
    Minutes,
    Hours,
    Days,
    Weeks,
    Months,
    Years,
}

impl HumanTimeUnit {
    fn into_human_readable_time(self, magnitude: HumanReadableMagnitude) -> HumanReadableTime {
        match self {
            Self::Seconds => HumanReadableTime::Seconds(magnitude),
            Self::Minutes => HumanReadableTime::Minutes(magnitude),
            Self::Hours => HumanReadableTime::Hours(magnitude),
            Self::Days => HumanReadableTime::Days(magnitude),
            Self::Weeks => HumanReadableTime::Weeks(magnitude),
            Self::Months => HumanReadableTime::Months(magnitude),
            Self::Years => HumanReadableTime::Years(magnitude),
        }
    }
}

const SECONDS_PER_MINUTE: f64 = 60.0;
const SECONDS_PER_HOUR: f64 = 60.0 * SECONDS_PER_MINUTE;
const SECONDS_PER_DAY: f64 = 24.0 * SECONDS_PER_HOUR;
const SECONDS_PER_WEEK: f64 = 7.0 * SECONDS_PER_DAY;
const SECONDS_PER_MONTH: f64 = 2_629_746.0;
const SECONDS_PER_YEAR: f64 = 31_556_952.0;

const AGE_UNIT_LADDER: [AgeUnit; 7] = [
    AgeUnit {
        unit: HumanTimeUnit::Seconds,
        seconds_per_unit: 1.0,
        upper_bound_seconds: 3.0 * SECONDS_PER_MINUTE,
    },
    AgeUnit {
        unit: HumanTimeUnit::Minutes,
        seconds_per_unit: SECONDS_PER_MINUTE,
        upper_bound_seconds: 90.0 * SECONDS_PER_MINUTE,
    },
    AgeUnit {
        unit: HumanTimeUnit::Hours,
        seconds_per_unit: SECONDS_PER_HOUR,
        upper_bound_seconds: 36.0 * SECONDS_PER_HOUR,
    },
    AgeUnit {
        unit: HumanTimeUnit::Days,
        seconds_per_unit: SECONDS_PER_DAY,
        upper_bound_seconds: 14.0 * SECONDS_PER_DAY,
    },
    AgeUnit {
        unit: HumanTimeUnit::Weeks,
        seconds_per_unit: SECONDS_PER_WEEK,
        upper_bound_seconds: 60.0 * SECONDS_PER_DAY,
    },
    AgeUnit {
        unit: HumanTimeUnit::Months,
        seconds_per_unit: SECONDS_PER_MONTH,
        upper_bound_seconds: 24.0 * SECONDS_PER_MONTH,
    },
    AgeUnit {
        unit: HumanTimeUnit::Years,
        seconds_per_unit: SECONDS_PER_YEAR,
        upper_bound_seconds: f64::INFINITY,
    },
];
