use crate::{error::Error, Token};
use futures::sink::Sink;
use serde::{
    de,
    ser::{self, Serialize},
};
use serde_transcode::transcode;

/// Transcodes a deserializer into a [`futures::Sink`] of `Token`s.
///
/// NOTE: currenty uses `unsafe` twice: for coercing an `'de` lifetime on the deserialized and borrowed `&[u8]` or `&str`. I believe this is safe because the `Tokenizer` is only used here, and always with an associated [`Deserializer<'de>`].
///
/// [`Sink`]: https://docs.rs/futures/0.1.27/futures/sink/trait.Sink.html
/// [`Deserializer<'de>`]: https://docs.serde.rs/serde/trait.Deserializer.html
pub fn tokenize<'de, D, S>(deserializer: D, sink: S) -> Result<(), Error>
where
    D: de::Deserializer<'de>,
    S: Sink<SinkItem = Token<'de>>,
{
    let mut ser = Tokenizer(sink);
    transcode(deserializer, &mut ser)
}

#[derive(Clone, Debug)]
struct Tokenizer<'a, S: Sink<SinkItem = Token<'a>>>(S);

impl<'a, S: Sink<SinkItem = Token<'a>>> Tokenizer<'a, S> {
    fn write_token(&mut self, token: Token<'a>) -> Result<(), Error> {
        use futures::AsyncSink;
        self.0
            .start_send(token)
            .map_err(|_| Error::TokenSinkError)
            .and_then(|sink| match sink {
                AsyncSink::Ready => Ok(()),
                AsyncSink::NotReady(_) => Err(Error::TokenSinkNotReadyError),
            })
    }
}

impl<'s, 'a: 's, S: Sink<SinkItem = Token<'a>>> ser::Serializer for &'s mut Tokenizer<'a, S> {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = CompoundTokenizer<'a, 's, S>;
    type SerializeTuple = CompoundTokenizer<'a, 's, S>;
    type SerializeTupleStruct = CompoundTokenizer<'a, 's, S>;
    type SerializeTupleVariant = CompoundTokenizer<'a, 's, S>;
    type SerializeMap = CompoundTokenizer<'a, 's, S>;
    type SerializeStruct = CompoundTokenizer<'a, 's, S>;
    type SerializeStructVariant = CompoundTokenizer<'a, 's, S>;

    #[inline]
    fn serialize_bool(self, v: bool) -> Result<(), Error> {
        self.write_token(Token::Bool(v))?;
        Ok(())
    }

    #[inline]
    fn serialize_i8(self, v: i8) -> Result<(), Error> {
        self.write_token(Token::I8(v))?;
        Ok(())
    }

    #[inline]
    fn serialize_i16(self, v: i16) -> Result<(), Error> {
        self.write_token(Token::I16(v))?;
        Ok(())
    }

    #[inline]
    fn serialize_i32(self, v: i32) -> Result<(), Error> {
        self.write_token(Token::I32(v))?;
        Ok(())
    }

    #[inline]
    fn serialize_i64(self, v: i64) -> Result<(), Error> {
        self.write_token(Token::I64(v))?;
        Ok(())
    }

    #[inline]
    fn serialize_i128(self, v: i128) -> Result<(), Error> {
        self.write_token(Token::I128(v))?;
        Ok(())
    }

    #[inline]
    fn serialize_u8(self, v: u8) -> Result<(), Error> {
        self.write_token(Token::U8(v))?;
        Ok(())
    }

    #[inline]
    fn serialize_u16(self, v: u16) -> Result<(), Error> {
        self.write_token(Token::U16(v))?;
        Ok(())
    }

    #[inline]
    fn serialize_u32(self, v: u32) -> Result<(), Error> {
        self.write_token(Token::U32(v))?;
        Ok(())
    }

    #[inline]
    fn serialize_u64(self, v: u64) -> Result<(), Error> {
        self.write_token(Token::U64(v))?;
        Ok(())
    }

    #[inline]
    fn serialize_u128(self, v: u128) -> Result<(), Error> {
        self.write_token(Token::U128(v))?;
        Ok(())
    }

    #[inline]
    fn serialize_f32(self, v: f32) -> Result<(), Error> {
        self.write_token(Token::F32(v))?;
        Ok(())
    }

    #[inline]
    fn serialize_f64(self, v: f64) -> Result<(), Error> {
        self.write_token(Token::F64(v))?;
        Ok(())
    }

    #[inline]
    fn serialize_char(self, v: char) -> Result<(), Error> {
        self.write_token(Token::Char(v))?;
        Ok(())
    }

