# Upgrades

## 0.6.0 — the 0.33 codec stack; archive added; Rust API change

What breaks:

- Rust: `HumanReadableMagnitude` is now a tuple struct `HumanReadableMagnitude(datom_codec::Decimal)`
  (it was `protos::Decimal`, an alias). `Decimal` is `datom_codec::Decimal`; read its float with
  `datom_codec::Decimating::float`. The generated module moved to `src/generated/relative_age_display.rs`
  and the authored map to `ethos/relative_age_display.ethos` (written in current Ethos syntax: the header is
  `Library`).
- Built on datom-codec `eabead4`, protos `3c7ade3` and generator ethos-zero `5a5daca`. The old `Datomic`,
  `Potential`/`IncorporationBudget` and `Textualizable`-on-codec API is replaced: text goes through
  `Datomizable`, `protos::Protosizable` and `Textualizable`, and admission through
  `datom_codec::Potential` with a `Budget`. The types derive `Datomizable` and `Composing` only under the
  new `datom` feature (which brings protos); default features build the rendering and the archive only.
- New: the types archive with rkyv. There was no archive before, so there is nothing to read from an
  earlier release. The archive does not re-check finiteness on reading: a hand-built archive holding a
  NaN float is admitted by `rkyv::from_bytes` (recorded in `tests/rendering.rs`, not fixed here).

Unchanged, shown by witnesses against 0.5.0 (`tests/datom_text.rs`, provenance in
`tests/previous_release_provenance.md`): the rendered display of seventeen spans across the ladder; the datom
text written for those spans; which of twenty-six texts are admitted or refused, including the refusal of
`NaN`, `inf`, integers without a point, exponents, and unknown unit names; and the refusal of non-finite
floats. One difference: negative zero is written `Days.0.0` where 0.5.0 wrote `Days.-0.0`. These are sampled
witnesses, not a proof for every text.

Known declared consumers: none found in any Cargo.toml under /git/github.com/LiGoldragon or /home/li/primary
(see the packet; that covers declared dependencies only).

Deploy: repin to this release with datom-codec `eabead4` and protos `3c7ade3`, wrap or unwrap the magnitude as above.

## 0.5.0 — current typed Datom codec

This breaking codec migration replaces the retired `datomic` crate with
`datom-codec`. Consumers must use `Potential<T>`,
`Actualizable::actualize(IncorporationBudget)`, and `Textualizable` at the
typed text boundary. `HumanReadableTime` retains canonical finite-decimal
texts such as `Minutes.10.0` and `Days.3.2`.

The declaration source is `relative-age-display.ethos`; its committed
`src/generated.rs` projection is regenerated with current Ethos `Generate`.
The package requires Rust 1.89 for its generator witness.
