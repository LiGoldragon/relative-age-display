# relative-age-display

Human-readable relative-age display: a shared library that renders an elapsed
time span for a human reader as its single most-significant unit, valued with
exactly two decimal places, using one uniform format for every unit.

```rust
use std::time::Duration;
use relative_age_display::RelativeAge;

assert_eq!(RelativeAge::from_duration(Duration::from_secs_f64(42.17)).to_string(), "42.17 seconds");
assert_eq!(RelativeAge::from_duration(Duration::from_secs(90 * 60)).to_string(), "1.50 hours");
assert_eq!(RelativeAge::from_nanoseconds(3_628_800_000_000_000).to_string(), "6.00 weeks");
```

## The unit ladder

The display climbs one ordered ladder of units. Each rung carries the span until
the elapsed time reaches that rung's upper bound, then the next coarser unit
takes over. Seconds are shown only up to three minutes; there are no special
cases, that cutoff is simply the seconds rung's upper bound.

| Unit    | Shown while elapsed time is below |
|---------|-----------------------------------|
| seconds | 3 minutes                         |
| minutes | 90 minutes                        |
| hours   | 36 hours                          |
| days    | 14 days                           |
| weeks   | 60 days                           |
| months  | 24 months (2 years)               |
| years   | open-ended                        |

Months and years use the mean Gregorian calendar length (365.2425 days per
year). The rendered value is always the single most-significant unit with two
decimal places (`3.42 days`), and unit labels are always plural for uniformity.

## Construction

- `RelativeAge::from_duration(span)` — from a `std::time::Duration`.
- `RelativeAge::from_nanoseconds(nanoseconds)` — from a raw elapsed nanosecond
  count, the shape stored record ages arrive in.
- `RelativeAge::elapsed_between(earlier, later)` — the span between two instants
  on one clock, saturating to zero when `later` precedes `earlier`.
