//! Serializes a value into a __ of tokens

use crate::{error::Error, Token};
use futures::{sink::Sink, unsync::mpsc, AsyncSink};
use serde::ser::{self, Serialize};

///
///
/// example of some part of an IPLD selector
/// ```rust
/// let (token_sink, token_stream) = futures::unsync::mpsc::channel();
/// let mut ser = Tokenizer(token_sink);
///
/// let json_str: &[u8] = r#"[1, "hello", {"A": false}]"#;
/// let de = serde_json::Deserializer;
///
/// /* produces a stream of ...? */
/// transcode(de, &mut ser);
///
/// /* reduce the stream into a future of a resolved value or RawDag */
/// token_sink.map(|token| {
///     format!("{:?}", token)
/// });
/// ```
#[derive(Clone)]
pub struct Tokenizer<'a>(mpsc::Sender<Token<'a>>);

impl<'a> Tokenizer<'a> {
    fn write_token(&mut self, token: Token<'a>) -> Result<(), Error> {
        self.0
            .start_send(token)
            .map_err(|_| Error::TokenWrite(String::from("")))
            .and_then(|sink| match sink {
                AsyncSink::Ready => Ok(()),
                AsyncSink::NotReady(_) => Err(Error::TokenWrite(String::from(""))),
            })
    }
}

impl<'s, 'a> ser::Serializer for &'s mut Tokenizer<'a> {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Compound<'a>;
    type SerializeTuple = Compound<'a>;
    type SerializeTupleStruct = Compound<'a>;
    type SerializeTupleVariant = Compound<'a>;
    type SerializeMap = Compound<'a>;
    type SerializeStruct = Compound<'a>;
    type SerializeStructVariant = Compound<'a>;

    fn serialize_bool(self, v: bool) -> Result<(), Error> {
        self.write_token(Token::Bool(v))?;
        Ok(())
    }

    fn serialize_i8(self, v: i8) -> Result<(), Error> {
        self.write_token(Token::I8(v))?;
        Ok(())
    }

    fn serialize_i16(self, v: i16) -> Result<(), Error> {
        self.write_token(Token::I16(v))?;
        Ok(())
    }

    fn serialize_i32(self, v: i32) -> Result<(), Error> {
        self.write_token(Token::I32(v))?;
        Ok(())
    }

    fn serialize_i64(self, v: i64) -> Result<(), Error> {
        self.write_token(Token::I64(v))?;
        Ok(())
    }

