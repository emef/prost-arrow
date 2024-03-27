use std::any::Any;
use std::sync::Arc;

use arrow_array::builder::{ArrayBuilder, BinaryBuilder, StringBuilder};
use arrow_array::ArrayRef;
use arrow_schema::DataType;
use prost::alloc::string::String;

use crate::traits::{ArrowBuilder, ToArrow};

impl ToArrow for String {
    type Item = String;
    type Builder = ArrowStringBuilder;

    fn to_datatype() -> DataType {
        DataType::Utf8
    }
}

impl ToArrow for Vec<u8> {
    type Item = Vec<u8>;
    type Builder = ArrowBinaryBuilder;

    fn to_datatype() -> DataType {
        DataType::Binary
    }
}

pub struct ArrowStringBuilder {
    builder: StringBuilder,
}

impl ArrowBuilder<String> for ArrowStringBuilder {
    fn new_with_capacity(capacity: usize) -> Self {
        Self {
            builder: StringBuilder::with_capacity(capacity, capacity * 1024),
        }
    }

    fn append(&mut self, value: Option<String>) {
        self.builder.append_option(value)
    }
}

impl ArrayBuilder for ArrowStringBuilder {
    fn len(&self) -> usize {
        self.builder.len()
    }

    fn finish(&mut self) -> ArrayRef {
        Arc::new(self.builder.finish())
    }

    fn finish_cloned(&self) -> ArrayRef {
        Arc::new(self.builder.finish_cloned())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn into_box_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }
}

pub struct ArrowBinaryBuilder {
    builder: BinaryBuilder,
}

impl ArrowBuilder<Vec<u8>> for ArrowBinaryBuilder {
    fn new_with_capacity(capacity: usize) -> Self {
        Self {
            builder: BinaryBuilder::with_capacity(capacity, capacity * 1024),
        }
    }

    fn append(&mut self, value: Option<Vec<u8>>) {
        self.builder.append_option(value)
    }
}

impl ArrayBuilder for ArrowBinaryBuilder {
    fn len(&self) -> usize {
        self.builder.len()
    }

    fn finish(&mut self) -> ArrayRef {
        Arc::new(self.builder.finish())
    }

    fn finish_cloned(&self) -> ArrayRef {
        Arc::new(self.builder.finish_cloned())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn into_box_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }
}
