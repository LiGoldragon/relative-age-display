# Upgrades

## 0.4.0 — typed Datomic presentation

This is a breaking replacement of the encoded presentation contract. Consumers
must depend on `datomic` and use `Datomic`, `Text<T>`, and `TextEdge` for the
typed edge. `HumanReadableTime` now textualizes as canonical finite-decimal
payloads such as `Minutes.10.0` and `Days.3.2`; the prior representation and
its codec traits are unavailable.

The declaration source is `relative-age-display.ethos`; its committed
`src/generated.rs` projection is regenerated with
`RustEmitter::datomic_library()`. The package now requires Rust 1.89 for its
generator witness.
