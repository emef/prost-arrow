use arrow_array::builder::ArrayBuilder;
use arrow_array::{RecordBatch, StructArray};
use prost_arrow::{ArrowBuilder, ToArrow};
use routeguide::{Point, Rectangle};

pub mod routeguide {
    include!(concat!(env!("OUT_DIR"), "/routeguide.rs"));
}

fn main() {
    let pt_1 = Point {
        latitude: 11,
        longitude: 20,
    };

    let pt_2 = Point {
        latitude: 3,
        longitude: 100,
    };

    let datatype = Rectangle::to_datatype();

    let mut builder = prost_arrow::new_builder::<Rectangle>();

    builder.append_option(Some(Rectangle {
        lo: Some(pt_1),
        hi: None,
        messages: vec!["one".to_string(), "two".to_string()],
        extra_points: vec![
            Point {
                latitude: 1,
                longitude: 2,
            },
            Point {
                latitude: 3,
                longitude: 4,
            },
        ],
        binary: vec![0, 1, 2, 3],
        repeated_binary: vec![vec![10, 100]],
    }));

    builder.append_value(Rectangle {
        lo: Some(pt_2),
        hi: None,
        messages: vec!["three".to_string()],
        extra_points: vec![],
        binary: vec![4, 5, 6, 7],
        repeated_binary: vec![vec![5, 50]],
    });

    let arr = builder.finish();
    let struct_arr = arr.as_any().downcast_ref::<StructArray>().unwrap();

    println!("Hello, world! {datatype:?}");
    println!("rectangles: {:?}", struct_arr);

    let record_batch: RecordBatch = struct_arr.into();

    println!("as record batch: {:?}", record_batch)
}
