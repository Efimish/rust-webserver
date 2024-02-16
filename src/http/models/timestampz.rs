use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::Visitor;
use time::{OffsetDateTime, format_description::well_known::Rfc3339};
use std::fmt::Formatter;

#[derive(sqlx::Type)]
pub struct Timestampz(pub OffsetDateTime);

/// We can use this instead of `Option<Timestampz>`\
/// Because `From<Option<OffsetDateTime>> for Option<Timestampz>` can not be implemented
#[derive(sqlx::Type)]
pub struct TimestampzOption(pub Option<Timestampz>);

impl From<OffsetDateTime> for Timestampz {
    fn from(value: OffsetDateTime) -> Self {
        Self(value)
    }
}

impl From<Option<OffsetDateTime>> for TimestampzOption {
    fn from(value: Option<OffsetDateTime>) -> Self {
        match value {
            Some(value) => Self(Some(Timestampz(value))),
            None => Self(None),
        }
    }
}

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

impl Serialize for TimestampzOption {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        match &self.0 {
            Some(timestampz) => serializer.serialize_some(timestampz),
            None => serializer.serialize_none(),
        }
    }
}

impl<'de> Deserialize<'de> for TimestampzOption {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        struct StrVisitor;

        impl<'v> Visitor<'v> for StrVisitor {
            type Value = TimestampzOption;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.pad("expected string")
            }

            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: serde::de::Error
            {
                Ok(TimestampzOption(None))
            }

            fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: Deserializer<'v>
            {
                let timestampz = Timestampz::deserialize(deserializer)?;
                Ok(TimestampzOption(Some(timestampz)))
            }
        }

        deserializer.deserialize_str(StrVisitor)
    }
}