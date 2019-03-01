//! Serialization.

use log::{debug, info, trace};
use serde::ser::{self, Serialize};
use std::fmt::{self, Debug, Display};
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

    /// For pretty printing the Serialization tree.
    tree_builder: ptree::TreeBuilder,
}

pub fn to_string<T>(value: &T) -> Result<String>
where
    T: Serialize + Debug,
{
    // It's not stupid if it works!
    let debug_struct = format!("{:#?}", value);
    let struct_name = debug_struct.split_whitespace().next().unwrap_or("Unknown");
    info!("Serializing: {}", struct_name);
    trace!("\n{:#?}", value);

    let mut serializer = Serializer {
        output: String::new(),
        inside_deg: false,
        field_index_in_struct: 0,
        last_struct_size: 0,
        struct_stack: vec![],
        current_segment_elements_count: 0,
        current_segment_index: 0,
        tree_builder: ptree::TreeBuilder::new("\nSerialize".to_string()),
    };
    value.serialize(&mut serializer)?;

    let mut tree_buf = Vec::new();
    ptree::write_tree(&serializer.tree_builder.build(), &mut tree_buf)
        .expect("Error printing serialization debug tree");
    trace!("{}", std::str::from_utf8(&tree_buf).unwrap());

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
        variant: &'static str,
    ) -> Result<()> {
        variant.serialize(&mut *self)?;
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
        if name.starts_with("Msg") {
            self.tree_builder.begin_child(format!("Message {}", name));
        } else if name.starts_with("Seg") {
            self.tree_builder.begin_child(format!("Segment {}", name));
        } else if name.starts_with("DEG") {
            self.tree_builder.begin_child(format!("DEG {}", name));
        };

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

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        // If the parent struct containing this field is a segment, we'll increase the current
        // segment counter.
        if let Some(last) = self.struct_stack.last() {
            if last.starts_with("Seg") {
                self.current_segment_index += 1;

                // If the parent is a Segment, we're not in a DEG.
                // This will also reset inside_deg when ascending from a DEG.
                // This will be set to true every time we descend into a struct
                // whose name starts with DEG (see `serialize_struct`)
                self.inside_deg = false;
            }
        }

        // Do not separate segments using delimiters.
        if self.struct_stack.iter().any(|x| x.starts_with("Seg")) {
            if self.field_index_in_struct != 0 {
                if self.inside_deg {
                    self.output += ":";
                } else {
                    self.output += "+";
                }
            }
        }
        self.field_index_in_struct += 1;
        self.tree_builder.add_empty_child(format!("DE {}", key));
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        // In case this is a segment, we have to terminate it with `'`.
        if let Some(last) = self.struct_stack.last() {
            if last.starts_with("Seg") && !self.output.is_empty() {
                self.output += "'\n";
            }
        }

        // This marks the end of a parsed struct so we have to pop it from the stack at the very
        // end.
        self.struct_stack.pop();
        self.tree_builder.end_child();

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
