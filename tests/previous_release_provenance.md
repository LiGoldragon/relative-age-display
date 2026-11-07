# Provenance of the 0.5.0 texts in `datom_text.rs`

The expected strings were produced by the 0.5.0 release itself: origin main
99d184541ed8b71daeafd44f025cdaafe7ecef82 (datom-codec 41a3c073d5c5, protos 2d999f173334, ethos-zero
bcf728bb), exported with `git archive` into a scratch directory, with this example added and run with
`cargo run --example witness`:

```rust
use datom_codec::{Actualizable, IncorporationBudget, Potential, Textualizable};
use protos::Decimal;
use relative_age_display::{HumanReadableTime, RelativeAge};
use std::time::Duration;

fn main() {
    // rendering and text across the ladder
    for seconds in [0.0, 0.004, 0.005, 1.0, 42.17, 179.99, 180.0, 3600.0, 5400.0, 86_400.0, 3.2 * 86_400.0, 3.428 * 86_400.0, 6.99 * 86_400.0, 7.0 * 86_400.0, 45.0 * 86_400.0, 400.0 * 86_400.0, 1.0e9 * 86_400.0] {
        let age = RelativeAge::from_duration(Duration::from_secs_f64(seconds));
        println!("AGE {seconds} => {} | {}", age, age.into_human_readable_time().textualize());
    }
    // admission of text
    for text in ["Days.3.2", "Days.3", "Minutes.10.0", "Days.-1", "Days.-0.0", "Days.0", "Days.1e3", "Days.1.5e2", "Days.NaN", "Days.inf", "Days.-inf", "Days.", "Days.1.2.3", "Days.3.20", "Days. 3", "days.3", "Fortnights.3", "Seconds.1e400", "Days.+3", "Days.3.", "Days..5", "Days.0.1", "Years.123456789012345678901234567890.5", "Seconds.0.1000000000000000055511151231257827", "Weeks.17", "Hours.1_0"] {
        let result = Potential::<HumanReadableTime>::from(text).actualize(IncorporationBudget::try_from(1024).unwrap());
        match result {
            Ok(value) => println!("PARSE {text:?} => Ok {}", value.textualize()),
            Err(error) => println!("PARSE {text:?} => Err"),
        }
    }
    // decimal admission from floats
    for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY, -0.0, 0.0, -1.5, 1.0e300, f64::MIN_POSITIVE, 5e-324] {
        match Decimal::try_from(value) {
            Ok(decimal) => println!("DECIMAL {value:?} => Ok {}", HumanReadableTime::Days(decimal).textualize()),
            Err(_) => println!("DECIMAL {value:?} => Err"),
        }
    }
    // error text for one refusal
    let error = Potential::<HumanReadableTime>::from("Days.NaN").actualize(IncorporationBudget::try_from(1024).unwrap()).err();
    println!("ERRKIND {:?}", error.map(|e| format!("{e:?}").chars().take(80).collect::<String>()));
}
```

Output:

```
17 |             Err(error) => println!("PARSE {text:?} => Err"),
AGE 0 => 0.00 seconds | Seconds.0.0
AGE 0.004 => 0.00 seconds | Seconds.0.0
AGE 0.005 => 0.01 seconds | Seconds.0.01
AGE 1 => 1.00 seconds | Seconds.1.0
AGE 42.17 => 42.17 seconds | Seconds.42.17
AGE 179.99 => 179.99 seconds | Seconds.179.99
AGE 180 => 3.00 minutes | Minutes.3.0
AGE 3600 => 60.00 minutes | Minutes.60.0
AGE 5400 => 1.50 hours | Hours.1.5
AGE 86400 => 24.00 hours | Hours.24.0
AGE 276480 => 3.20 days | Days.3.2
AGE 296179.2 => 3.43 days | Days.3.43
AGE 603936 => 6.99 days | Days.6.99
AGE 604800 => 7.00 days | Days.7.0
AGE 3888000 => 6.43 weeks | Weeks.6.43
AGE 34560000 => 13.14 months | Months.13.14
AGE 86400000000000 => 2737907.01 years | Years.2737907.01
PARSE "Days.3.2" => Ok Days.3.2
PARSE "Days.3" => Err
PARSE "Minutes.10.0" => Ok Minutes.10.0
PARSE "Days.-1" => Err
PARSE "Days.-0.0" => Ok Days.-0.0
PARSE "Days.0" => Err
PARSE "Days.1e3" => Err
PARSE "Days.1.5e2" => Err
PARSE "Days.NaN" => Err
PARSE "Days.inf" => Err
PARSE "Days.-inf" => Err
PARSE "Days." => Err
PARSE "Days.1.2.3" => Err
PARSE "Days.3.20" => Ok Days.3.2
PARSE "Days. 3" => Err
PARSE "days.3" => Err
PARSE "Fortnights.3" => Err
PARSE "Seconds.1e400" => Err
PARSE "Days.+3" => Err
PARSE "Days.3." => Err
PARSE "Days..5" => Err
PARSE "Days.0.1" => Ok Days.0.1
PARSE "Years.123456789012345678901234567890.5" => Ok Years.123456789012345680000000000000.0
PARSE "Seconds.0.1000000000000000055511151231257827" => Ok Seconds.0.1
PARSE "Weeks.17" => Err
PARSE "Hours.1_0" => Err
DECIMAL NaN => Err
DECIMAL inf => Err
DECIMAL -inf => Err
DECIMAL -0.0 => Ok Days.-0.0
DECIMAL 0.0 => Ok Days.0.0
DECIMAL -1.5 => Ok Days.-1.5
DECIMAL 1e300 => Ok Days.1000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000.0
DECIMAL 2.2250738585072014e-308 => Ok Days.0.000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000022250738585072014
DECIMAL 5e-324 => Ok Days.0.000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000005
ERRKIND Some("Corporate(Locus { path: [1], extent: Extent(5, 8) }, Value(Opaque(\"NaN\")))")
```

The same program ported to the 0.6.0 API gave identical AGE, PARSE and DECIMAL lines except two:
`PARSE "Days.-0.0"` and `DECIMAL -0.0`, which 0.5.0 wrote as `Days.-0.0` and 0.6.0 writes as `Days.0.0`.
The error value of a refusal differs in shape (`Corporate(Locus ...)` before, `Error { layer: Composition ... }`
now); only accepted versus refused is compared. Limits: one program, the listed inputs, not every text.