    #[inline]
    fn serialize_str(self, v: &str) -> Result<(), Error> {
        let new_v = unsafe { std::mem::transmute::<&str, &'a str>(v) };
        self.write_token(Token::Str(new_v))?;
        Ok(())
    }

    #[inline]
    fn serialize_bytes(self, v: &[u8]) -> Result<(), Self::Error> {
        let new_v = unsafe { std::mem::transmute::<&[u8], &'a [u8]>(v) };
        self.write_token(Token::Bytes(new_v))?;
        Ok(())
    }

    #[inline]
    fn serialize_unit(self) -> Result<(), Error> {
        self.write_token(Token::Unit)?;
        Ok(())
    }

    #[inline]
    fn serialize_unit_struct(self, name: &'static str) -> Result<(), Error> {
        self.write_token(Token::UnitStruct { name })?;
        Ok(())
    }

    #[inline]
    fn serialize_unit_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<(), Error> {
        self.write_token(Token::UnitVariant { name, variant })?;
        Ok(())
    }

    #[inline]
    fn serialize_newtype_struct<T: ?Sized>(self, name: &'static str, value: &T) -> Result<(), Error>
    where
        T: Serialize,
    {
        self.write_token(Token::NewtypeStruct { name })?;
        value.serialize(self)
    }

    #[inline]
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

    #[inline]
    fn serialize_none(self) -> Result<(), Error> {
        Ok(())
    }

    #[inline]
    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<(), Error>
    where
        T: Serialize,
    {
        self.write_token(Token::Some)?;
        value.serialize(self)
    }

    #[inline]
    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Error> {
        self.write_token(Token::Seq { len })?;
        Ok(CompoundTokenizer {
            ser: self,
            end: Token::SeqEnd,
        })
    }

    #[inline]
    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Error> {
        self.write_token(Token::Tuple { len })?;
        Ok(CompoundTokenizer {
            ser: self,
            end: Token::TupleEnd,
        })
    }

    #[inline]
    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Error> {
        self.write_token(Token::TupleStruct { name, len })?;
        Ok(CompoundTokenizer {
            ser: self,
            end: Token::TupleStructEnd,
        })
    }

    #[inline]
    fn serialize_tuple_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Error> {
        self.write_token(Token::TupleVariant { name, variant, len })?;
        Ok(CompoundTokenizer {
            ser: self,
            end: Token::TupleVariantEnd,
        })
    }

    #[inline]
    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Error> {
        self.write_token(Token::Map { len })?;
        Ok(CompoundTokenizer {
            ser: self,
            end: Token::MapEnd,
        })
    }

    #[inline]
    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Error> {
        self.write_token(Token::Struct { name, len })?;
        Ok(CompoundTokenizer {
            ser: self,
            end: Token::StructEnd,
        })
    }

    #[inline]
    fn serialize_struct_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Error> {
        self.write_token(Token::StructVariant { name, variant, len })?;
        Ok(CompoundTokenizer {
            ser: self,
            end: Token::StructVariantEnd,
        })
    }

    #[inline]
    fn is_human_readable(&self) -> bool {
        true
    }
}

struct CompoundTokenizer<'a, 's, S>
where
    S: Sink<SinkItem = Token<'a>>,
{
    ser: &'s mut Tokenizer<'a, S>,
    end: Token<'a>,
}

impl<'s, 'a: 's, S> CompoundTokenizer<'a, 's, S>
where
    S: Sink<SinkItem = Token<'a>>,
{
    fn do_end(self) -> Result<(), Error> {
        self.ser.write_token(self.end)?;
        Ok(())
    }
}

impl<'a, 's, S> ser::SerializeSeq for CompoundTokenizer<'a, 's, S>
where
    S: Sink<SinkItem = Token<'a>>,
{
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Error>
    where
        T: Serialize,
    {
        value.serialize(&mut *self.ser)
    }

    fn end(self) -> Result<(), Error> {
        self.do_end()
    }
}

impl<'a, 's, S> ser::SerializeTuple for CompoundTokenizer<'a, 's, S>
where
    S: Sink<SinkItem = Token<'a>>,
{
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Error>
    where
        T: Serialize,
    {
        value.serialize(&mut *self.ser)
    }

    fn end(self) -> Result<(), Error> {
        self.do_end()
    }
}

impl<'a, 's, S> ser::SerializeTupleStruct for CompoundTokenizer<'a, 's, S>
where
    S: Sink<SinkItem = Token<'a>>,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Error>
    where
        T: Serialize,
    {
        value.serialize(&mut *self.ser)
    }

    fn end(self) -> Result<(), Error> {
        self.do_end()
    }
}

impl<'a, 's, S> ser::SerializeTupleVariant for CompoundTokenizer<'a, 's, S>
where
    S: Sink<SinkItem = Token<'a>>,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Error>
    where
        T: Serialize,
    {
        value.serialize(&mut *self.ser)
    }

    fn end(self) -> Result<(), Error> {
        self.do_end()
    }
}

impl<'a, 's, S> ser::SerializeMap for CompoundTokenizer<'a, 's, S>
where
    S: Sink<SinkItem = Token<'a>>,
{
    type Ok = ();
    type Error = Error;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        key.serialize(&mut *self.ser)
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        value.serialize(&mut *self.ser)
    }

    fn end(self) -> Result<(), Self::Error> {
        self.do_end()
    }
}

impl<'a, 's, S> ser::SerializeStruct for CompoundTokenizer<'a, 's, S>
where
    S: Sink<SinkItem = Token<'a>>,
{
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
        key.serialize(&mut *self.ser)?;
        value.serialize(&mut *self.ser)
    }

    fn end(self) -> Result<(), Self::Error> {
        self.do_end()
    }
}

impl<'a, 's, S> ser::SerializeStructVariant for CompoundTokenizer<'a, 's, S>
where
    S: Sink<SinkItem = Token<'a>>,
{
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
        key.serialize(&mut *self.ser)?;
        value.serialize(&mut *self.ser)
    }

    fn end(self) -> Result<(), Self::Error> {
        self.do_end()
    }
}
