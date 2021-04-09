
use std::fmt;

use sdl2::rect::Point;
use serde::{Deserializer, Serializer, de::{self, Visitor, SeqAccess, MapAccess}, ser::SerializeStruct};

pub fn serialize<S>(point: &Point, serializer: S) -> Result<S::Ok, S::Error>
where S: Serializer {
    let mut state = serializer.serialize_struct("Point", 2)?;
    state.serialize_field("x", &point.x)?;
    state.serialize_field("y", &point.y)?;
    state.end()
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<Point, D::Error>
where D: Deserializer<'de> {
    deserializer.deserialize_i64(PointVisitor)
}

pub struct PointVisitor;

#[derive(Deserialize)]
#[serde(field_identifier, rename_all = "lowercase")]    
enum Field { X, Y }

impl<'de> Visitor<'de> for PointVisitor {
    type Value = Point;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a valid Point with a raw that has x and y as i32")
    }

    fn visit_seq<V>(self, mut seq: V) -> Result<Point, V::Error>
    where
        V: SeqAccess<'de>,
    {
        let x = seq.next_element()?
            .ok_or_else(|| de::Error::invalid_length(0, &self))?;
        let y = seq.next_element()?
            .ok_or_else(|| de::Error::invalid_length(1, &self))?;
        Ok(Point::new(x, y))
    }

    fn visit_map<V>(self, mut map: V) -> Result<Point, V::Error>
    where
        V: MapAccess<'de>,
    {
        let mut x = None;
        let mut y = None;
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
            }
        }
        let x = x.ok_or_else(|| de::Error::missing_field("x"))?;
        let y = y.ok_or_else(|| de::Error::missing_field("y"))?;
        Ok(Point::new(x, y))
    }


}


