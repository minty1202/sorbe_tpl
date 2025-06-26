use crate::{
    shared::Map,
    value::{Number, Value},
};
use serde::de::{
    self, Deserialize, Deserializer, Error as DeError, IntoDeserializer, MapAccess, Visitor,
};
use std::fmt;

impl Value {
    fn as_i64(&self) -> Option<i64> {
        match self {
            Value::Number(Number::Int(i)) => Some(*i),
            Value::Number(Number::UInt(u)) if *u <= i64::MAX as u64 => Some(*u as i64),
            Value::Number(Number::Float(f))
                if f.fract() == 0.0 && *f >= i64::MIN as f64 && *f <= i64::MAX as f64 =>
            {
                Some(*f as i64)
            }
            _ => None,
        }
    }

    fn as_u64(&self) -> Option<u64> {
        match self {
            Value::Number(Number::UInt(u)) => Some(*u),
            Value::Number(Number::Int(i)) if *i >= 0 => Some(*i as u64),
            Value::Number(Number::Float(f))
                if f.fract() == 0.0 && *f >= 0.0 && *f <= u64::MAX as f64 =>
            {
                Some(*f as u64)
            }
            _ => None,
        }
    }

    fn as_f64(&self) -> Option<f64> {
        match self {
            Value::Number(Number::Float(f)) => Some(*f),
            Value::Number(Number::Int(i)) => Some(*i as f64),
            Value::Number(Number::UInt(u)) => Some(*u as f64),
            _ => None,
        }
    }
}

impl<'de> Deserialize<'de> for Value {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(ValueVisitor)
    }
}

struct ValueVisitor;

impl<'de> Visitor<'de> for ValueVisitor {
    type Value = Value;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("any valid sorbe_tpl value")
    }

    fn visit_bool<E>(self, value: bool) -> Result<Value, E> {
        Ok(Value::Bool(value))
    }

    fn visit_i64<E>(self, value: i64) -> Result<Value, E> {
        Ok(Value::Number(Number::Int(value)))
    }

    fn visit_u64<E>(self, value: u64) -> Result<Value, E> {
        Ok(Value::Number(Number::UInt(value)))
    }

    fn visit_f64<E>(self, value: f64) -> Result<Value, E> {
        Ok(Value::Number(Number::Float(value)))
    }

    fn visit_str<E>(self, value: &str) -> Result<Value, E> {
        Ok(Value::String(value.to_string()))
    }

    fn visit_string<E>(self, value: String) -> Result<Value, E> {
        Ok(Value::String(value))
    }

    fn visit_unit<E>(self) -> Result<Value, E> {
        Ok(Value::Null)
    }

    fn visit_map<M>(self, mut map: M) -> Result<Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut dict = Map::new();
        while let Some((key, value)) = map.next_entry::<String, Value>()? {
            dict.insert(key, value);
        }
        Ok(Value::Dict(dict))
    }
}

#[derive(Debug, Clone)]
pub struct DeserializeError(String);

impl fmt::Display for DeserializeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl std::error::Error for DeserializeError {}

impl DeError for DeserializeError {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        DeserializeError(msg.to_string())
    }
}

