//! Behavioral proof of the relative-age unit ladder and typed presentation.

use std::{fs, time::Duration};

use ethos_zero::{Actualizing, File, Generating, Potential};
use relative_age_display::{HumanReadableTime, RelativeAge};

fn rendered_from_seconds(seconds: f64) -> String {
    RelativeAge::from_duration(Duration::from_secs_f64(seconds)).to_string()
}

#[test]
fn zero_span_reads_as_zero_seconds() {
    assert_eq!(rendered_from_seconds(0.0), "0.00 seconds");
}

#[test]
fn sub_three_minute_spans_render_in_seconds_with_two_decimals() {
    assert_eq!(rendered_from_seconds(42.17), "42.17 seconds");
    // Just below the three-minute promotion boundary stays in seconds.
    assert_eq!(rendered_from_seconds(179.99), "179.99 seconds");
}

#[test]
fn three_minutes_exactly_promotes_to_minutes() {
    // The seconds rung's upper bound is exclusive at 180s, so 180s is minutes.
    assert_eq!(rendered_from_seconds(180.0), "3.00 minutes");
}

#[test]
fn minute_spans_render_in_minutes_with_two_decimals() {
    assert_eq!(rendered_from_seconds(17.80 * 60.0), "17.80 minutes");
    // Just below the ninety-minute promotion boundary stays in minutes.
    assert_eq!(rendered_from_seconds(89.99 * 60.0), "89.99 minutes");
}

#[test]
fn ninety_minutes_promotes_to_hours() {
    assert_eq!(rendered_from_seconds(90.0 * 60.0), "1.50 hours");
}

#[test]
fn hour_spans_render_in_hours_until_thirty_six_hours() {
    assert_eq!(rendered_from_seconds(5.25 * 3600.0), "5.25 hours");
    assert_eq!(rendered_from_seconds(35.99 * 3600.0), "35.99 hours");
}

#[test]
fn thirty_six_hours_promotes_to_days() {
    assert_eq!(rendered_from_seconds(36.0 * 3600.0), "1.50 days");
}

#[test]
fn day_spans_render_in_days_with_two_decimals() {
    assert_eq!(rendered_from_seconds(3.42 * 86_400.0), "3.42 days");
    assert_eq!(rendered_from_seconds(13.99 * 86_400.0), "13.99 days");
}

#[test]
fn fourteen_days_promotes_to_weeks() {
    assert_eq!(rendered_from_seconds(14.0 * 86_400.0), "2.00 weeks");
}

#[test]
fn week_spans_render_in_weeks_until_sixty_days() {
    // 30 days is well inside the weeks rung: 30 / 7 = 4.285... -> 4.29 weeks.
    assert_eq!(rendered_from_seconds(30.0 * 86_400.0), "4.29 weeks");
    // Just below the sixty-day promotion boundary stays in weeks.
    assert_eq!(rendered_from_seconds(59.99 * 86_400.0), "8.57 weeks");
}

#[test]
fn sixty_days_promotes_to_months() {
    // 60 days / mean-Gregorian-month (30.436875 days) = 1.971... -> 1.97 months.
    assert_eq!(rendered_from_seconds(60.0 * 86_400.0), "1.97 months");
}

#[test]
fn month_spans_render_in_months_until_two_years() {
    // Mean month is 2_629_746 s; six of them render as exactly 6.00 months.
    assert_eq!(rendered_from_seconds(6.0 * 2_629_746.0), "6.00 months");
    assert_eq!(rendered_from_seconds(23.99 * 2_629_746.0), "23.99 months");
}

#[test]
fn two_years_promotes_to_years() {
    assert_eq!(rendered_from_seconds(24.0 * 2_629_746.0), "2.00 years");
}

#[test]
fn large_spans_render_in_years_open_ended() {
    // Mean year is 31_556_952 s.
    assert_eq!(rendered_from_seconds(2.50 * 31_556_952.0), "2.50 years");
    assert_eq!(rendered_from_seconds(140.0 * 31_556_952.0), "140.00 years");
}

#[test]
fn nanosecond_construction_matches_duration_construction() {
    let three_and_a_half_days_nanoseconds = (3.5 * 86_400.0 * 1_000_000_000.0) as u64;
    assert_eq!(
        RelativeAge::from_nanoseconds(three_and_a_half_days_nanoseconds).to_string(),
        "3.50 days"
    );
}

#[test]
fn elapsed_between_saturates_reversed_instants_to_zero() {
    let earlier = Duration::from_secs(1_000);
    let later = Duration::from_secs(400);
    assert_eq!(
        RelativeAge::elapsed_between(earlier, later).to_string(),
        "0.00 seconds"
    );
}

