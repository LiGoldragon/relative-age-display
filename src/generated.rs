#![allow(dead_code)]
#![allow(clippy::redundant_closure)]
pub type HumanReadableMagnitude = protos::Decimal;
#[derive(Clone, Debug, PartialEq)]
pub enum HumanReadableTime {
    Seconds(HumanReadableMagnitude),
    Minutes(HumanReadableMagnitude),
    Hours(HumanReadableMagnitude),
    Days(HumanReadableMagnitude),
    Weeks(HumanReadableMagnitude),
    Months(HumanReadableMagnitude),
    Years(HumanReadableMagnitude),
}
impl datom_codec::Datomic for HumanReadableTime {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let v = datom_codec::Sited::variant(site)?;
        match v.name {
            "Seconds" => {
                std::result::Result::Ok(Self::Seconds(datom_codec::Carrying::body(v)?))
            }
            "Minutes" => {
                std::result::Result::Ok(Self::Minutes(datom_codec::Carrying::body(v)?))
            }
            "Hours" => {
                std::result::Result::Ok(Self::Hours(datom_codec::Carrying::body(v)?))
            }
            "Days" => {
                std::result::Result::Ok(Self::Days(datom_codec::Carrying::body(v)?))
            }
            "Weeks" => {
                std::result::Result::Ok(Self::Weeks(datom_codec::Carrying::body(v)?))
            }
            "Months" => {
                std::result::Result::Ok(Self::Months(datom_codec::Carrying::body(v)?))
            }
            "Years" => {
                std::result::Result::Ok(Self::Years(datom_codec::Carrying::body(v)?))
            }
            _ => {
                std::result::Result::Err(
                    datom_codec::Headed::reject(
                        &v,
                        datom_codec::Problem::UnknownVariant(
                            protos::Word::try_from(v.name).expect("variant name"),
                        ),
                    ),
                )
            }
        }
    }
}
impl protos::Conceivable<datom_codec::Datom> for HumanReadableTime {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                match self {
                    Self::Seconds(p0) => {
                        datom_codec::Datom::Variant(
                            protos::Symbol::try_from("Seconds").expect("static variant"),
                            std::boxed::Box::new(
                                protos::Conceivable::conceive(p0)
                                    .expect("infallible datom ascent")
                                    .1,
                            ),
                        )
                    }
                    Self::Minutes(p0) => {
                        datom_codec::Datom::Variant(
                            protos::Symbol::try_from("Minutes").expect("static variant"),
                            std::boxed::Box::new(
                                protos::Conceivable::conceive(p0)
                                    .expect("infallible datom ascent")
                                    .1,
                            ),
                        )
                    }
                    Self::Hours(p0) => {
                        datom_codec::Datom::Variant(
                            protos::Symbol::try_from("Hours").expect("static variant"),
                            std::boxed::Box::new(
                                protos::Conceivable::conceive(p0)
                                    .expect("infallible datom ascent")
                                    .1,
                            ),
                        )
                    }
                    Self::Days(p0) => {
                        datom_codec::Datom::Variant(
                            protos::Symbol::try_from("Days").expect("static variant"),
                            std::boxed::Box::new(
                                protos::Conceivable::conceive(p0)
                                    .expect("infallible datom ascent")
                                    .1,
                            ),
                        )
                    }
                    Self::Weeks(p0) => {
                        datom_codec::Datom::Variant(
                            protos::Symbol::try_from("Weeks").expect("static variant"),
                            std::boxed::Box::new(
                                protos::Conceivable::conceive(p0)
                                    .expect("infallible datom ascent")
                                    .1,
                            ),
                        )
                    }
                    Self::Months(p0) => {
                        datom_codec::Datom::Variant(
                            protos::Symbol::try_from("Months").expect("static variant"),
                            std::boxed::Box::new(
                                protos::Conceivable::conceive(p0)
                                    .expect("infallible datom ascent")
                                    .1,
                            ),
                        )
                    }
                    Self::Years(p0) => {
                        datom_codec::Datom::Variant(
                            protos::Symbol::try_from("Years").expect("static variant"),
                            std::boxed::Box::new(
                                protos::Conceivable::conceive(p0)
                                    .expect("infallible datom ascent")
                                    .1,
                            ),
                        )
                    }
                },
            ),
        )
    }
}
