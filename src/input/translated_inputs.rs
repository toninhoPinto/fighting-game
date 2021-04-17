use std::fmt::{self, Display};

use serde::de::{value, Deserializer, IntoDeserializer, Visitor};
use serde::ser::{Serialize, Serializer};
use serde::{ser::SerializeTupleVariant, Deserialize};
use std::result::Result;
use std::str::FromStr;
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum TranslatedInput {
    LightPunch,
    MediumPunch,
    HeavyPunch,
    LightKick,
    MediumKick,
    HeavyKick,
    Horizontal(i32),
    Vertical(i32),
}

impl TranslatedInput {
    pub fn init_dir_input_state() -> [(TranslatedInput, bool); 4] {
        [
            (TranslatedInput::Horizontal(1), false),
            (TranslatedInput::Horizontal(-1), false),
            (TranslatedInput::Vertical(1), false),
            (TranslatedInput::Vertical(-1), false),
        ]
    }

    pub fn init_button_input_state() -> [(TranslatedInput, bool); 6] {
        [
            (TranslatedInput::LightPunch, false),
            (TranslatedInput::MediumPunch, false),
            (TranslatedInput::HeavyPunch, false),
            (TranslatedInput::LightKick, false),
            (TranslatedInput::MediumKick, false),
            (TranslatedInput::HeavyKick, false),
        ]
    }

    pub fn is_directional_input(input: TranslatedInput) -> bool {
        matches!(
            input,
            TranslatedInput::Horizontal(_) | TranslatedInput::Vertical(_)
        )
    }

    pub fn is_currently_any_directional_input(
        current_inputs_state: &[(TranslatedInput, bool); 4],
    ) -> bool {
        current_inputs_state[0].1
            || current_inputs_state[1].1
            || current_inputs_state[2].1
            || current_inputs_state[3].1
    }
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
            TranslatedInput::LightPunch => {
                serializer.serialize_unit_variant("LightPunch", 0, "LightPunch")
            }
            TranslatedInput::MediumPunch => {
                serializer.serialize_unit_variant("MediumPunch", 0, "MediumPunch")
            }
            TranslatedInput::HeavyPunch => {
                serializer.serialize_unit_variant("HeavyPunch", 0, "HeavyPunch")
            }
            TranslatedInput::LightKick => {
                serializer.serialize_unit_variant("LightKick", 0, "LightKick")
            }
            TranslatedInput::MediumKick => {
                serializer.serialize_unit_variant("MediumKick", 0, "MediumKick")
            }
            TranslatedInput::HeavyKick => {
                serializer.serialize_unit_variant("HeavyKick", 0, "HeavyKick")
            }
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
                    "LightPunch" => TranslatedInput::LightPunch,
                    "MediumPunch" => TranslatedInput::MediumPunch,
                    "HeavyPunch" => TranslatedInput::HeavyPunch,
                    "LightKick" => TranslatedInput::LightKick,
                    "MediumKick" => TranslatedInput::MediumKick,
                    "HeavyKick" => TranslatedInput::HeavyKick,
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
