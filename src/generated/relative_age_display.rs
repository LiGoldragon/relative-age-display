#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    clippy::type_complexity
)]
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct HumanReadableMagnitude(pub datom_codec::Decimal);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum HumanReadableTime {
    Seconds(HumanReadableMagnitude),
    Minutes(HumanReadableMagnitude),
    Hours(HumanReadableMagnitude),
    Days(HumanReadableMagnitude),
    Weeks(HumanReadableMagnitude),
    Months(HumanReadableMagnitude),
    Years(HumanReadableMagnitude),
}
