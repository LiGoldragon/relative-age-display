//! Human-readable relative-age presentation.
//!
//! A [`RelativeAge`] converts an elapsed time span into a closed
//! [`HumanReadableTime`] value. The value retains the selected unit and its
//! deterministic two-decimal magnitude, so consumers can encode the age as
//! structured NOTA rather than flattening it into display text.
//!
//! The unit ladder climbs from seconds through years. Seconds are shown only up
//! to three minutes of elapsed time; past that the presentation promotes to
//! minutes, then hours, days, weeks, months, and finally years. The ladder is a
//! single ordered table with no special cases: the seconds-only-up-to-three-
//! minutes rule is just the first row's upper bound.

use std::{fmt, time::Duration};

use nota::{Block, Delimiter, NotaBlock, NotaDecode, NotaDecodeError, NotaEncode};

/// One two-decimal magnitude carried by a [`HumanReadableTime`] unit.
///
/// The stored value is quantized through the same two-decimal formatting rule
/// the prior text presentation used. Its NOTA form omits a fractional suffix
/// when the rounded value is whole (`10`) and parenthesizes a fractional
/// literal (`(3.2)`) so an enclosing unit variant has one unambiguous payload.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HumanReadableMagnitude(f64);

impl HumanReadableMagnitude {
    /// Quantize a converted unit value to the deterministic two-decimal
    /// magnitude shown to humans.
    pub fn from_unit_value(value: f64) -> Self {
        let rendered = format!("{value:.2}");
        Self(
            rendered
                .parse()
                .expect("two-decimal f64 rendering always parses as f64"),
        )
    }

    /// The quantized numeric magnitude.
    pub const fn value(self) -> f64 {
        self.0
    }

    /// The fixed-two-decimal text used by the legacy display projection.
    fn display_text(self) -> String {
        format!("{:.2}", self.0)
    }

    /// The canonical numeric NOTA payload, parenthesized when fractional so an
    /// enclosing unit variant remains a two-level application.
    fn nota_payload(self) -> String {
        let fixed = self.display_text();
        let numeric = fixed.trim_end_matches('0').trim_end_matches('.');
        if numeric.contains('.') {
            format!("({numeric})")
        } else {
            numeric.to_owned()
        }
    }

    /// The scalar inside the optional fractional-parenthesis payload.
    fn from_nota_payload(block: &Block) -> Result<&Block, NotaDecodeError> {
        match block.as_delimited(Delimiter::Parenthesis) {
            Some(_) => Ok(&NotaBlock::new(block).expect_children(
                Delimiter::Parenthesis,
                "HumanReadableMagnitude",
                1,
            )?[0]),
            None => Ok(block),
        }
    }
}

impl NotaEncode for HumanReadableMagnitude {
    fn to_nota(&self) -> String {
        self.nota_payload()
    }
}

impl NotaDecode for HumanReadableMagnitude {
    fn from_nota_block(block: &Block) -> Result<Self, NotaDecodeError> {
        let value = f64::from_nota_block(Self::from_nota_payload(block)?)?;
        if !value.is_finite() || value.is_sign_negative() {
            return Err(NotaDecodeError::InvalidValue {
                type_name: "HumanReadableMagnitude",
                value: value.to_string(),
                reason: "expected a finite non-negative magnitude".to_owned(),
            });
        }
        Ok(Self::from_unit_value(value))
    }
}

/// A closed elapsed-time presentation whose unit remains data.
///
/// Each variant carries a [`HumanReadableMagnitude`] in the unit selected by
/// the relative-age ladder. Its NOTA representation is positional and typed:
/// `Minutes.10` and `Days.(3.2)` are values, not prose.
#[derive(Debug, Clone, Copy, PartialEq, NotaDecode, NotaEncode)]
pub enum HumanReadableTime {
    Seconds(HumanReadableMagnitude),
    Minutes(HumanReadableMagnitude),
    Hours(HumanReadableMagnitude),
    Days(HumanReadableMagnitude),
    Weeks(HumanReadableMagnitude),
    Months(HumanReadableMagnitude),
    Years(HumanReadableMagnitude),
}

impl HumanReadableTime {
    /// Render the compatibility text projection without losing the typed
    /// unit/magnitude representation used by coordination output.
    fn display_text(self) -> String {
        match self {
            Self::Seconds(magnitude) => format!("{} seconds", magnitude.display_text()),
            Self::Minutes(magnitude) => format!("{} minutes", magnitude.display_text()),
            Self::Hours(magnitude) => format!("{} hours", magnitude.display_text()),
            Self::Days(magnitude) => format!("{} days", magnitude.display_text()),
            Self::Weeks(magnitude) => format!("{} weeks", magnitude.display_text()),
            Self::Months(magnitude) => format!("{} months", magnitude.display_text()),
            Self::Years(magnitude) => format!("{} years", magnitude.display_text()),
        }
    }
}

impl fmt::Display for HumanReadableTime {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.display_text())
    }
}

/// An elapsed time span that can project its most-significant unit for a human
/// reader.
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

    /// Convert this elapsed span into its closed typed human-time value.
    pub fn into_human_readable_time(self) -> HumanReadableTime {
        let unit = self.unit();
        unit.unit
            .into_human_readable_time(HumanReadableMagnitude::from_unit_value(
                self.span.as_secs_f64() / unit.seconds_per_unit,
            ))
    }

    /// The unit this span presents in, chosen as the first ladder rung whose
    /// upper bound the span falls below.
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

/// One rung of the relative-age unit ladder: the closed typed unit, how many
/// seconds one of its units spans, and the elapsed span (exclusive, in
/// seconds) at or above which the presentation promotes to the next coarser
/// unit.
#[derive(Clone, Copy)]
struct AgeUnit {
    unit: HumanTimeUnit,
    seconds_per_unit: f64,
    upper_bound_seconds: f64,
}

/// The internal ladder unit that builds the public typed presentation.
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
    /// Attach one quantized magnitude to this unit's public variant.
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
/// The mean Gregorian month: 365.2425 days / 12.
const SECONDS_PER_MONTH: f64 = 2_629_746.0;
/// The mean Gregorian year: 365.2425 days.
const SECONDS_PER_YEAR: f64 = 31_556_952.0;

/// The ordered unit ladder. Each rung is used while the elapsed span is below
/// its `upper_bound_seconds`; the final rung (years) is the open-ended catch-all.
///
/// Chosen breakpoints, uniform "single most-significant unit" presentation:
/// - seconds below 3 minutes (the psyche's explicit seconds cutoff),
/// - minutes below 90 minutes,
/// - hours below 36 hours,
/// - days below 14 days (2 weeks),
/// - weeks below 60 days (~8.6 weeks),
/// - months below 24 months (2 years),
/// - years thereafter.
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