impl<'de> Deserializer<'de> for Value {
    type Error = DeserializeError;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Value::Null => visitor.visit_unit(),
            Value::Bool(b) => visitor.visit_bool(b),
            Value::Number(Number::Int(i)) => visitor.visit_i64(i),
            Value::Number(Number::UInt(u)) => visitor.visit_u64(u),
            Value::Number(Number::Float(f)) => visitor.visit_f64(f),
            Value::String(s) => visitor.visit_string(s),
            Value::Dict(dict) => visitor.visit_map(DictAccess::new(dict)),
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Value::Bool(b) => visitor.visit_bool(b),
            _ => Err(DeserializeError::custom("expected bool")),
        }
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if let Some(i) = self.as_i64() {
            if i >= i8::MIN as i64 && i <= i8::MAX as i64 {
                visitor.visit_i8(i as i8)
            } else {
                Err(DeserializeError::custom("expected i8"))
            }
        } else {
            Err(DeserializeError::custom("expected i8"))
        }
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if let Some(i) = self.as_i64() {
            if i >= i16::MIN as i64 && i <= i16::MAX as i64 {
                return visitor.visit_i16(i as i16);
            }
        }
        Err(DeserializeError::custom("expected i16"))
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if let Some(i) = self.as_i64() {
            if i >= i32::MIN as i64 && i <= i32::MAX as i64 {
                return visitor.visit_i32(i as i32);
            }
        }
        Err(DeserializeError::custom("expected i32"))
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if let Some(i) = self.as_i64() {
            return visitor.visit_i64(i);
        }
        Err(DeserializeError::custom("expected i64"))
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if let Some(u) = self.as_u64() {
            if u <= u8::MAX as u64 {
                return visitor.visit_u8(u as u8);
            }
        }
        Err(DeserializeError::custom("expected u8"))
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if let Some(u) = self.as_u64() {
            if u <= u16::MAX as u64 {
                return visitor.visit_u16(u as u16);
            }
        }
        Err(DeserializeError::custom("expected u16"))
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if let Some(u) = self.as_u64() {
            if u <= u32::MAX as u64 {
                return visitor.visit_u32(u as u32);
            }
        }
        Err(DeserializeError::custom("expected u32"))
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if let Some(u) = self.as_u64() {
            return visitor.visit_u64(u);
        }
        Err(DeserializeError::custom("expected u64"))
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if let Some(f) = self.as_f64() {
            if f.is_finite() && f >= f32::MIN as f64 && f <= f32::MAX as f64 {
                return visitor.visit_f32(f as f32);
            }
        }
        Err(DeserializeError::custom("expected f32"))
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if let Some(f) = self.as_f64() {
            return visitor.visit_f64(f);
        }
        Err(DeserializeError::custom("expected f64"))
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Value::String(s) => {
                let mut chars = s.chars();
                match (chars.next(), chars.next()) {
                    (Some(c), None) => visitor.visit_char(c),
                    _ => Err(DeserializeError::custom("expected single character")),
                }
            }
            _ => Err(DeserializeError::custom("expected char")),
        }
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Value::String(s) => visitor.visit_string(s),
            _ => Err(DeserializeError::custom("expected string")),
        }
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Value::String(s) => visitor.visit_byte_buf(s.into_bytes()),
            _ => Err(DeserializeError::custom("expected bytes")),
        }
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_bytes(visitor)
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Value::Null => visitor.visit_none(),
            _ => visitor.visit_some(self),
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Value::Null => visitor.visit_unit(),
            _ => Err(DeserializeError::custom("expected null")),
        }
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_unit(visitor)
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(DeserializeError::custom("sequences not supported"))
    }

    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Value::Dict(dict) => visitor.visit_map(DictAccess::new(dict)),
            _ => Err(DeserializeError::custom("expected map")),
        }
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Value::String(s) => visitor.visit_enum(s.into_deserializer()),
            _ => Err(DeserializeError::custom("expected enum")),
        }
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_unit()
    }
}

struct DictAccess {
    iter: std::vec::IntoIter<(String, Value)>,
    value: Option<Value>,
}

impl DictAccess {
    fn new(dict: Map<String, Value>) -> Self {
        DictAccess {
            iter: dict.into_iter().collect::<Vec<_>>().into_iter(),
            value: None,
        }
    }
}

impl<'de> MapAccess<'de> for DictAccess {
    type Error = DeserializeError;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: de::DeserializeSeed<'de>,
    {
        match self.iter.next() {
            Some((key, value)) => {
                self.value = Some(value);
                seed.deserialize(key.into_deserializer()).map(Some)
            }
            None => Ok(None),
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: de::DeserializeSeed<'de>,
    {
        match self.value.take() {
            Some(value) => seed.deserialize(value),
            None => Err(DeserializeError::custom("value is missing")),
        }
    }
}
