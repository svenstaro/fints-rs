//! Serialization.

use serde::ser::{self, Serialize};
use std::fmt::{self, Display};
use std::str;

#[derive(Clone, Debug, PartialEq)]
pub struct Error(String);
type Result<T> = std::result::Result<T, Error>;

impl ser::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Error(msg.to_string())
    }
}

impl Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(std::error::Error::description(self))
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match self {
            Error(ref msg) => msg,
        }
    }
}

pub struct Serializer {
    /// The final serialized output string.
    output: String,

    /// Keep track of whether we're currently inside a DEG or not.
    /// This is necessary because DEGs are delmited differently than DEs.
    inside_deg: bool,

    /// The current field's index in the current struct.
    field_index_in_struct: u32,

    /// Total size of the last struct.
    last_struct_size: usize,

    /// Stack of struct names.
    /// We need this to be able to correctly count recursively nested structs.
    struct_stack: Vec<String>,

    /// Total number of elements in the current segment.
    /// This is set to the size of the struct every time we pass a new segment.
    /// This is important to know so we know when a segment ends.
    current_segment_elements_count: usize,

    /// The element position inside the current segment.
    current_segment_index: u32,
}

pub fn to_string<T>(value: &T) -> Result<String>
where
    T: Serialize,
{
    let mut serializer = Serializer {
        output: String::new(),
        inside_deg: false,
        field_index_in_struct: 0,
        last_struct_size: 0,
        struct_stack: vec![],
        current_segment_elements_count: 0,
        current_segment_index: 0,
    };
    value.serialize(&mut serializer)?;
    Ok(serializer.output)
}

impl<'a> ser::Serializer for &'a mut Serializer {
    type Ok = ();
    type Error = Error;
    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, _v: bool) -> Result<()> {
        unimplemented!();
    }

    fn serialize_i8(self, v: i8) -> Result<()> {
        self.output += &v.to_string();
        Ok(())
    }

    fn serialize_i16(self, v: i16) -> Result<()> {
        self.output += &v.to_string();
        Ok(())
    }

    fn serialize_i32(self, v: i32) -> Result<()> {
        self.output += &v.to_string();
        Ok(())
    }

    fn serialize_i64(self, v: i64) -> Result<()> {
        self.output += &v.to_string();
        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<()> {
        self.output += &v.to_string();
        Ok(())
    }

    fn serialize_u16(self, v: u16) -> Result<()> {
        self.output += &v.to_string();
        Ok(())
    }

    fn serialize_u32(self, v: u32) -> Result<()> {
        self.output += &v.to_string();
        Ok(())
    }

    fn serialize_u64(self, v: u64) -> Result<()> {
        self.output += &v.to_string();
        Ok(())
    }

    fn serialize_f32(self, _v: f32) -> Result<()> {
        unimplemented!();
    }

    fn serialize_f64(self, _v: f64) -> Result<()> {
        unimplemented!()
    }

    fn serialize_char(self, v: char) -> Result<()> {
        self.output += &v.to_string();
        Ok(())
    }

    fn serialize_str(self, v: &str) -> Result<()> {
        self.output += v;
        Ok(())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<()> {
        self.output += str::from_utf8(v).map_err(|x| Error(x.to_string()))?;
        Ok(())
    }

    fn serialize_none(self) -> Result<()> {
        Ok(())
    }

    fn serialize_some<T>(self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut *self)?;
        Ok(())
    }

    fn serialize_unit(self) -> Result<()> {
        unimplemented!()
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        unimplemented!()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<()> {
        Ok(())
    }

    fn serialize_newtype_struct<T>(self, _name: &'static str, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!()
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!()
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        unimplemented!()
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        unimplemented!()
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        unimplemented!()
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        unimplemented!()
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        unimplemented!()
    }

    fn serialize_struct(self, name: &'static str, len: usize) -> Result<Self::SerializeStruct> {
        self.field_index_in_struct = 0;
        self.last_struct_size = len;
        self.struct_stack.push(name.to_string());

        // If this is a new segment, set the length so that we also know when we're finished with
        // this particular segment.
        if name.starts_with("Seg") {
            self.current_segment_elements_count = len;
            self.current_segment_index = 0;
        }

        // Keep track of when a DEG starts since DEs inside of a DEG are delimited using `:`.
        // Outside of oa DEG, DEs are delimited using `+`.
        if name.starts_with("DEG") {
            self.inside_deg = true;
        } else {
            self.inside_deg = false;
        }

        Ok(self)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        unimplemented!()
    }
}

impl<'a> ser::SerializeStruct for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        // If the parent struct containing this field is a segment, we'll increase the current
        // segment counter.
        if let Some(last) = self.struct_stack.last() {
            if last.starts_with("Seg") {
                self.current_segment_index += 1;
            }
        }

        if self.field_index_in_struct != 0 {
            if self.inside_deg {
                self.output += ":";
            } else {
                self.output += "+";
            }
        }
        self.field_index_in_struct += 1;
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        // if self.current_segment_index == self.current_segment_elements_count as u32
        //     && self.struct_stack.last().unwrap().starts_with("Seg")
        // {
        //     self.output += "|";
        // }

        // In case this is a segment, we have to terminate it with `'`.
        if let Some(last) = self.struct_stack.last() {
            if last.starts_with("Seg") && !self.output.is_empty() {
                self.output += "'";
            }
        }

        // If we read the last field in this struct, we should reset the data we set for this
        // struct.
        if self.field_index_in_struct >= self.last_struct_size as u32 {
            self.inside_deg = false;
        }

        // This marks the end of a parsed struct so we have to pop it from the stack at the very
        // end.
        self.struct_stack.pop();

        Ok(())
    }
}

impl<'a> ser::SerializeSeq for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!()
    }
    fn end(self) -> Result<()> {
        unimplemented!()
    }
}

impl<'a> ser::SerializeTuple for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!()
    }
    fn end(self) -> Result<()> {
        unimplemented!()
    }
}

impl<'a> ser::SerializeTupleStruct for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!()
    }
    fn end(self) -> Result<()> {
        unimplemented!()
    }
}

impl<'a> ser::SerializeTupleVariant for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!()
    }
    fn end(self) -> Result<()> {
        unimplemented!()
    }
}

impl<'a> ser::SerializeMap for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T>(&mut self, _key: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!()
    }
    fn serialize_value<T>(&mut self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!()
    }
    fn end(self) -> Result<()> {
        unimplemented!()
    }
}

impl<'a> ser::SerializeStructVariant for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _key: &'static str, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!()
    }
    fn end(self) -> Result<()> {
        unimplemented!()
    }
}
