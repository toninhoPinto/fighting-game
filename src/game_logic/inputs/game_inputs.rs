use std::fmt::{self, Display};

use std::result::Result;
use std::str::FromStr;
use serde::{Deserialize, ser::{self, SerializeTupleVariant}};
use serde::ser::{Serialize, Serializer};
use serde::de::{Visitor, value, Deserializer, IntoDeserializer};

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

pub fn input_state() -> [(GameInputs, bool); 10]{
    let mut current_inputs_state: [(GameInputs, bool); 10] = [(GameInputs::LightPunch, false); 10];
    current_inputs_state[0] = (GameInputs::LightPunch, false);
    current_inputs_state[1] = (GameInputs::MediumPunch, false);
    current_inputs_state[2] = (GameInputs::HeavyPunch, false);
    current_inputs_state[3] = (GameInputs::LightKick, false);
    current_inputs_state[4] = (GameInputs::MediumKick, false);
    current_inputs_state[5] = (GameInputs::HeavyKick, false);
    current_inputs_state[6] = (GameInputs::Horizontal(1), false);
    current_inputs_state[7] = (GameInputs::Vertical(1), false);
    current_inputs_state[8] = (GameInputs::Horizontal(-1), false);
    current_inputs_state[9] = (GameInputs::Vertical(-1), false);

    current_inputs_state
}

fn handle_buttons(current_inputs_state: &mut [(GameInputs, bool); 10], input: GameInputs, is_pressed: bool) {
    for i in 0..8 {
        if current_inputs_state[i].0 == input {
            current_inputs_state[i] = (current_inputs_state[i].0, is_pressed);
            break;
        }
    }
}

fn handle_joystick(current_inputs_state: &mut [(GameInputs, bool); 10], axis_idx: u8, input: i32) {
    let is_pressed;
    if input == 0 {
        is_pressed = false;
    } else {
        is_pressed = true;
    }
    if axis_idx == 0 {
        current_inputs_state[6] = (GameInputs::Horizontal(1), is_pressed);
    } else {
        current_inputs_state[7] = (GameInputs::Vertical(1), is_pressed);
    }
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