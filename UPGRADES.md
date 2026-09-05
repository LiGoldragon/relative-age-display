# Upgrades

## 0.5.0 — current typed Datom codec

This breaking codec migration replaces the retired `datomic` crate with
`datom-codec`. Consumers must use `Potential<T>`,
`Actualizable::actualize(IncorporationBudget)`, and `Textualizable` at the
typed text boundary. `HumanReadableTime` retains canonical finite-decimal
texts such as `Minutes.10.0` and `Days.3.2`.

The declaration source is `relative-age-display.ethos`; its committed
`src/generated.rs` projection is regenerated with current Ethos `Generate`.
The package requires Rust 1.89 for its generator witness.
