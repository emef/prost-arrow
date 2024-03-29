use arrow_array::builder::ArrayBuilder;
use arrow_schema::DataType;

pub trait ToArrow {
    type Item;
    type Builder: ArrowBuilder<Self::Item> + ArrayBuilder;

    fn to_datatype() -> DataType;
}

pub trait ArrowBuilder<T> {
    fn new_with_capacity(capacity: usize) -> Self;
    fn append_value(&mut self, value: T);
    fn append_option(&mut self, value: Option<T>);
}
