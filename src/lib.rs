mod de;
mod error;
mod ser;

#[derive(Clone, Debug)]
pub enum Token<'a> {
    /// A serialized `bool`.
    Bool(bool),

    /// A serialized `i8`.
    I8(i8),

    /// A serialized `i16`.
    I16(i16),

    /// A serialized `i32`.
    I32(i32),

    /// A serialized `i64`.
    I64(i64),

    /// A serialized `i128`.
    I128(i128),

    /// A serialized `u8`.
    U8(u8),

    /// A serialized `u16`.
    U16(u16),

    /// A serialized `u32`.
    U32(u32),

    /// A serialized `u64`.
    U64(u64),

    /// A serialized `u128`.
    U128(u128),

    /// A serialized `f32`.
    F32(f32),

    /// A serialized `f64`.
    F64(f64),

    /// A serialized `char`.
    Char(char),

    /// A serialized `str`.
    Str(&'a str),

    /// A borrowed `str`.
    BorrowedStr(&'a str),

    /// A borrowed `[u8]`.
    BorrowedBytes(&'a [u8]),

    /// A serialized `Option<T>` containing none.
    None,

    /// The header to a serialized `Option<T>` containing some value.
    Some,

    /// A serialized `()`.
    Unit,

    /// A serialized unit struct of the given name.
    UnitStruct { name: &'a str },

    /// A unit variant of an enum.
    UnitVariant { name: &'a str, variant: &'a str },

    /// The header to a serialized newtype struct of the given name.
    NewtypeStruct { name: &'a str },

    /// The header to a newtype variant of an enum.
    NewtypeVariant { name: &'a str, variant: &'a str },

    /// The header to a sequence.
    Seq { len: Option<usize> },

    /// An indicator of the end of a sequence.
    SeqEnd,

    /// The header to a tuple.
    Tuple { len: usize },

    /// An indicator of the end of a tuple.
    TupleEnd,

    /// The header to a tuple struct.
    TupleStruct { name: &'a str, len: usize },

    /// An indicator of the end of a tuple struct.
    TupleStructEnd,

    /// The header to a tuple variant of an enum.
    TupleVariant {
        name: &'a str,
        variant: &'a str,
        len: usize,
    },

    /// An indicator of the end of a tuple variant.
    TupleVariantEnd,

    /// The header to a map.
    Map { len: Option<usize> },

    /// An indicator of the end of a map.
    MapEnd,

    /// The header of a struct.
    Struct { name: &'a str, len: usize },

    /// An indicator of the end of a struct.
    StructEnd,

    /// The header of a struct variant of an enum.
    StructVariant {
        name: &'a str,
        variant: &'a str,
        len: usize,
    },

    /// An indicator of the end of a struct variant.
    StructVariantEnd,

    /// The header to an enum of the given name.
    Enum { name: &'a str },
}
