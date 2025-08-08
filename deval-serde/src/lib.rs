use std::fmt::Display;

use deval_data_model::{Annotated, AnnotatedData};
use serde::{de::{self, MapAccess, SeqAccess}, Deserialize, Deserializer};

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
            MyError(format!("{msg}"))
        }
    }

    struct MyStringDeserializer<'b>(&'b Annotated<String>);

    impl<'b> Deserializer<'b> for MyStringDeserializer<'b> {
        type Error = MyError;
    
        fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: de::Visitor<'b> {
            visitor.visit_str(&self.0.value)
        }
    
        fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: de::Visitor<'b> {
            todo!()
        }
    
        fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: de::Visitor<'b> {
            todo!()
        }
    
        fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: de::Visitor<'b> {
            todo!()
        }
    
        fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: de::Visitor<'b> {
            todo!()
        }
    
        fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: de::Visitor<'b> {
            todo!()
        }
    
        fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: de::Visitor<'b> {
            todo!()
        }
    
        fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: de::Visitor<'b> {
            todo!()
        }
    
        fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: de::Visitor<'b> {
            todo!()
        }
    
        fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: de::Visitor<'b> {
            todo!()
        }
    
        fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: de::Visitor<'b> {
            todo!()
        }
    
        fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: de::Visitor<'b> {
            todo!()
        }
    
        fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: de::Visitor<'b> {
            todo!()
        }
    
        fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: de::Visitor<'b> {
            todo!()
        }
    
        fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: de::Visitor<'b> {
            todo!()
        }
    
        fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: de::Visitor<'b> {
            todo!()
        }
    
        fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: de::Visitor<'b> {
            todo!()
        }
    
        fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: de::Visitor<'b> {
            todo!()
        }
    
        fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: de::Visitor<'b> {
            todo!()
        }
    
        fn deserialize_unit_struct<V>(
            self,
            name: &'static str,
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: de::Visitor<'b> {
            todo!()
        }
    
        fn deserialize_newtype_struct<V>(
            self,
            name: &'static str,
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: de::Visitor<'b> {
            todo!()
        }
    
        fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: de::Visitor<'b> {
            todo!()
        }
    
        fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: de::Visitor<'b> {
            todo!()
        }
    
        fn deserialize_tuple_struct<V>(
            self,
            name: &'static str,
            len: usize,
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: de::Visitor<'b> {
            todo!()
        }
    
        fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: de::Visitor<'b> {
            todo!()
        }
    
        fn deserialize_struct<V>(
            self,
            name: &'static str,
            fields: &'static [&'static str],
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: de::Visitor<'b> {
            todo!()
        }
    
        fn deserialize_enum<V>(
            self,
            name: &'static str,
            variants: &'static [&'static str],
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: de::Visitor<'b> {
            todo!()
        }
    
        fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: de::Visitor<'b> {
            self.deserialize_any(visitor)
        }
    
        fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: de::Visitor<'b> {
            todo!()
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

    struct MyDeserializer<'b>(&'b Annotated<AnnotatedData>);

    impl<'b> Deserializer<'b> for MyDeserializer<'b> {
        type Error = MyError;

        fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: serde::de::Visitor<'b>,
        {
            match &self.0.value {
                AnnotatedData::Null => visitor.visit_unit(),
                AnnotatedData::Bool(b) => visitor.visit_bool(b.value),
                AnnotatedData::Number(annotated) => todo!(),
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
            V: serde::de::Visitor<'b>,
        {
            todo!()
        }

        fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: serde::de::Visitor<'b>,
        {
            todo!()
        }

        fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: serde::de::Visitor<'b>,
        {
            todo!()
        }

        fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: serde::de::Visitor<'b>,
        {
            todo!()
        }

        fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: serde::de::Visitor<'b>,
        {
            todo!()
        }

        fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: serde::de::Visitor<'b>,
        {
            todo!()
        }

        fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: serde::de::Visitor<'b>,
        {
            todo!()
        }

        fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: serde::de::Visitor<'b>,
        {
            todo!()
        }

        fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: serde::de::Visitor<'b>,
        {
            todo!()
        }

        fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: serde::de::Visitor<'b>,
        {
            todo!()
        }

        fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: serde::de::Visitor<'b>,
        {
            todo!()
        }

        fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: serde::de::Visitor<'b>,
        {
            todo!()
        }

        fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: serde::de::Visitor<'b>,
        {
            todo!()
        }

        fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: serde::de::Visitor<'b>,
        {
            self.deserialize_any(visitor)
        }

        fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: serde::de::Visitor<'b>,
        {
            todo!()
        }

        fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: serde::de::Visitor<'b>,
        {
            todo!()
        }

        fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: serde::de::Visitor<'b>,
        {
            todo!()
        }

        fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: serde::de::Visitor<'b>,
        {
            todo!()
        }

        fn deserialize_unit_struct<V>(
            self,
            name: &'static str,
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: serde::de::Visitor<'b>,
        {
            todo!()
        }

        fn deserialize_newtype_struct<V>(
            self,
            name: &'static str,
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: serde::de::Visitor<'b>,
        {
            todo!()
        }

        fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: serde::de::Visitor<'b>,
        {
            self.deserialize_any(visitor)
        }

        fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: serde::de::Visitor<'b>,
        {
            todo!()
        }

        fn deserialize_tuple_struct<V>(
            self,
            name: &'static str,
            len: usize,
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: serde::de::Visitor<'b>,
        {
            todo!()
        }

        fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: serde::de::Visitor<'b>,
        {
            todo!()
        }

        fn deserialize_struct<V>(
            self,
            _: &'static str,
            _: &'static [&'static str],
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: serde::de::Visitor<'b>,
        {
            self.deserialize_any(visitor)
        }

        fn deserialize_enum<V>(
            self,
            name: &'static str,
            variants: &'static [&'static str],
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: serde::de::Visitor<'b>,
        {
            todo!()
        }

        fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: serde::de::Visitor<'b>,
        {
            todo!()
        }

        fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: serde::de::Visitor<'b>,
        {
            todo!()
        }
    }

    R::deserialize(MyDeserializer(data)).unwrap()
}
