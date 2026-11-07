# Upgrades

## 0.6.0 — the 0.33 codec stack; archive added; Rust API change

What breaks:

- Rust: `HumanReadableMagnitude` is now a tuple struct `HumanReadableMagnitude(datom_codec::Decimal)`
  (it was `protos::Decimal`, an alias). `Decimal` is `datom_codec::Decimal`; read its float with
  `datom_codec::Decimating::float`. The generated module moved to `src/generated/relative_age_display.rs`
  and the authored map to `ethos/relative_age_display.ethos` (written in current Ethos syntax: the header is
  `Library`).
- Built on datom-codec `5cd6219` (0.33.1), protos `3c7ade3` and generator ethos-zero `5a5daca`. The old `Datomic`,
  `Potential`/`IncorporationBudget` and `Textualizable`-on-codec API is replaced: text goes through
  `Datomizable`, `protos::Protosizable` and `Textualizable`, and admission through
  `datom_codec::Potential` with a `Budget`. The types derive `Datomizable` and `Composing` only under the
  `datom` feature (which brings protos), and that feature is in `default`, so a default build keeps the
  typed text capability 0.5.0 had; `--no-default-features` builds the rendering and the archive only.
- New: the types archive with rkyv. There was no archive before, so there is nothing to read from an
  earlier release. The archive does not re-check finiteness on reading: a hand-built archive holding a
  NaN float is admitted by `rkyv::from_bytes` (recorded in `tests/rendering.rs`, not fixed here).

Unchanged, shown by witnesses against 0.5.0 (`tests/datom_text.rs`, provenance in
`tests/previous_release_provenance.md`): the rendered display of seventeen spans across the ladder; the datom
text written for those spans; which of twenty-six texts are admitted or refused, including the refusal of
`NaN`, `inf`, integers without a point, exponents, and unknown unit names; and the refusal of non-finite
floats. These are sampled witnesses, not a proof for every text.

Text change, reviewed as one: negative zero. 0.5.0 wrote `Days.-0.0` for a magnitude of negative zero and read
`Days.-0.0` back to it; 0.6.0 writes and reads `Days.0.0`, because `datom_codec::Decimal` has no negative zero
(`Decimal::try_from` normalizes it away; datom-codec `src/decimal.rs`). In 0.5.0 the two zeros were distinct
values (`protos::Decimal` compared bits), so this is a change of text and of equality for that one value, not
an equality-preserving rewrite. No `RelativeAge` produced from a `Duration` is negative, so the rendering never
produces it; only a hand-built or archived magnitude can.

Archive: the types now archive with rkyv because the generator emits those derives unconditionally. There was
no archive in 0.5.0, so nothing historic is read. Reading an archive into a native `HumanReadableTime` goes
through datom-codec 0.33.1's checked `Decimal` reconstruction: a non-finite magnitude is refused in every unit
and a negative zero reads as zero (`tests/rendering.rs`, archives built from layout mirrors). The borrowed
archived form is still only structurally validated. This release therefore needs datom-codec 0.33.1 or later:
0.33.0 admits a non-finite float from an archive.

Known declared consumers: none found in any Cargo.toml under /git/github.com/LiGoldragon or /home/li/primary
(see the packet; that covers declared dependencies only).

Deploy: repin to this release with datom-codec `5cd6219` and protos `3c7ade3`, wrap or unwrap the magnitude as above.

## 0.5.0 — current typed Datom codec

This breaking codec migration replaces the retired `datomic` crate with
`datom-codec`. Consumers must use `Potential<T>`,
`Actualizable::actualize(IncorporationBudget)`, and `Textualizable` at the
typed text boundary. `HumanReadableTime` retains canonical finite-decimal
texts such as `Minutes.10.0` and `Days.3.2`.

The declaration source is `relative-age-display.ethos`; its committed
`src/generated.rs` projection is regenerated with current Ethos `Generate`.
The package requires Rust 1.89 for its generator witness.
