use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::Visitor;
use time::{OffsetDateTime, format_description::well_known::Rfc3339};
use std::fmt::Formatter;

#[derive(sqlx::Type)]
pub struct Timestampz(pub OffsetDateTime);

impl Serialize for Timestampz {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        serializer.collect_str(&self.0.format(&Rfc3339).unwrap())
    }
}

impl<'de> Deserialize<'de> for Timestampz {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        struct StrVisitor;

        impl Visitor<'_> for StrVisitor {
            type Value = Timestampz;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.pad("expected string")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error
            {
                OffsetDateTime::parse(v, &Rfc3339)
                    .map(Timestampz)
                    .map_err(E::custom)
            }
        }

        deserializer.deserialize_str(StrVisitor)
    }
}