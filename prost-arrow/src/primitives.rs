use std::any::Any;
use std::sync::Arc;

use arrow_array::builder::{ArrayBuilder, PrimitiveBuilder};
use arrow_array::{ArrayRef, ArrowPrimitiveType};
use arrow_schema::DataType;

use crate::traits::{ArrowBuilder, ToArrow};

macro_rules! make_impl {
    ($native_ty:ty, $data_ty:expr, $array_ty:ty) => {
        impl ToArrow for $native_ty {
            type Item = $native_ty;
            type Builder = PrimitiveArrowBuilder<$array_ty>;

            fn to_datatype() -> DataType {
                return $data_ty;
            }
        }
    };
}

make_impl!(i32, DataType::Int32, arrow_array::types::Int32Type);
make_impl!(i64, DataType::Int64, arrow_array::types::Int64Type);
make_impl!(u32, DataType::UInt32, arrow_array::types::UInt32Type);
make_impl!(u64, DataType::UInt64, arrow_array::types::UInt64Type);
make_impl!(f32, DataType::Float32, arrow_array::types::Float32Type);
make_impl!(f64, DataType::Float64, arrow_array::types::Float64Type);

pub struct PrimitiveArrowBuilder<T: ArrowPrimitiveType> {
    builder: PrimitiveBuilder<T>,
}

impl<T: ArrowPrimitiveType> ArrowBuilder<T::Native> for PrimitiveArrowBuilder<T> {
    fn new_with_capacity(capacity: usize) -> Self {
        Self {
            builder: PrimitiveBuilder::with_capacity(capacity),
        }
    }

    fn append_value(&mut self, value: T::Native) {
        self.builder.append_value(value)
    }

    fn append_option(&mut self, value: Option<T::Native>) {
        self.builder.append_option(value)
    }
}

impl<T: ArrowPrimitiveType> ArrayBuilder for PrimitiveArrowBuilder<T> {
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
