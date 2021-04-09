use std::fmt;
use parry2d::math::Point as aabbPoint;

use parry2d::bounding_volume::AABB;
use serde::{Deserializer, Serializer, de::{self, Visitor, SeqAccess, MapAccess}, ser::SerializeStruct};

pub fn serialize<S>(aabb: &AABB, serializer: S) -> Result<S::Ok, S::Error>
where S: Serializer {
    let mut state = serializer.serialize_struct("AABB", 4)?;
    state.serialize_field("min_x", &aabb.mins.x)?;
    state.serialize_field("min_y", &aabb.mins.y)?;
    state.serialize_field("max_x", &aabb.maxs.x)?;
    state.serialize_field("max_y", &aabb.maxs.y)?;
    state.end()
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<AABB, D::Error>
where D: Deserializer<'de> {
    deserializer.deserialize_i64(AABBVisitor)
}

pub struct AABBVisitor;
#[derive(Deserialize)]
#[serde(field_identifier, rename_all = "lowercase")]    
enum Field { MinX, MinY, MaxX, MaxY}

impl<'de> Visitor<'de> for AABBVisitor {
    type Value = AABB;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a valid SDL2 Scancode integer")
    }


    fn visit_seq<V>(self, mut seq: V) -> Result<AABB, V::Error>
    where
        V: SeqAccess<'de>,
    {
        let min_x = seq.next_element()?
            .ok_or_else(|| de::Error::invalid_length(0, &self))?;
        let min_y = seq.next_element()?
            .ok_or_else(|| de::Error::invalid_length(1, &self))?;
        let max_x = seq.next_element()?
            .ok_or_else(|| de::Error::invalid_length(2, &self))?;
        let max_y = seq.next_element()?
            .ok_or_else(|| de::Error::invalid_length(3, &self))?;
        Ok(AABB::new(aabbPoint::new(min_x, min_y),aabbPoint::new(max_x, max_y)))
    }

    fn visit_map<V>(self, mut map: V) -> Result<AABB, V::Error>
    where
        V: MapAccess<'de>,
    {
        let mut min_x = None;
        let mut min_y = None;
        let mut max_x = None;
        let mut max_y = None;
        while let Some(key) = map.next_key()? {
            match key {
                Field::MinX => {
                    if min_x.is_some() {
                        return Err(de::Error::duplicate_field("x"));
                    }
                    min_x = Some(map.next_value()?);
                }
                Field::MinY => {
                    if min_y.is_some() {
                        return Err(de::Error::duplicate_field("y"));
                    }
                    min_y = Some(map.next_value()?);
                }
                Field::MaxX => {
                    if max_x.is_some() {
                        return Err(de::Error::duplicate_field("y"));
                    }
                    max_x = Some(map.next_value()?);
                }
                Field::MaxY => {
                    if max_y.is_some() {
                        return Err(de::Error::duplicate_field("y"));
                    }
                    max_y = Some(map.next_value()?);
                }
            }
        }
        let min_x = min_x.ok_or_else(|| de::Error::missing_field("min_x"))?;
        let min_y = min_y.ok_or_else(|| de::Error::missing_field("min_y"))?;
        let max_x = max_x.ok_or_else(|| de::Error::missing_field("max_x"))?;
        let max_y = max_y.ok_or_else(|| de::Error::missing_field("max_y"))?;
        Ok(AABB::new(aabbPoint::new(min_x, min_y),aabbPoint::new(max_x, max_y)))
    }
}
