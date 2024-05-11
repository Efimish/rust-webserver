//! Custom DateTime type

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::Visitor;
use time::{OffsetDateTime, format_description::well_known::Rfc3339};
use std::fmt::Formatter;

/// `OffsetDateTime` provides RFC-3339 (ISO-8601 subset) serialization, but the default
/// `serde::Serialize` implementation produces array of integers, which is great for binary
/// serialization, but infeasible to consume when returned from an API, and certainly
/// not human-readable.
///
/// With this wrapper type, we override this to provide the serialization format we want.
#[derive(sqlx::Type)]
pub struct Timestamptz(pub OffsetDateTime);

/// This has to be used instead of `Option<Timestamptz>`\
/// Because `From<Option<OffsetDateTime>> for Option<Timestamptz>` can not be implemented
#[derive(sqlx::Type)]
pub struct TimestamptzOption(pub Option<Timestamptz>);

impl From<OffsetDateTime> for Timestamptz {
    fn from(value: OffsetDateTime) -> Self {
        Self(value)
    }
}

impl From<Option<OffsetDateTime>> for TimestamptzOption {
    fn from(value: Option<OffsetDateTime>) -> Self {
        Self(value.map(Timestamptz))
    }
}

impl Serialize for Timestamptz {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        serializer.collect_str(&self.0.format(&Rfc3339).unwrap())
    }
}

impl<'de> Deserialize<'de> for Timestamptz {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        struct StrVisitor;

        impl Visitor<'_> for StrVisitor {
            type Value = Timestamptz;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.pad("expected string")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error
            {
                OffsetDateTime::parse(v, &Rfc3339)
                    .map(Timestamptz)
                    .map_err(E::custom)
            }
        }

        deserializer.deserialize_str(StrVisitor)
    }
}

impl Serialize for TimestamptzOption {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        match &self.0 {
            Some(timestamptz) => serializer.serialize_some(timestamptz),
            None => serializer.serialize_none(),
        }
    }
}

impl<'de> Deserialize<'de> for TimestamptzOption {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        struct StrVisitor;

        impl<'v> Visitor<'v> for StrVisitor {
            type Value = TimestamptzOption;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.pad("expected string")
            }

            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: serde::de::Error
            {
                Ok(TimestamptzOption(None))
            }

            fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: Deserializer<'v>
            {
                let timestamptz = Timestamptz::deserialize(deserializer)?;
                Ok(TimestamptzOption(Some(timestamptz)))
            }
        }

        deserializer.deserialize_str(StrVisitor)
    }
}