#[test]
fn elapsed_between_measures_forward_spans() {
    let earlier = Duration::from_secs(100);
    let later = Duration::from_secs(100 + 42);
    assert_eq!(
        RelativeAge::elapsed_between(earlier, later).to_string(),
        "42.00 seconds"
    );
}

#[test]
fn committed_projection_is_the_real_generation_of_the_authored_ethos() {
    // Compared, never overwritten: a stale committed file fails here.
    let source = fs::read_to_string("ethos/relative_age_display.ethos").expect("authored map");
    let file = Potential::<File>::from(source)
        .actualize()
        .expect("map incorporates");
    let generated = file.generate().expect("Rust generation");
    assert_eq!(
        generated,
        fs::read_to_string("src/generated/relative_age_display.rs").expect("committed projection")
    );
}

#[test]
fn a_human_readable_time_returns_from_its_archive_equal() {
    for seconds in [0.0, 42.17, 3.2 * 86_400.0, 400.0 * 86_400.0] {
        let time =
            RelativeAge::from_duration(Duration::from_secs_f64(seconds)).into_human_readable_time();
        let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&time).expect("archives");
        let back =
            rkyv::from_bytes::<HumanReadableTime, rkyv::rancor::Error>(&bytes).expect("restores");
        assert_eq!(back, time);
    }
}

/// Layout mirrors that archive a bare float in the position of the magnitude, one per unit: the
/// bytes a peer could hand over. The archive of the real type is the same layout.
#[derive(rkyv::Archive, rkyv::Serialize)]
enum MirrorTime {
    Seconds(MirrorMagnitude),
    Minutes(MirrorMagnitude),
    Hours(MirrorMagnitude),
    Days(MirrorMagnitude),
    Weeks(MirrorMagnitude),
    Months(MirrorMagnitude),
    Years(MirrorMagnitude),
}
#[derive(rkyv::Archive, rkyv::Serialize)]
struct MirrorMagnitude(MirrorDecimal);
#[derive(rkyv::Archive, rkyv::Serialize)]
struct MirrorDecimal(f64);

fn mirrors(value: f64) -> Vec<MirrorTime> {
    let magnitude = || MirrorMagnitude(MirrorDecimal(value));
    vec![
        MirrorTime::Seconds(magnitude()),
        MirrorTime::Minutes(magnitude()),
        MirrorTime::Hours(magnitude()),
        MirrorTime::Days(magnitude()),
        MirrorTime::Weeks(magnitude()),
        MirrorTime::Months(magnitude()),
        MirrorTime::Years(magnitude()),
    ]
}

/// Reading an archive into a native value goes through the codec's `Decimal::try_from` (datom-codec
/// 0.33.1), nested in the magnitude and the time: a non-finite float is refused in every unit.
#[test]
fn a_non_finite_magnitude_in_an_archive_is_refused_in_every_unit() {
    for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        for mirror in mirrors(value) {
            let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&mirror).expect("archives");
            assert!(
                rkyv::from_bytes::<HumanReadableTime, rkyv::rancor::Error>(&bytes).is_err(),
                "{value}"
            );
        }
    }
}

#[test]
fn a_finite_archive_keeps_the_layout_of_the_mirror_and_a_negative_zero_reads_as_zero() {
    let time = RelativeAge::from_duration(Duration::from_secs_f64(3.2 * 86_400.0))
        .into_human_readable_time();
    let real = rkyv::to_bytes::<rkyv::rancor::Error>(&time).expect("archives");
    let mirror = rkyv::to_bytes::<rkyv::rancor::Error>(&mirrors(3.2).remove(3)).expect("archives");
    assert_eq!(real.as_slice(), mirror.as_slice());

    let negative =
        rkyv::to_bytes::<rkyv::rancor::Error>(&mirrors(-0.0).remove(0)).expect("archives");
    let zero = RelativeAge::from_duration(Duration::ZERO).into_human_readable_time();
    let back = rkyv::from_bytes::<HumanReadableTime, rkyv::rancor::Error>(&negative)
        .expect("a negative zero restores as zero");
    assert_eq!(back, zero);
    // equality treats the two zeros alike; the bits show which one was read
    let HumanReadableTime::Seconds(magnitude) = back else {
        panic!("the unit is kept");
    };
    assert_eq!(
        datom_codec::Decimating::float(&magnitude.0).to_bits(),
        0.0f64.to_bits()
    );
}
