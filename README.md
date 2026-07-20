# relative-age-display

`relative-age-display` converts an elapsed span into a closed typed
`HumanReadableTime` value. It is the shared duration-presentation layer for
status surfaces: a consumer keeps the exact source duration separately, then
encodes this projection as NOTA rather than flattening it into prose.

```rust
use std::time::Duration;
use nota::NotaEncode;
use relative_age_display::RelativeAge;

let minutes = RelativeAge::from_duration(Duration::from_secs(10 * 60))
    .into_human_readable_time();
let days = RelativeAge::from_duration(Duration::from_secs_f64(3.2 * 86_400.0))
    .into_human_readable_time();

assert_eq!(minutes.to_nota(), "Minutes.10");
assert_eq!(days.to_nota(), "Days.(3.2)");
```

`HumanReadableTime` has closed `Seconds`, `Minutes`, `Hours`, `Days`, `Weeks`,
`Months`, and `Years` variants. Its magnitude is deterministically quantized to
two decimal places. Whole magnitudes are bare NOTA values; fractional
magnitudes are parenthesized so an enclosing variant remains structurally
unambiguous.

`RelativeAge` still implements `Display` for compatibility with text-only
consumers, using that same typed conversion. New coordination output should use
`into_human_readable_time()` and encode the resulting value directly.

## Presentation codec family

This crate is an immutable Nota 0.9 presentation-family release. Its sole codec
dependency is pinned to Nota 0.9 revision
`89dc3c85a9ff96d4e4d53accfd867df672cae5a8`; it has no `nota-human` dependency
and does not import legacy Nota 0.5 wire traits. Consumers must keep this
human-presentation codec family separate from legacy coordination-wire values.

## The unit ladder

The presentation climbs one ordered ladder of units. Each rung carries the
span until the elapsed time reaches that rung's upper bound, then the next
coarser unit takes over. Seconds are shown only up to three minutes; there are
no special cases, that cutoff is simply the seconds rung's upper bound.

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
year). The underlying `RelativeAge` conversion quantizes each magnitude to two
decimal places, and the typed NOTA value preserves its unit.

## Construction

- `RelativeAge::from_duration(span)` — from a `std::time::Duration`.
- `RelativeAge::from_nanoseconds(nanoseconds)` — from a raw elapsed nanosecond
  count, the shape stored record ages arrive in.
- `RelativeAge::elapsed_between(earlier, later)` — the span between two
  instants on one clock, saturating to zero when `later` precedes `earlier`.
- `RelativeAge::into_human_readable_time()` — the structured unit-and-magnitude
  projection for NOTA output.
