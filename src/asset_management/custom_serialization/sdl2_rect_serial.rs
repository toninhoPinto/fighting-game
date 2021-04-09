use std::fmt;

use sdl2::rect::Rect;
use serde::{Deserializer, Serializer, de::{self, Visitor, SeqAccess, MapAccess}, ser::SerializeStruct};

pub fn serialize<S>(rect: &Rect, serializer: S) -> Result<S::Ok, S::Error>
where S: Serializer {
    let mut state = serializer.serialize_struct("Rect", 4)?;
    state.serialize_field("x", &rect.x())?;
    state.serialize_field("y", &rect.y())?;
    state.serialize_field("h", &rect.height())?;
    state.serialize_field("w", &rect.width())?;
    state.end()
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<Rect, D::Error>
where D: Deserializer<'de> {
    deserializer.deserialize_i64(RectVisitor)
}

pub struct RectVisitor;
#[derive(Deserialize)]
#[serde(field_identifier, rename_all = "lowercase")]
enum Field { X, Y, H, W }

impl<'de> Visitor<'de> for RectVisitor {
    type Value = Rect;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a valid Rect with a raw that has x and y as i32 and width/height as u32")
    }

    fn visit_seq<V>(self, mut seq: V) -> Result<Rect, V::Error>
    where
        V: SeqAccess<'de>,
    {
        let x = seq.next_element()?
            .ok_or_else(|| de::Error::invalid_length(0, &self))?;
        let y = seq.next_element()?
            .ok_or_else(|| de::Error::invalid_length(1, &self))?;
        let h = seq.next_element()?
            .ok_or_else(|| de::Error::invalid_length(2, &self))?;
        let w = seq.next_element()?
            .ok_or_else(|| de::Error::invalid_length(3, &self))?;
        Ok(Rect::new(x, y, w, h))
    }

    fn visit_map<V>(self, mut map: V) -> Result<Rect, V::Error>
    where
        V: MapAccess<'de>,
    {
        let mut x = None;
        let mut y = None;
        let mut h: Option<u32> = None;
        let mut w: Option<u32> = None;
        while let Some(key) = map.next_key()? {
            match key {
                Field::X => {
                    if x.is_some() {
                        return Err(de::Error::duplicate_field("x"));
                    }
                    x = Some(map.next_value()?);
                }
                Field::Y => {
                    if y.is_some() {
                        return Err(de::Error::duplicate_field("y"));
                    }
                    y = Some(map.next_value()?);
                }
                Field::H => {
                    if h.is_some() {
                        return Err(de::Error::duplicate_field("h"));
                    }
                    h = Some(map.next_value()?);
                }
                Field::W => {
                    if w.is_some() {
                        return Err(de::Error::duplicate_field("w"));
                    }
                    w = Some(map.next_value()?);
                }
            }
        }
        let x = x.ok_or_else(|| de::Error::missing_field("x"))?;
        let y = y.ok_or_else(|| de::Error::missing_field("y"))?;
        let h = h.ok_or_else(|| de::Error::missing_field("h"))?;
        let w = w.ok_or_else(|| de::Error::missing_field("w"))?;
        Ok(Rect::new(x, y, w, h))
    }
}

