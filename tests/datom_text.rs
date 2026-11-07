//! The datom text of a relative age. The expected strings were produced by the 0.5.0 release
//! itself (origin main 99d1845, datom-codec 41a3c073d5c5, protos 2d999f173334), see
//! `tests/previous_release_provenance.md`; the one difference found is recorded below.
use datom_codec::{Actualizing, Budget, Datomizable, Decimal, Potential};
use protos::{Protosizable, ReaderBudget, Textualizable};
use relative_age_display::{HumanReadableMagnitude, HumanReadableTime, RelativeAge};
use std::time::Duration;

fn budget() -> Budget {
    Budget {
        remaining: 1024,
        reader: ReaderBudget::try_from(1024).expect("positive budget"),
        depth: 0,
        maximum_depth: 1024,
    }
}

fn text(value: HumanReadableTime) -> String {
    value
        .datomize(vec![])
        .protosize()
        .textualize()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

#[test]
fn the_ladder_renders_and_writes_the_texts_of_0_5_0() {
    let expected: &[(f64, &str, &str)] = &[
        (0.0, "0.00 seconds", "Seconds.0.0"),
        (0.004, "0.00 seconds", "Seconds.0.0"),
        (0.005, "0.01 seconds", "Seconds.0.01"),
        (1.0, "1.00 seconds", "Seconds.1.0"),
        (42.17, "42.17 seconds", "Seconds.42.17"),
        (179.99, "179.99 seconds", "Seconds.179.99"),
        (180.0, "3.00 minutes", "Minutes.3.0"),
        (3600.0, "60.00 minutes", "Minutes.60.0"),
        (5400.0, "1.50 hours", "Hours.1.5"),
        (86400.0, "24.00 hours", "Hours.24.0"),
        (276480.0, "3.20 days", "Days.3.2"),
        (296179.2, "3.43 days", "Days.3.43"),
        (603936.0, "6.99 days", "Days.6.99"),
        (604800.0, "7.00 days", "Days.7.0"),
        (3888000.0, "6.43 weeks", "Weeks.6.43"),
        (34560000.0, "13.14 months", "Months.13.14"),
        (86400000000000.0, "2737907.01 years", "Years.2737907.01"),
    ];
    for (seconds, display, datom) in expected {
        let age = RelativeAge::from_duration(Duration::from_secs_f64(*seconds));
        assert_eq!(age.to_string(), *display, "{seconds}");
        assert_eq!(text(age.into_human_readable_time()), *datom, "{seconds}");
    }
}

/// Text in, as 0.5.0 admitted or refused it; `None` is a refusal. `Days.-0.0` is the one
/// difference: 0.5.0 kept the sign of negative zero (`Days.-0.0`), the current codec writes
/// `Days.0.0`; the value is equal either way.
#[test]
fn the_admission_of_text_matches_0_5_0() {
    let expected: &[(&str, Option<&str>)] = &[
        ("Days.3.2", Some("Days.3.2")),
        ("Days.3", None),
        ("Minutes.10.0", Some("Minutes.10.0")),
        ("Days.-1", None),
        ("Days.-0.0", Some("Days.0.0")),
        ("Days.0", None),
        ("Days.1e3", None),
        ("Days.1.5e2", None),
        ("Days.NaN", None),
        ("Days.inf", None),
        ("Days.-inf", None),
        ("Days.", None),
        ("Days.1.2.3", None),
        ("Days.3.20", Some("Days.3.2")),
        ("Days. 3", None),
        ("days.3", None),
        ("Fortnights.3", None),
        ("Seconds.1e400", None),
        ("Days.+3", None),
        ("Days.3.", None),
        ("Days..5", None),
        ("Days.0.1", Some("Days.0.1")),
        (
            "Years.123456789012345678901234567890.5",
            Some("Years.123456789012345680000000000000.0"),
        ),
        (
            "Seconds.0.1000000000000000055511151231257827",
            Some("Seconds.0.1"),
        ),
        ("Weeks.17", None),
        ("Hours.1_0", None),
    ];
    for (source, wanted) in expected {
        let admitted = Potential::<HumanReadableTime>::from(*source)
            .actualize(&mut budget())
            .ok()
            .map(text);
        assert_eq!(admitted.as_deref(), *wanted, "{source}");
    }
}

#[test]
fn non_finite_magnitudes_are_refused_and_finite_ones_keep_their_text() {
    let expected: &[(f64, Option<&str>)] = &[
        (f64::NAN, None),
        (f64::INFINITY, None),
        (f64::NEG_INFINITY, None),
        (-0.0, Some("Days.0.0")),
        (0.0, Some("Days.0.0")),
        (-1.5, Some("Days.-1.5")),
        (1e300, Some("<long>")),
        (f64::MIN_POSITIVE, Some("<long>")),
        (5e-324, Some("<long>")),
    ];
    for (value, wanted) in expected {
        let admitted = Decimal::try_from(*value)
            .ok()
            .map(|decimal| text(HumanReadableTime::Days(HumanReadableMagnitude(decimal))));
        match (admitted.as_deref(), wanted) {
            (Some(got), Some("<long>")) => assert!(got.starts_with("Days."), "{value}"),
            (got, want) => assert_eq!(got, *want, "{value}"),
        }
    }
}
