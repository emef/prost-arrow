use std::any::Any;
use std::sync::Arc;

use arrow_array::builder::{ArrayBuilder, ListBuilder};
use arrow_array::ArrayRef;

use crate::traits::{ArrowBuilder, ToArrow};

pub struct ArrowListBuilder<T: ToArrow<Item = T>> {
    builder: ListBuilder<T::Builder>,
}

impl<T: ToArrow<Item = T>> ArrowBuilder<Vec<T>> for ArrowListBuilder<T>
where
    T: 'static,
{
    fn new_with_capacity(capacity: usize) -> Self {
        Self {
            builder: ListBuilder::<T::Builder>::with_capacity(
                T::Builder::new_with_capacity(capacity),
                capacity,
            ),
        }
    }

    fn append(&mut self, value: Option<Vec<T>>) {
        match value {
            Some(vs) => {
                let values = self.builder.values();
                for v in vs {
                    values.append(Some(v));
                }
                self.builder.append(true);
            }
            None => self.builder.append(false),
        }
    }
}

impl<T: ToArrow<Item = T>> ArrayBuilder for ArrowListBuilder<T>
where
    T: 'static,
{
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