    fn serialize_i128(self, v: i128) -> Result<(), Error> {
        self.write_token(Token::I128(v))?;
        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<(), Error> {
        self.write_token(Token::U8(v))?;
        Ok(())
    }

    fn serialize_u16(self, v: u16) -> Result<(), Error> {
        self.write_token(Token::U16(v))?;
        Ok(())
    }

    fn serialize_u32(self, v: u32) -> Result<(), Error> {
        self.write_token(Token::U32(v))?;
        Ok(())
    }

    fn serialize_u64(self, v: u64) -> Result<(), Error> {
        self.write_token(Token::U64(v))?;
        Ok(())
    }

    fn serialize_u128(self, v: u128) -> Result<(), Error> {
        self.write_token(Token::U128(v))?;
        Ok(())
    }

    fn serialize_f32(self, v: f32) -> Result<(), Error> {
        self.write_token(Token::F32(v))?;
        Ok(())
    }

    fn serialize_f64(self, v: f64) -> Result<(), Error> {
        self.write_token(Token::F64(v))?;
        Ok(())
    }

    fn serialize_char(self, v: char) -> Result<(), Error> {
        self.write_token(Token::Char(v))?;
        Ok(())
    }

    fn serialize_str(self, v: &str) -> Result<(), Error> {
        // self.write_token(Token::BorrowedStr(v))?;
        Ok(())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<(), Self::Error> {
        // self.write_token(Token::BorrowedBytes(v))?;
        Ok(())
    }

    fn serialize_unit(self) -> Result<(), Error> {
        self.write_token(Token::Unit)?;
        Ok(())
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<(), Error> {
        self.write_token(Token::UnitStruct { name })?;
        Ok(())
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<(), Error> {
        self.write_token(Token::UnitVariant { name, variant })?;
        Ok(())
    }

    fn serialize_newtype_struct<T: ?Sized>(self, name: &'static str, value: &T) -> Result<(), Error>
    where
        T: Serialize,
    {
        self.write_token(Token::NewtypeStruct { name })?;
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<(), Error>
    where
        T: Serialize,
    {
        self.write_token(Token::NewtypeVariant { name, variant })?;
        value.serialize(self)
    }

    fn serialize_none(self) -> Result<(), Error> {
        Ok(())
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<(), Error>
    where
        T: Serialize,
    {
        self.write_token(Token::Some)?;
        value.serialize(self)
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Compound<'a>, Error> {
        self.write_token(Token::Seq { len })?;
        Ok(Compound {
            ser: self.clone(),
            end: Token::SeqEnd,
        })
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Error> {
        self.write_token(Token::Tuple { len })?;
        Ok(Compound {
            ser: self.clone(),
            end: Token::TupleEnd,
        })
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Error> {
        self.write_token(Token::TupleStruct { name, len })?;
        Ok(Compound {
            ser: self.clone(),
            end: Token::TupleStructEnd,
        })
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Error> {
        self.write_token(Token::TupleVariant { name, variant, len })?;
        Ok(Compound {
            ser: self.clone(),
            end: Token::TupleVariantEnd,
        })
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Error> {
        self.write_token(Token::Map { len })?;
        Ok(Compound {
            ser: self.clone(),
            end: Token::MapEnd,
        })
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Error> {
        self.write_token(Token::Struct { name, len })?;
        Ok(Compound {
            ser: self.clone(),
            end: Token::StructEnd,
        })
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Error> {
        self.write_token(Token::StructVariant { name, variant, len })?;
        Ok(Compound {
            ser: self.clone(),
            end: Token::StructVariantEnd,
        })
    }

    fn is_human_readable(&self) -> bool {
        true
    }
}

///
pub struct Compound<'a> {
    ser: Tokenizer<'a>,
    end: Token<'a>,
}

impl<'a> Compound<'a> {
    fn do_end(mut self) -> Result<(), Error> {
        self.ser.write_token(self.end)?;
        Ok(())
    }
}

impl<'a> ser::SerializeSeq for Compound<'a> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Error>
    where
        T: Serialize,
    {
        value.serialize(&mut self.ser)
    }

    fn end(self) -> Result<(), Error> {
        self.do_end()
    }
}

impl<'a> ser::SerializeTuple for Compound<'a> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Error>
    where
        T: Serialize,
    {
        value.serialize(&mut self.ser)
    }

    fn end(self) -> Result<(), Error> {
        self.do_end()
    }
}

impl<'a> ser::SerializeTupleStruct for Compound<'a> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Error>
    where
        T: Serialize,
    {
        value.serialize(&mut self.ser)
    }

    fn end(self) -> Result<(), Error> {
        self.do_end()
    }
}

impl<'a> ser::SerializeTupleVariant for Compound<'a> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Error>
    where
        T: Serialize,
    {
        value.serialize(&mut self.ser)
    }

    fn end(self) -> Result<(), Error> {
        self.do_end()
    }
}

impl<'a> ser::SerializeMap for Compound<'a> {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        key.serialize(&mut self.ser)
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        value.serialize(&mut self.ser)
    }

    fn end(self) -> Result<(), Self::Error> {
        self.do_end()
    }
}

impl<'a> ser::SerializeStruct for Compound<'a> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        key.serialize(&mut self.ser)?;
        value.serialize(&mut self.ser)
    }

    fn end(self) -> Result<(), Self::Error> {
        self.do_end()
    }
}

impl<'a> ser::SerializeStructVariant for Compound<'a> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        key.serialize(&mut self.ser)?;
        value.serialize(&mut self.ser)
    }

    fn end(self) -> Result<(), Self::Error> {
        self.do_end()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
