use std::fmt::{self, Display};

use std::result::Result;
use std::str::FromStr;
use serde::{Deserialize, ser::{self, SerializeTupleVariant}};
use serde::ser::{Serialize, Serializer};
use serde::de::{Error, Visitor, value, Deserializer, IntoDeserializer};

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum GameInputs {
    LightPunch,
    MediumPunch,
    HeavyPunch,
    LightKick,
    MediumKick,
    HeavyKick,
    Horizontal (i32),
    Vertical (i32),
    FWD,
    FwdDOWN,
    FwdUP,
    BACK,
    BackDOWN,
    BackUP,
    UP,
    DOWN
}

impl Display for GameInputs {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Serialize for GameInputs {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            GameInputs::LightPunch => serializer.serialize_unit_variant("LightPunch", 0, "LightPunch"),
            GameInputs::MediumPunch => serializer.serialize_unit_variant("MediumPunch", 0, "MediumPunch"),
            GameInputs::HeavyPunch => serializer.serialize_unit_variant("HeavyPunch", 0, "HeavyPunch"),
            GameInputs::LightKick => serializer.serialize_unit_variant("LightKick", 0, "LightKick"),
            GameInputs::MediumKick => serializer.serialize_unit_variant("MediumKick", 0, "MediumKick"),
            GameInputs::HeavyKick => serializer.serialize_unit_variant("HeavyKick", 0, "HeavyKick"),
            GameInputs::Vertical (ref v) => {
                let mut state =
                    serializer.serialize_tuple_variant("Vertical", 1, "Vertical", 1)?;
                state.serialize_field(v)?;
                state.end()
            },
            GameInputs::Horizontal (ref h) => {
                let mut state =
                    serializer.serialize_tuple_variant("Horizontal", 1, "Horizontal", 1)?;
                state.serialize_field(h)?;
                state.end()
            },
            _ => { Err(ser::Error::custom("path contains invalid UTF-8 characters")) }
        }
    }
}

impl FromStr for GameInputs {
    type Err = value::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}

impl<'de> Deserialize<'de> for GameInputs {
    fn deserialize<D>(deserializer: D) -> Result<GameInputs, D::Error>
        where D: Deserializer<'de>
    {
        struct FieldVisitor {
            min: usize,
        };

        impl<'de> Visitor<'de> for FieldVisitor {
            type Value = GameInputs;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(formatter, "a string containing at least {} bytes", self.min)
            }

            fn visit_str<E>(self, value: &str) -> Result<GameInputs, E>
                where E: serde::de::Error
            {
                let kind = match value {
                    "LightPunch" => GameInputs::LightPunch,
                    "MediumPunch" => GameInputs::MediumPunch,
                    "HeavyPunch" => GameInputs::HeavyPunch,
                    "LightKick" => GameInputs::LightKick,
                    "MediumKick" => GameInputs::MediumKick,
                    "HeavyKick" => GameInputs::HeavyKick,
                    "Horizontal(1)" => GameInputs::Horizontal(1),
                    "Horizontal(-1)" => GameInputs::Horizontal(-1),
                    "Vertical(1)" => GameInputs::Vertical(1),
                    "Vertical(-1)" => GameInputs::Vertical(-1),
                    s => {
                        return Err(serde::de::Error::invalid_value(serde::de::Unexpected::Str(s),
                                                                   &self));
                    }
                };
                Ok(kind)
            }
        }
        deserializer.deserialize_str(FieldVisitor { min: 4 })
    }
}