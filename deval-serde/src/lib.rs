use std::fmt::Display;

use deval_data_model::{Annotated, AnnotatedData};
use serde::{de::{self, MapAccess, SeqAccess, Visitor}, Deserialize, Deserializer};

pub fn deserialize_from_annotated<'a, R>(data: &'a Annotated<AnnotatedData>) -> R
where
    R: Deserialize<'a>,
{
    #[derive(Debug)]
    struct MyError(String);

    impl Display for MyError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    impl std::error::Error for MyError {
    }

    impl de::Error for MyError {
        fn custom<T>(msg: T) -> Self
        where
            T: std::fmt::Display,
        {
            MyError(format!("{}", msg))
        }
    }

    struct MyStringDeserializer<'b>(&'b Annotated<String>);

    impl<'b> Deserializer<'b> for MyStringDeserializer<'b> {
        type Error = MyError;
    
        fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'b> {
            visitor.visit_str(&self.0.value)
        }
    
        fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'b> {
            self.deserialize_any(visitor)
        }
    
        fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'b> {
            self.deserialize_any(visitor)
        }
    
        fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'b> {
            self.deserialize_any(visitor)
        }
    
        fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'b> {
            self.deserialize_any(visitor)
        }
    
        fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'b> {
            self.deserialize_any(visitor)
        }
    
        fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'b> {
            self.deserialize_any(visitor)
        }
    
        fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'b> {
            self.deserialize_any(visitor)
        }
    
        fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'b> {
            self.deserialize_any(visitor)
        }
    
        fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'b> {
            self.deserialize_any(visitor)
        }
    
        fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'b> {
            self.deserialize_any(visitor)
        }
    
        fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'b> {
            self.deserialize_any(visitor)
        }
    
        fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'b> {
            self.deserialize_any(visitor)
        }
    
        fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'b> {
            self.deserialize_any(visitor)
        }
    
        fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'b> {
            self.deserialize_any(visitor)
        }
    
        fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'b> {
            self.deserialize_any(visitor)
        }
    
        fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'b> {
            self.deserialize_any(visitor)
        }
    
        fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'b> {
            self.deserialize_any(visitor)
        }
    
        fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'b> {
            self.deserialize_any(visitor)
        }
    
        fn deserialize_unit_struct<V>(
            self,
            _name: &'static str,
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'b> {
            self.deserialize_any(visitor)
        }
    
        fn deserialize_newtype_struct<V>(
            self,
            _name: &'static str,
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'b> {
            self.deserialize_any(visitor)
        }
    
        fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'b> {
            self.deserialize_any(visitor)
        }
    
        fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'b> {
            self.deserialize_any(visitor)
        }
    
        fn deserialize_tuple_struct<V>(
            self,
            _name: &'static str,
            _len: usize,
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'b> {
            self.deserialize_any(visitor)
        }
    
        fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'b> {
            self.deserialize_any(visitor)
        }
    
        fn deserialize_struct<V>(
            self,
            _name: &'static str,
            _fields: &'static [&'static str],
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'b> {
            self.deserialize_any(visitor)
        }
    
        fn deserialize_enum<V>(
            self,
            _name: &'static str,
            _variants: &'static [&'static str],
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'b> {
            self.deserialize_any(visitor)
        }
    
        fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'b> {
            self.deserialize_any(visitor)
        }
    
        fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'b> {
            self.deserialize_any(visitor)
        }
    }

    struct MySeqAccess<'b>(std::slice::Iter<'b, Annotated<AnnotatedData>>);

    impl<'b> SeqAccess<'b> for MySeqAccess<'b> {
        type Error = MyError;
    
        fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
        where
            T: de::DeserializeSeed<'b> {
            let Some(v) = self.0.next() else {
                return Ok(None);
            };
            seed.deserialize(MyDeserializer(v)).map(Some)
        }
    }

    struct MyMapAccess<'b>(std::slice::Iter<'b, (Annotated<String>, Annotated<AnnotatedData>)>, Option<&'b Annotated<AnnotatedData>>);

    impl<'b> MapAccess<'b> for MyMapAccess<'b> {
        type Error = MyError;
    
        fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
        where
            K: de::DeserializeSeed<'b> {
            let Some(kv) = self.0.next() else {
                return Ok(None);
            };
            self.1 = Some(&kv.1);
            seed.deserialize(MyStringDeserializer(&kv.0)).map(Some)
        }
    
        fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
        where
            V: de::DeserializeSeed<'b> {
            let v = self.1.unwrap();
            seed.deserialize(MyDeserializer(v))
        }
    }

    struct MyEnumAccess<'b> {
        tag: String,
        value: Option<&'b Annotated<AnnotatedData>>,
        variants: std::slice::Iter<'b, (Annotated<String>, Annotated<AnnotatedData>)>,
    }

    impl<'b> de::EnumAccess<'b> for MyEnumAccess<'b> {
        type Error = MyError;
        type Variant = Self;

        fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
        where
            V: de::DeserializeSeed<'b>,
        {
            // For externally tagged enums, we would have already determined the variant
            // For internally tagged enums, we need to find the tag field
            let variant_value = seed.deserialize(de::value::StrDeserializer::new(&self.tag))?;
            Ok((variant_value, self))
        }
    }

    impl<'b> de::VariantAccess<'b> for MyEnumAccess<'b> {
        type Error = MyError;

        fn unit_variant(self) -> Result<(), Self::Error> {
            Ok(())
        }

        fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
        where
            T: de::DeserializeSeed<'b>,
        {
            match self.value {
                Some(value) => seed.deserialize(MyDeserializer(value)),
                None => Err(de::Error::custom("expected value for newtype variant")),
            }
        }

        fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'b>,
        {
            match self.value {
                Some(value) => visitor.visit_seq(MySeqAccess(std::slice::from_ref(value).iter())),
                None => Err(de::Error::custom("expected value for tuple variant")),
            }
        }

        fn struct_variant<V>(
            self,
            _fields: &'static [&'static str],
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'b>,
        {
            match self.value {
                Some(value) => match &value.value {
                    AnnotatedData::Object(items) => {
                        visitor.visit_map(MyMapAccess(items.iter(), None))
                    }
                    _ => Err(de::Error::custom("expected object for struct variant")),
                },
                None => Err(de::Error::custom("expected value for struct variant")),
            }
        }
    }

    struct MyStructAccess<'b> {
        fields: std::slice::Iter<'b, (Annotated<String>, Annotated<AnnotatedData>)>,
        current_value: Option<&'b Annotated<AnnotatedData>>,
        tag_field: Option<&'static str>,
    }

    impl<'b> de::MapAccess<'b> for MyStructAccess<'b> {
        type Error = MyError;

        fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
        where
            K: de::DeserializeSeed<'b>,
        {
            // Skip the tag field if specified
            while let Some((key, value)) = self.fields.next() {
                if let Some(tag_field) = self.tag_field {
                    if key.value == tag_field {
                        continue;
                    }
                }
                self.current_value = Some(value);
                return seed.deserialize(MyStringDeserializer(key)).map(Some);
            }
            Ok(None)
        }

        fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
        where
            V: de::DeserializeSeed<'b>,
        {
            match self.current_value.take() {
                Some(value) => seed.deserialize(MyDeserializer(value)),
                None => Err(de::Error::custom("no value available")),
            }
        }
    }

    struct MyDeserializer<'b>(&'b Annotated<AnnotatedData>);

    impl<'b> Deserializer<'b> for MyDeserializer<'b> {
        type Error = MyError;

        fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'b>,
        {
            match &self.0.value {
                AnnotatedData::Null => visitor.visit_unit(),
                AnnotatedData::Bool(b) => visitor.visit_bool(b.value),
                AnnotatedData::Number(annotated) => visitor.visit_f64(annotated.value),
                AnnotatedData::String(annotated) => visitor.visit_str(&annotated.value),
                AnnotatedData::Array(items) => {
                    visitor.visit_seq(MySeqAccess(items.iter()))
                },
                AnnotatedData::Object(items) => {
                    visitor.visit_map(MyMapAccess(items.iter(), None))
                },
            }
        }

        fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'b>,
        {
            match &self.0.value {
                AnnotatedData::Bool(b) => visitor.visit_bool(b.value),
                _ => self.deserialize_any(visitor),
            }
        }

        fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'b>,
        {
            match &self.0.value {
                AnnotatedData::Number(n) => {
                    if n.value.fract() == 0.0 && n.value >= i8::MIN as f64 && n.value <= i8::MAX as f64 {
                        visitor.visit_i8(n.value as i8)
                    } else {
                        Err(de::Error::custom(format!("cannot convert {} to i8", n.value)))
                    }
                },
                _ => self.deserialize_any(visitor),
            }
        }

        fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'b>,
        {
            match &self.0.value {
                AnnotatedData::Number(n) => {
                    if n.value.fract() == 0.0 && n.value >= i16::MIN as f64 && n.value <= i16::MAX as f64 {
                        visitor.visit_i16(n.value as i16)
                    } else {
                        Err(de::Error::custom(format!("cannot convert {} to i16", n.value)))
                    }
                },
                _ => self.deserialize_any(visitor),
            }
        }

        fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'b>,
        {
            match &self.0.value {
                AnnotatedData::Number(n) => {
                    if n.value.fract() == 0.0 && n.value >= i32::MIN as f64 && n.value <= i32::MAX as f64 {
                        visitor.visit_i32(n.value as i32)
                    } else {
                        Err(de::Error::custom(format!("cannot convert {} to i32", n.value)))
                    }
                },
                _ => self.deserialize_any(visitor),
            }
        }

        fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'b>,
        {
            match &self.0.value {
                AnnotatedData::Number(n) => {
                    if n.value.fract() == 0.0 && n.value >= i64::MIN as f64 && n.value <= i64::MAX as f64 {
                        visitor.visit_i64(n.value as i64)
                    } else {
                        Err(de::Error::custom(format!("cannot convert {} to i64", n.value)))
                    }
                },
                _ => self.deserialize_any(visitor),
            }
        }

        fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'b>,
        {
            match &self.0.value {
                AnnotatedData::Number(n) => {
                    if n.value.fract() == 0.0 && n.value >= 0.0 && n.value <= u8::MAX as f64 {
                        visitor.visit_u8(n.value as u8)
                    } else {
                        Err(de::Error::custom(format!("cannot convert {} to u8", n.value)))
                    }
                },
                _ => self.deserialize_any(visitor),
            }
        }

        fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'b>,
        {
            match &self.0.value {
                AnnotatedData::Number(n) => {
                    if n.value.fract() == 0.0 && n.value >= 0.0 && n.value <= u16::MAX as f64 {
                        visitor.visit_u16(n.value as u16)
                    } else {
                        Err(de::Error::custom(format!("cannot convert {} to u16", n.value)))
                    }
                },
                _ => self.deserialize_any(visitor),
            }
        }

        fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'b>,
        {
            match &self.0.value {
                AnnotatedData::Number(n) => {
                    if n.value.fract() == 0.0 && n.value >= 0.0 && n.value <= u32::MAX as f64 {
                        visitor.visit_u32(n.value as u32)
                    } else {
                        Err(de::Error::custom(format!("cannot convert {} to u32", n.value)))
                    }
                },
                _ => self.deserialize_any(visitor),
            }
        }

        fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'b>,
        {
            match &self.0.value {
                AnnotatedData::Number(n) => {
                    if n.value.fract() == 0.0 && n.value >= 0.0 && n.value <= u64::MAX as f64 {
                        visitor.visit_u64(n.value as u64)
                    } else {
                        Err(de::Error::custom(format!("cannot convert {} to u64", n.value)))
                    }
                },
                _ => self.deserialize_any(visitor),
            }
        }

        fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'b>,
        {
            match &self.0.value {
                AnnotatedData::Number(n) => visitor.visit_f32(n.value as f32),
                _ => self.deserialize_any(visitor),
            }
        }

        fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'b>,
        {
            match &self.0.value {
                AnnotatedData::Number(n) => visitor.visit_f64(n.value),
                _ => self.deserialize_any(visitor),
            }
        }

        fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'b>,
        {
            self.deserialize_any(visitor)
        }

        fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'b>,
        {
            match &self.0.value {
                AnnotatedData::String(s) => visitor.visit_str(&s.value),
                _ => self.deserialize_any(visitor),
            }
        }

        fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'b>,
        {
            self.deserialize_str(visitor)
        }

        fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'b>,
        {
            self.deserialize_any(visitor)
        }

        fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'b>,
        {
            self.deserialize_any(visitor)
        }

        fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'b>,
        {
            match &self.0.value {
                AnnotatedData::Null => visitor.visit_none(),
                _ => visitor.visit_some(self),
            }
        }

        fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'b>,
        {
            match &self.0.value {
                AnnotatedData::Null => visitor.visit_unit(),
                _ => self.deserialize_any(visitor),
            }
        }

        fn deserialize_unit_struct<V>(
            self,
            _name: &'static str,
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'b>,
        {
            self.deserialize_unit(visitor)
        }

        fn deserialize_newtype_struct<V>(
            self,
            _name: &'static str,
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'b>,
        {
            // For newtype structs, we deserialize the inner value directly
            visitor.visit_newtype_struct(self)
        }

        fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'b>,
        {
            match &self.0.value {
                AnnotatedData::Array(items) => {
                    visitor.visit_seq(MySeqAccess(items.iter()))
                },
                _ => self.deserialize_any(visitor),
            }
        }

        fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'b>,
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
            V: Visitor<'b>,
        {
            self.deserialize_seq(visitor)
        }

        fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'b>,
        {
            match &self.0.value {
                AnnotatedData::Object(items) => {
                    visitor.visit_map(MyMapAccess(items.iter(), None))
                },
                _ => self.deserialize_any(visitor),
            }
        }

        fn deserialize_struct<V>(
            self,
            _name: &'static str,
            _fields: &'static [&'static str],
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'b>,
        {
            match &self.0.value {
                AnnotatedData::Object(items) => {
                    visitor.visit_map(MyMapAccess(items.iter(), None))
                },
                _ => self.deserialize_any(visitor),
            }
        }

        fn deserialize_enum<V>(
            self,
            _name: &'static str,
            _variants: &'static [&'static str],
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'b>,
        {
            match &self.0.value {
                AnnotatedData::Object(items) => {
                    // For internally tagged enums, we need to find the tag field
                    // For simplicity, we'll just visit the map directly
                    visitor.visit_map(MyMapAccess(items.iter(), None))
                },
                AnnotatedData::String(s) => {
                    // For externally tagged unit variants
                    visitor.visit_enum(de::value::StrDeserializer::new(&s.value))
                },
                _ => self.deserialize_any(visitor),
            }
        }

        fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'b>,
        {
            self.deserialize_str(visitor)
        }

        fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'b>,
        {
            self.deserialize_any(visitor)
        }
    }

    R::deserialize(MyDeserializer(data)).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use deval_data_model::{Annotated, AnnotatedData, Span, SpanSet};
    use serde::Deserialize;

    fn annotated_string(value: &str) -> Annotated<String> {
        Annotated {
            value: value.to_string(),
            span: SpanSet(vec![Span {
                filename: "test".to_string(),
                start: 0,
                end: value.len(),
            }]),
            docs: String::new(),
            semantic_type: None,
        }
    }

    fn annotated_number(value: f64) -> Annotated<f64> {
        Annotated {
            value,
            span: SpanSet(vec![Span {
                filename: "test".to_string(),
                start: 0,
                end: format!("{}", value).len(),
            }]),
            docs: String::new(),
            semantic_type: None,
        }
    }

    fn annotated_bool(value: bool) -> Annotated<bool> {
        Annotated {
            value,
            span: SpanSet(vec![Span {
                filename: "test".to_string(),
                start: 0,
                end: if value { 4 } else { 5 },
            }]),
            docs: String::new(),
            semantic_type: None,
        }
    }

    fn annotated_null() -> AnnotatedData {
        AnnotatedData::Null
    }

    #[test]
    fn test_deserialize_string() {
        let data = Annotated {
            value: AnnotatedData::String(annotated_string("hello")),
            span: SpanSet(vec![Span {
                filename: "test".to_string(),
                start: 0,
                end: 7,
            }]),
            docs: String::new(),
            semantic_type: None,
        };

        let result: String = deserialize_from_annotated(&data);
        assert_eq!(result, "hello");
    }

    #[test]
    fn test_deserialize_number() {
        let data = Annotated {
            value: AnnotatedData::Number(annotated_number(42.5)),
            span: SpanSet(vec![Span {
                filename: "test".to_string(),
                start: 0,
                end: 4,
            }]),
            docs: String::new(),
            semantic_type: None,
        };

        let result: f64 = deserialize_from_annotated(&data);
        assert_eq!(result, 42.5);
    }

    #[test]
    fn test_deserialize_integer_types() {
        let data = Annotated {
            value: AnnotatedData::Number(annotated_number(42.0)),
            span: SpanSet(vec![Span {
                filename: "test".to_string(),
                start: 0,
                end: 2,
            }]),
            docs: String::new(),
            semantic_type: None,
        };

        // Test different integer types
        let result_i8: i8 = deserialize_from_annotated(&data);
        assert_eq!(result_i8, 42);

        let result_i16: i16 = deserialize_from_annotated(&data);
        assert_eq!(result_i16, 42);

        let result_i32: i32 = deserialize_from_annotated(&data);
        assert_eq!(result_i32, 42);

        let result_i64: i64 = deserialize_from_annotated(&data);
        assert_eq!(result_i64, 42);

        let result_u8: u8 = deserialize_from_annotated(&data);
        assert_eq!(result_u8, 42);

        let result_u16: u16 = deserialize_from_annotated(&data);
        assert_eq!(result_u16, 42);

        let result_u32: u32 = deserialize_from_annotated(&data);
        assert_eq!(result_u32, 42);

        let result_u64: u64 = deserialize_from_annotated(&data);
        assert_eq!(result_u64, 42);

        let result_f32: f32 = deserialize_from_annotated(&data);
        assert_eq!(result_f32, 42.0);
    }

    #[test]
    fn test_deserialize_bool() {
        let data = Annotated {
            value: AnnotatedData::Bool(annotated_bool(true)),
            span: SpanSet(vec![Span {
                filename: "test".to_string(),
                start: 0,
                end: 4,
            }]),
            docs: String::new(),
            semantic_type: None,
        };

        let result: bool = deserialize_from_annotated(&data);
        assert_eq!(result, true);
    }

    #[test]
    fn test_deserialize_null() {
        let data = Annotated {
            value: annotated_null(),
            span: SpanSet(vec![Span {
                filename: "test".to_string(),
                start: 0,
                end: 4,
            }]),
            docs: String::new(),
            semantic_type: None,
        };

        #[derive(Deserialize, Debug, PartialEq)]
        struct NullTest;

        let result: Option<NullTest> = deserialize_from_annotated(&data);
        assert_eq!(result, None);
    }

    #[test]
    fn test_deserialize_array() {
        let data = Annotated {
            value: AnnotatedData::Array(vec![
                Annotated {
                    value: AnnotatedData::Number(annotated_number(1.0)),
                    span: SpanSet(vec![Span {
                        filename: "test".to_string(),
                        start: 0,
                        end: 1,
                    }]),
                    docs: String::new(),
                    semantic_type: None,
                },
                Annotated {
                    value: AnnotatedData::Number(annotated_number(2.0)),
                    span: SpanSet(vec![Span {
                        filename: "test".to_string(),
                        start: 0,
                        end: 1,
                    }]),
                    docs: String::new(),
                    semantic_type: None,
                },
                Annotated {
                    value: AnnotatedData::Number(annotated_number(3.0)),
                    span: SpanSet(vec![Span {
                        filename: "test".to_string(),
                        start: 0,
                        end: 1,
                    }]),
                    docs: String::new(),
                    semantic_type: None,
                },
            ]),
            span: SpanSet(vec![Span {
                filename: "test".to_string(),
                start: 0,
                end: 4,
            }]),
            docs: String::new(),
            semantic_type: None,
        };

        let result: Vec<f64> = deserialize_from_annotated(&data);
        assert_eq!(result, vec![1.0, 2.0, 3.0]);
    }

    #[test]
    fn test_deserialize_object() {
        let data = Annotated {
            value: AnnotatedData::Object(vec![
                (
                    annotated_string("name"),
                    Annotated {
                        value: AnnotatedData::String(annotated_string("John")),
                        span: SpanSet(vec![Span {
                            filename: "test".to_string(),
                            start: 0,
                            end: 6,
                        }]),
                        docs: String::new(),
                        semantic_type: None,
                    },
                ),
                (
                    annotated_string("age"),
                    Annotated {
                        value: AnnotatedData::Number(annotated_number(30.0)),
                        span: SpanSet(vec![Span {
                            filename: "test".to_string(),
                            start: 0,
                            end: 2,
                        }]),
                        docs: String::new(),
                        semantic_type: None,
                    },
                ),
            ]),
            span: SpanSet(vec![Span {
                filename: "test".to_string(),
                start: 0,
                end: 4,
            }]),
            docs: String::new(),
            semantic_type: None,
        };

        #[derive(Deserialize, Debug, PartialEq)]
        struct Person {
            name: String,
            age: f64,
        }

        let result: Person = deserialize_from_annotated(&data);
        assert_eq!(
            result,
            Person {
                name: "John".to_string(),
                age: 30.0
            }
        );
    }

    #[test]
    fn test_deserialize_nested_object() {
        let data = Annotated {
            value: AnnotatedData::Object(vec![
                (
                    annotated_string("person"),
                    Annotated {
                        value: AnnotatedData::Object(vec![
                            (
                                annotated_string("name"),
                                Annotated {
                                    value: AnnotatedData::String(annotated_string("Alice")),
                                    span: SpanSet(vec![Span {
                                        filename: "test".to_string(),
                                        start: 0,
                                        end: 7,
                                    }]),
                                    docs: String::new(),
                                    semantic_type: None,
                                },
                            ),
                            (
                                annotated_string("age"),
                                Annotated {
                                    value: AnnotatedData::Number(annotated_number(25.0)),
                                    span: SpanSet(vec![Span {
                                        filename: "test".to_string(),
                                        start: 0,
                                        end: 2,
                                    }]),
                                    docs: String::new(),
                                    semantic_type: None,
                                },
                            ),
                        ]),
                        span: SpanSet(vec![Span {
                            filename: "test".to_string(),
                            start: 0,
                            end: 4,
                        }]),
                        docs: String::new(),
                        semantic_type: None,
                    },
                ),
                (
                    annotated_string("active"),
                    Annotated {
                        value: AnnotatedData::Bool(annotated_bool(true)),
                        span: SpanSet(vec![Span {
                            filename: "test".to_string(),
                            start: 0,
                            end: 4,
                        }]),
                        docs: String::new(),
                        semantic_type: None,
                    },
                ),
            ]),
            span: SpanSet(vec![Span {
                filename: "test".to_string(),
                start: 0,
                end: 4,
            }]),
            docs: String::new(),
            semantic_type: None,
        };

        #[derive(Deserialize, Debug, PartialEq)]
        struct Person {
            name: String,
            age: f64,
        }

        #[derive(Deserialize, Debug, PartialEq)]
        struct Data {
            person: Person,
            active: bool,
        }

        let result: Data = deserialize_from_annotated(&data);
        assert_eq!(
            result,
            Data {
                person: Person {
                    name: "Alice".to_string(),
                    age: 25.0
                },
                active: true
            }
        );
    }

    #[test]
    fn test_deserialize_newtype_struct() {
        #[derive(Deserialize, Debug, PartialEq)]
        struct Millimeters(u32);

        let data = Annotated {
            value: AnnotatedData::Number(annotated_number(100.0)),
            span: SpanSet(vec![Span {
                filename: "test".to_string(),
                start: 0,
                end: 3,
            }]),
            docs: String::new(),
            semantic_type: None,
        };

        let result: Millimeters = deserialize_from_annotated(&data);
        assert_eq!(result, Millimeters(100));
    }

    #[test]
    fn test_deserialize_enum_external_tag() {
        #[derive(Deserialize, Debug, PartialEq)]
        enum Message {
            Request,
            Response,
        }

        // Test Request variant
        let request_data = Annotated {
            value: AnnotatedData::String(annotated_string("Request")),
            span: SpanSet(vec![Span {
                filename: "test".to_string(),
                start: 0,
                end: 9,
            }]),
            docs: String::new(),
            semantic_type: None,
        };

        let request_result: Message = deserialize_from_annotated(&request_data);
        assert_eq!(request_result, Message::Request);

        // Test Response variant
        let response_data = Annotated {
            value: AnnotatedData::String(annotated_string("Response")),
            span: SpanSet(vec![Span {
                filename: "test".to_string(),
                start: 0,
                end: 10,
            }]),
            docs: String::new(),
            semantic_type: None,
        };

        let response_result: Message = deserialize_from_annotated(&response_data);
        assert_eq!(response_result, Message::Response);
    }

    #[test]
    #[should_panic(expected = "cannot convert 2.5 to i32")]
    fn test_deserialize_float_to_int_should_fail() {
        #[derive(Deserialize, Debug)]
        struct Point {
            x: i32,
        }

        let data = Annotated {
            value: AnnotatedData::Object(vec![
                (
                    annotated_string("x"),
                    Annotated {
                        value: AnnotatedData::Number(annotated_number(2.5)), // Float value
                        span: SpanSet(vec![Span {
                            filename: "test".to_string(),
                            start: 0,
                            end: 3,
                        }]),
                        docs: String::new(),
                        semantic_type: None,
                    },
                ),
            ]),
            span: SpanSet(vec![Span {
                filename: "test".to_string(),
                start: 0,
                end: 4,
            }]),
            docs: String::new(),
            semantic_type: None,
        };

        let _result: Point = deserialize_from_annotated(&data);
    }
}