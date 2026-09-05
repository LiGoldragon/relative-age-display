# relative-age-display

`relative-age-display` projects an elapsed span onto its single most-significant
unit. The projection is a closed, typed Datom value. Text enters through
`datom_codec::Potential<T>`, is bounded by an `IncorporationBudget`, and
outbound data uses `Textualizable`.

```rust
use datom_codec::{Actualizable, IncorporationBudget, Potential, Textualizable};
use relative_age_display::{HumanReadableTime, RelativeAge};
use std::time::Duration;

let minutes = RelativeAge::from_duration(Duration::from_secs(10 * 60))
    .into_human_readable_time();
let text = minutes.textualize();

assert_eq!(text, "Minutes.10.0");
assert!(matches!(
    Potential::<HumanReadableTime>::from("Days.3.2")
        .actualize(IncorporationBudget::try_from(1024).expect("budget")),
    Ok(HumanReadableTime::Days(_))
));
```

`HumanReadableTime` has closed `Seconds`, `Minutes`, `Hours`, `Days`, `Weeks`,
`Months`, and `Years` variants. Each payload is a Protos decimal, so text and
Datom projection remain total for every value produced by `RelativeAge`.
The source-owned `relative-age-display.ethos` map owns those declarations;
`src/generated.rs` is its byte-exact current Ethos `Generate` output.

`RelativeAge` implements `Display` for text-only views. Use
`into_human_readable_time()` with the Datom edge whenever the selected unit
must remain data.

## The unit ladder

The presentation climbs one ordered ladder of units. Each rung carries the
span until the elapsed time reaches that rung's upper bound, then the next
coarser unit takes over. Seconds are shown only up to three minutes.

| Unit | Shown while elapsed time is below |
| --- | --- |
| seconds | 3 minutes |
| minutes | 90 minutes |
| hours | 36 hours |
| days | 14 days |
| weeks | 60 days |
| months | 24 months (2 years) |
| years | open-ended |

Months and years use the mean Gregorian calendar length (365.2425 days per
year). Each magnitude is deterministically quantized to two decimal places.

## Construction

- `RelativeAge::from_duration(span)` builds from a `std::time::Duration`.
- `RelativeAge::from_nanoseconds(nanoseconds)` builds from a raw elapsed count.
- `RelativeAge::elapsed_between(earlier, later)` saturates reversed instants to zero.
- `RelativeAge::into_human_readable_time()` returns the typed projection.
