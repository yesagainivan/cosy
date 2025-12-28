// src/serde_support.rs
pub mod serializer;

use crate::CosynError;
use crate::value::{Value, ValueKind};
use indexmap::IndexMap;
use serde::de::{self, Error as DeError, MapAccess, SeqAccess, Visitor};
use serde::ser::{Error as SeError, SerializeMap};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
// Removed: use std::collections::HashMap;
use std::error::Error as StdError;
use std::fmt;

/// Deserialize any type that implements `Deserialize` from COSY text
pub fn from_str<'a, T>(input: &'a str) -> Result<T, CosynError>
where
    T: Deserialize<'a>,
{
    let value = crate::from_str(input)?;
    from_value(value)
}

/// Deserialize any type that implements `Deserialize` from a COSY `Value`
pub fn from_value<'a, T>(value: Value) -> Result<T, CosynError>
where
    T: Deserialize<'a>,
{
    T::deserialize(ValueDeserializer::new(value)).map_err(|e| {
        CosynError::Parse(crate::ParseError {
            message: e.to_string(),
            line: 0,
            column: 0,
        })
    })
}

/// Serialize any type that implements `Serialize` to COSY text
pub fn to_string<T>(value: &T) -> Result<String, SerializeError>
where
    T: Serialize,
{
    let cosy_value = value.serialize(ValueSerializer)?;
    Ok(crate::to_string(&cosy_value))
}

// ============================================================================
// ERROR TYPE
// ============================================================================

/// Error type for Serde serialization
#[derive(Debug)]
pub struct SerializeError(String);

impl fmt::Display for SerializeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Serialization error: {}", self.0)
    }
}

impl StdError for SerializeError {}

impl serde::ser::Error for SerializeError {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        SerializeError(msg.to_string())
    }
}

/// Error type for Serde deserialization
#[derive(Debug)]
pub struct DeserializeError(String);

impl fmt::Display for DeserializeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Deserialization error: {}", self.0)
    }
}

impl StdError for DeserializeError {}

impl serde::de::Error for DeserializeError {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        DeserializeError(msg.to_string())
    }
}

// ============================================================================
// DESERIALIZER IMPLEMENTATION
// ============================================================================

/// A deserializer for COSY `Value` types
pub struct ValueDeserializer {
    value: Value,
}

impl ValueDeserializer {
    fn new(value: Value) -> Self {
        ValueDeserializer { value }
    }
}

