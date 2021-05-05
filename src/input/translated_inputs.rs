use std::fmt::{self, Display};

use serde::de::{value, Deserializer, IntoDeserializer, Visitor};
use serde::ser::{Serialize, Serializer};
use serde::{ser::SerializeTupleVariant, Deserialize};
use std::result::Result;
use std::str::FromStr;
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum TranslatedInput {
    Punch,
    Kick,
    Jump,
    Block,
    Horizontal(i32),
    Vertical(i32),
}

impl Display for TranslatedInput {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Serialize for TranslatedInput {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            TranslatedInput::Punch => {
                serializer.serialize_unit_variant("Punch", 0, "Punch")
            },
            TranslatedInput::Kick => {
                serializer.serialize_unit_variant("Kick", 0, "Kick")
            },
            TranslatedInput::Jump => {
                serializer.serialize_unit_variant("Jump", 0, "Jump")
            },
            TranslatedInput::Block => {
                serializer.serialize_unit_variant("Block", 0, "Block")
            },
            TranslatedInput::Vertical(ref v) => {
                let mut state = serializer.serialize_tuple_variant("Vertical", 1, "Vertical", 1)?;
                state.serialize_field(v)?;
                state.end()
            }
            TranslatedInput::Horizontal(ref h) => {
                let mut state =
                    serializer.serialize_tuple_variant("Horizontal", 1, "Horizontal", 1)?;
                state.serialize_field(h)?;
                state.end()
            }
        }
    }
}

impl FromStr for TranslatedInput {
    type Err = value::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}

impl<'de> Deserialize<'de> for TranslatedInput {
    fn deserialize<D>(deserializer: D) -> Result<TranslatedInput, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FieldVisitor {
            min: usize,
        }

        impl<'de> Visitor<'de> for FieldVisitor {
            type Value = TranslatedInput;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(formatter, "a string containing at least {} bytes", self.min)
            }

            fn visit_str<E>(self, value: &str) -> Result<TranslatedInput, E>
            where
                E: serde::de::Error,
            {
                let kind = match value {
                    "Punch" => TranslatedInput::Punch,
                    "Kick" => TranslatedInput::Kick,
                    "Jump" => TranslatedInput::Jump,
                    "Block" => TranslatedInput::Block,
                    "Horizontal(1)" => TranslatedInput::Horizontal(1),
                    "Horizontal(-1)" => TranslatedInput::Horizontal(-1),
                    "Vertical(1)" => TranslatedInput::Vertical(1),
                    "Vertical(-1)" => TranslatedInput::Vertical(-1),
                    s => {
                        return Err(serde::de::Error::invalid_value(
                            serde::de::Unexpected::Str(s),
                            &self,
                        ));
                    }
                };
                Ok(kind)
            }
        }
        deserializer.deserialize_str(FieldVisitor { min: 4 })
    }
}
