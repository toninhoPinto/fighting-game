use std::fmt::{self, Display};

use std::result::Result;
use std::str::FromStr;
use serde::{Deserialize, ser::{SerializeTupleVariant}};
use serde::ser::{Serialize, Serializer};
use serde::de::{Visitor, value, Deserializer, IntoDeserializer};
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum TranslatedInput
 {
    LightPunch,
    MediumPunch,
    HeavyPunch,
    LightKick,
    MediumKick,
    HeavyKick,
    Horizontal (i32),
    Vertical (i32),
}

impl TranslatedInput {
    pub fn init_dir_input_state() -> [(TranslatedInput, bool); 4]{
        let mut current_inputs_state: [(TranslatedInput, bool); 4] = [(TranslatedInput::LightPunch, false); 4];
        current_inputs_state[0] = (TranslatedInput::Horizontal(1), false);
        current_inputs_state[1] = (TranslatedInput::Horizontal(-1), false);
        current_inputs_state[2] = (TranslatedInput::Vertical(1), false);
        current_inputs_state[3] = (TranslatedInput::Vertical(-1), false);

        current_inputs_state
    }

    pub fn get_button_index(current_inputs_state: &mut [(TranslatedInput, bool); 4], input: TranslatedInput) -> Option<usize> {
        let mut return_index= None;
        for i in 0..4 {
            if current_inputs_state[i].0 == input{
                return_index = Some(i);
                break;
            }
        }
        return_index
    }

    pub fn is_directional_input(input: TranslatedInput) -> bool{
        match input {
            TranslatedInput::Horizontal(_h) => true,
            TranslatedInput::Vertical(_v) => true,
            _ => false
        }
    }

    pub fn is_currently_any_directional_input(current_inputs_state: &[(TranslatedInput, bool); 4]) -> bool {
        let mut return_index= false;
        for i in 0..4 {
            return_index |= current_inputs_state[i].1;
        }
        return_index
    }
    
}


impl Display for TranslatedInput
 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Serialize for TranslatedInput
 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            TranslatedInput
            ::LightPunch => serializer.serialize_unit_variant("LightPunch", 0, "LightPunch"),
            TranslatedInput
            ::MediumPunch => serializer.serialize_unit_variant("MediumPunch", 0, "MediumPunch"),
            TranslatedInput
            ::HeavyPunch => serializer.serialize_unit_variant("HeavyPunch", 0, "HeavyPunch"),
            TranslatedInput
            ::LightKick => serializer.serialize_unit_variant("LightKick", 0, "LightKick"),
            TranslatedInput
            ::MediumKick => serializer.serialize_unit_variant("MediumKick", 0, "MediumKick"),
            TranslatedInput
            ::HeavyKick => serializer.serialize_unit_variant("HeavyKick", 0, "HeavyKick"),
            TranslatedInput
            ::Vertical (ref v) => {
                let mut state =
                    serializer.serialize_tuple_variant("Vertical", 1, "Vertical", 1)?;
                state.serialize_field(v)?;
                state.end()
            },
            TranslatedInput
            ::Horizontal (ref h) => {
                let mut state =
                    serializer.serialize_tuple_variant("Horizontal", 1, "Horizontal", 1)?;
                state.serialize_field(h)?;
                state.end()
            }
        }
    }
}

impl FromStr for TranslatedInput
 {
    type Err = value::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}

impl<'de> Deserialize<'de> for TranslatedInput
 {
    fn deserialize<D>(deserializer: D) -> Result<TranslatedInput
    , D::Error>
        where D: Deserializer<'de>
    {
        struct FieldVisitor {
            min: usize,
        };

        impl<'de> Visitor<'de> for FieldVisitor {
            type Value = TranslatedInput
            ;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(formatter, "a string containing at least {} bytes", self.min)
            }

            fn visit_str<E>(self, value: &str) -> Result<TranslatedInput
            , E>
                where E: serde::de::Error
            {
                let kind = match value {
                    "LightPunch" => TranslatedInput
                    ::LightPunch,
                    "MediumPunch" => TranslatedInput
                    ::MediumPunch,
                    "HeavyPunch" => TranslatedInput
                    ::HeavyPunch,
                    "LightKick" => TranslatedInput
                    ::LightKick,
                    "MediumKick" => TranslatedInput
                    ::MediumKick,
                    "HeavyKick" => TranslatedInput
                    ::HeavyKick,
                    "Horizontal(1)" => TranslatedInput
                    ::Horizontal(1),
                    "Horizontal(-1)" => TranslatedInput
                    ::Horizontal(-1),
                    "Vertical(1)" => TranslatedInput
                    ::Vertical(1),
                    "Vertical(-1)" => TranslatedInput
                    ::Vertical(-1),
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