impl<'de> Deserializer<'de> for ValueDeserializer {
    type Error = DeserializeError;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value.kind {
            ValueKind::Null => visitor.visit_unit(),
            ValueKind::Bool(b) => visitor.visit_bool(b),
            ValueKind::Integer(i) => visitor.visit_i64(i),
            ValueKind::Float(f) => visitor.visit_f64(f),
            ValueKind::String(s) => visitor.visit_string(s),
            ValueKind::Array(arr) => visitor.visit_seq(SeqDeserializer::new(arr)),
            ValueKind::Object(obj) => visitor.visit_map(MapDeserializer::new(obj)),
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value.kind {
            ValueKind::Bool(b) => visitor.visit_bool(b),
            _ => Err(DeserializeError::custom("expected bool")),
        }
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value.kind {
            ValueKind::Integer(i) => visitor.visit_i64(i),
            _ => Err(DeserializeError::custom("expected integer")),
        }
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value.kind {
            ValueKind::Integer(i) => {
                if i >= 0 {
                    visitor.visit_u64(i as u64)
                } else {
                    Err(DeserializeError::custom("expected non-negative integer"))
                }
            }
            _ => Err(DeserializeError::custom("expected integer")),
        }
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value.kind {
            ValueKind::Float(f) => visitor.visit_f64(f),
            ValueKind::Integer(i) => visitor.visit_f64(i as f64),
            _ => Err(DeserializeError::custom("expected number")),
        }
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value.kind {
            ValueKind::String(s) => visitor.visit_string(s),
            _ => Err(DeserializeError::custom("expected string")),
        }
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value.kind {
            ValueKind::String(s) => visitor.visit_string(s),
            _ => Err(DeserializeError::custom("expected string")),
        }
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value.kind {
            ValueKind::Array(arr) => visitor.visit_seq(SeqDeserializer::new(arr)),
            _ => Err(DeserializeError::custom("expected array")),
        }
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value.kind {
            ValueKind::Object(obj) => visitor.visit_map(MapDeserializer::new(obj)),
            _ => Err(DeserializeError::custom("expected object")),
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
        match self.value.kind {
            ValueKind::Object(obj) => visitor.visit_map(MapDeserializer::new(obj)),
            _ => Err(DeserializeError::custom("expected object")),
        }
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
        match self.value.kind {
            ValueKind::String(s) => visitor.visit_enum(UnitVariantDeserializer { value: s }),
            ValueKind::Object(obj) => {
                if obj.len() == 1 {
                    let (key, val) = obj.into_iter().next().unwrap();
                    visitor.visit_enum(NewtypeVariantDeserializer { key, value: val })
                } else {
                    Err(DeserializeError::custom(
                        "enum variants with multiple fields are not supported (use newtype or unit variants)",
                    ))
                }
            }
            _ => Err(DeserializeError::custom(
                "expected enum (string or single-key object)",
            )),
        }
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value.kind {
            ValueKind::Null => visitor.visit_none(),
            _ => visitor.visit_some(ValueDeserializer::new(self.value)),
        }
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    serde::forward_to_deserialize_any! {
        u8 u16 u32 i8 i16 i32 f32 unit unit_struct newtype_struct
        tuple tuple_struct bytes byte_buf char identifier
    }
}

struct SeqDeserializer {
    array: std::vec::IntoIter<Value>,
}

impl SeqDeserializer {
    fn new(array: Vec<Value>) -> Self {
        SeqDeserializer {
            array: array.into_iter(),
        }
    }
}

impl<'de> SeqAccess<'de> for SeqDeserializer {
    type Error = DeserializeError;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: de::DeserializeSeed<'de>,
    {
        match self.array.next() {
            Some(value) => seed.deserialize(ValueDeserializer::new(value)).map(Some),
            None => Ok(None),
        }
    }
}

struct MapDeserializer {
    iter: indexmap::map::IntoIter<String, Value>,
    value: Option<Value>,
}

impl MapDeserializer {
    fn new(object: IndexMap<String, Value>) -> Self {
        MapDeserializer {
            iter: object.into_iter(),
            value: None,
        }
    }
}

impl<'de> MapAccess<'de> for MapDeserializer {
    type Error = DeserializeError;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: de::DeserializeSeed<'de>,
    {
        match self.iter.next() {
            Some((key, value)) => {
                self.value = Some(value);
                seed.deserialize(ValueDeserializer::new(Value::from(ValueKind::String(key))))
                    .map(Some)
            }
            None => Ok(None),
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: de::DeserializeSeed<'de>,
    {
        match self.value.take() {
            Some(value) => seed.deserialize(ValueDeserializer::new(value)),
            None => Err(DeserializeError::custom("value missing")),
        }
    }
}

struct UnitVariantDeserializer {
    value: String,
}

impl<'de> de::EnumAccess<'de> for UnitVariantDeserializer {
    type Error = DeserializeError;
    type Variant = UnitVariantDeserializer;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: de::DeserializeSeed<'de>,
    {
        let val = seed.deserialize(ValueDeserializer::new(Value::from(ValueKind::String(
            self.value.clone(),
        ))))?;
        Ok((val, self))
    }
}

impl<'de> de::VariantAccess<'de> for UnitVariantDeserializer {
    type Error = DeserializeError;

    fn unit_variant(self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn newtype_variant_seed<T>(self, _seed: T) -> Result<T::Value, Self::Error>
    where
        T: de::DeserializeSeed<'de>,
    {
        Err(DeserializeError::custom("expected unit variant"))
    }

    fn tuple_variant<V>(self, _len: usize, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(DeserializeError::custom(
            "tuple variants not supported; use newtype or unit variants",
        ))
    }

    fn struct_variant<V>(
        self,
        _fields: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(DeserializeError::custom(
            "struct variants not supported; use newtype or unit variants",
        ))
    }
}

struct NewtypeVariantDeserializer {
    key: String,
    value: Value,
}

impl<'de> de::EnumAccess<'de> for NewtypeVariantDeserializer {
    type Error = DeserializeError;
    type Variant = NewtypeVariantDeserializer;

    fn variant_seed<V>(mut self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: de::DeserializeSeed<'de>,
    {
        let key = std::mem::take(&mut self.key);
        let val = seed.deserialize(ValueDeserializer::new(Value::from(ValueKind::String(key))))?;
        Ok((val, self))
    }
}

impl<'de> de::VariantAccess<'de> for NewtypeVariantDeserializer {
    type Error = DeserializeError;

    fn unit_variant(self) -> Result<(), Self::Error> {
        Err(DeserializeError::custom("expected newtype variant"))
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
    where
        T: de::DeserializeSeed<'de>,
    {
        seed.deserialize(ValueDeserializer::new(self.value))
    }

    fn tuple_variant<V>(self, _len: usize, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(DeserializeError::custom(
            "tuple variants not supported; use newtype or unit variants",
        ))
    }

    fn struct_variant<V>(
        self,
        _fields: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(DeserializeError::custom(
            "struct variants not supported; use newtype or unit variants",
        ))
    }
}

// ============================================================================
// SERIALIZER IMPLEMENTATION
// ============================================================================

/// Serializer that converts any `Serialize` type to a COSY `Value`
pub struct ValueSerializer;

impl Serializer for ValueSerializer {
    type Ok = Value;
    type Error = SerializeError;

    type SerializeSeq = SerializeArray;
    type SerializeTuple = SerializeArray;
    type SerializeTupleStruct = SerializeArray;
    type SerializeTupleVariant = SerializeArray;
    type SerializeMap = SerializeObject;
    type SerializeStruct = SerializeObject;
    type SerializeStructVariant = SerializeObject;

    fn serialize_bool(self, v: bool) -> Result<Value, SerializeError> {
        Ok(Value::from(ValueKind::Bool(v)))
    }

    fn serialize_i8(self, v: i8) -> Result<Value, SerializeError> {
        Ok(Value::from(ValueKind::Integer(v as i64)))
    }

    fn serialize_i16(self, v: i16) -> Result<Value, SerializeError> {
        Ok(Value::from(ValueKind::Integer(v as i64)))
    }

    fn serialize_i32(self, v: i32) -> Result<Value, SerializeError> {
        Ok(Value::from(ValueKind::Integer(v as i64)))
    }

    fn serialize_i64(self, v: i64) -> Result<Value, SerializeError> {
        Ok(Value::from(ValueKind::Integer(v)))
    }

    fn serialize_u8(self, v: u8) -> Result<Value, SerializeError> {
        Ok(Value::from(ValueKind::Integer(v as i64)))
    }

    fn serialize_u16(self, v: u16) -> Result<Value, SerializeError> {
        Ok(Value::from(ValueKind::Integer(v as i64)))
    }

    fn serialize_u32(self, v: u32) -> Result<Value, SerializeError> {
        Ok(Value::from(ValueKind::Integer(v as i64)))
    }

    fn serialize_u64(self, v: u64) -> Result<Value, SerializeError> {
        Ok(Value::from(ValueKind::Integer(v as i64)))
    }

    fn serialize_f32(self, v: f32) -> Result<Value, SerializeError> {
        Ok(Value::from(ValueKind::Float(v as f64)))
    }

    fn serialize_f64(self, v: f64) -> Result<Value, SerializeError> {
        Ok(Value::from(ValueKind::Float(v)))
    }

    fn serialize_char(self, v: char) -> Result<Value, SerializeError> {
        Ok(Value::from(ValueKind::String(v.to_string())))
    }

    fn serialize_str(self, v: &str) -> Result<Value, SerializeError> {
        Ok(Value::from(ValueKind::String(v.to_string())))
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Value, SerializeError> {
        Ok(Value::from(ValueKind::Array(
            v.iter()
                .map(|b| Value::from(ValueKind::Integer(*b as i64)))
                .collect(),
        )))
    }

    fn serialize_none(self) -> Result<Value, SerializeError> {
        Ok(Value::from(ValueKind::Null))
    }

    fn serialize_some<T>(self, value: &T) -> Result<Value, SerializeError>
    where
        T: Serialize + ?Sized,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Value, SerializeError> {
        Ok(Value::from(ValueKind::Null))
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Value, SerializeError> {
        Ok(Value::from(ValueKind::Null))
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Value, SerializeError> {
        Ok(Value::from(ValueKind::String(variant.to_string())))
    }

    fn serialize_newtype_struct<T>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Value, SerializeError>
    where
        T: Serialize + ?Sized,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Value, SerializeError>
    where
        T: Serialize + ?Sized,
    {
        let mut map = IndexMap::new();
        map.insert(variant.to_string(), value.serialize(self)?);
        Ok(Value::from(ValueKind::Object(map)))
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<SerializeArray, SerializeError> {
        Ok(SerializeArray {
            array: Vec::with_capacity(len.unwrap_or(0)),
        })
    }

    fn serialize_tuple(self, len: usize) -> Result<SerializeArray, SerializeError> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<SerializeArray, SerializeError> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        len: usize,
    ) -> Result<SerializeArray, SerializeError> {
        self.serialize_seq(Some(len))
    }

    fn serialize_map(self, len: Option<usize>) -> Result<SerializeObject, SerializeError> {
        Ok(SerializeObject {
            object: IndexMap::with_capacity(len.unwrap_or(0)),
            next_key: None,
        })
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<SerializeObject, SerializeError> {
        self.serialize_map(Some(len))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<SerializeObject, SerializeError> {
        Ok(SerializeObject {
            object: IndexMap::with_capacity(len),
            next_key: Some(variant.to_string()),
        })
    }
}

pub struct SerializeArray {
    array: Vec<Value>,
}

impl serde::ser::SerializeSeq for SerializeArray {
    type Ok = Value;
    type Error = SerializeError;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), SerializeError>
    where
        T: Serialize + ?Sized,
    {
        self.array.push(value.serialize(ValueSerializer)?);
        Ok(())
    }

    fn end(self) -> Result<Value, SerializeError> {
        Ok(Value::array(self.array))
    }
}

impl serde::ser::SerializeTuple for SerializeArray {
    type Ok = Value;
    type Error = SerializeError;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), SerializeError>
    where
        T: Serialize + ?Sized,
    {
        self.array.push(value.serialize(ValueSerializer)?);
        Ok(())
    }

    fn end(self) -> Result<Value, SerializeError> {
        Ok(Value::array(self.array))
    }
}

impl serde::ser::SerializeTupleStruct for SerializeArray {
    type Ok = Value;
    type Error = SerializeError;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), SerializeError>
    where
        T: Serialize + ?Sized,
    {
        self.array.push(value.serialize(ValueSerializer)?);
        Ok(())
    }

    fn end(self) -> Result<Value, SerializeError> {
        Ok(Value::array(self.array))
    }
}

impl serde::ser::SerializeTupleVariant for SerializeArray {
    type Ok = Value;
    type Error = SerializeError;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), SerializeError>
    where
        T: Serialize + ?Sized,
    {
        self.array.push(value.serialize(ValueSerializer)?);
        Ok(())
    }

    fn end(self) -> Result<Value, SerializeError> {
        Ok(Value::array(self.array))
    }
}

pub struct SerializeObject {
    object: IndexMap<String, Value>,
    next_key: Option<String>,
}

impl SerializeMap for SerializeObject {
    type Ok = Value;
    type Error = SerializeError;

    fn serialize_key<T>(&mut self, key: &T) -> Result<(), SerializeError>
    where
        T: Serialize + ?Sized,
    {
        self.next_key = Some(match key.serialize(ValueSerializer)?.kind {
            ValueKind::String(s) => s,
            _ => return Err(SerializeError::custom("keys must be strings")),
        });
        Ok(())
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<(), SerializeError>
    where
        T: Serialize + ?Sized,
    {
        if let Some(key) = self.next_key.take() {
            self.object.insert(key, value.serialize(ValueSerializer)?);
            Ok(())
        } else {
            Err(SerializeError::custom(
                "serialize_value called before serialize_key",
            ))
        }
    }

    fn end(self) -> Result<Value, SerializeError> {
        Ok(Value::object(self.object))
    }
}

impl serde::ser::SerializeStruct for SerializeObject {
    type Ok = Value;
    type Error = SerializeError;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), SerializeError>
    where
        T: Serialize + ?Sized,
    {
        self.object
            .insert(key.to_string(), value.serialize(ValueSerializer)?);
        Ok(())
    }

    fn end(self) -> Result<Value, SerializeError> {
        Ok(Value::object(self.object))
    }
}

impl serde::ser::SerializeStructVariant for SerializeObject {
    type Ok = Value;
    type Error = SerializeError;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), SerializeError>
    where
        T: Serialize + ?Sized,
    {
        self.object
            .insert(key.to_string(), value.serialize(ValueSerializer)?);
        Ok(())
    }

    fn end(self) -> Result<Value, SerializeError> {
        Ok(Value::object(self.object))
    }
}
