mod mutable;

use std::sync::Arc;

use re_arrow2::{
    array::*,
    bitmap::Bitmap,
    datatypes::{DataType, Field},
};

fn data() -> FixedSizeListArray {
    let values = Int32Array::from_slice([10, 20, 0, 0]);

    FixedSizeListArray::try_new(
        DataType::FixedSizeList(
            Arc::new(Field::new("a", values.data_type().clone(), true)),
            2,
        ),
        values.boxed(),
        Some([true, false].into()),
    )
    .unwrap()
}

#[test]
fn basics() {
    let array = data();
    assert_eq!(array.size(), 2);
    assert_eq!(array.len(), 2);
    assert_eq!(array.validity(), Some(&Bitmap::from([true, false])));

    assert_eq!(array.value(0).as_ref(), Int32Array::from_slice([10, 20]));
    assert_eq!(array.value(1).as_ref(), Int32Array::from_slice([0, 0]));

    let array = array.sliced(1, 1);

    assert_eq!(array.value(0).as_ref(), Int32Array::from_slice([0, 0]));
}

#[test]
fn with_validity() {
    let array = data();

    let a = array.with_validity(None);
    assert!(a.validity().is_none());
}

#[test]
fn debug() {
    let array = data();

    assert_eq!(format!("{array:?}"), "FixedSizeListArray[[10, 20], None]");
}

#[test]
fn empty() {
    let array = FixedSizeListArray::new_empty(DataType::FixedSizeList(
        Arc::new(Field::new("a", DataType::Int32, true)),
        2,
    ));
    assert_eq!(array.values().len(), 0);
    assert_eq!(array.validity(), None);
}

#[test]
fn null() {
    let array = FixedSizeListArray::new_null(
        DataType::FixedSizeList(
            std::sync::Arc::new(Field::new("a", DataType::Int32, true)),
            2,
        ),
        2,
    );
    assert_eq!(array.values().len(), 4);
    assert_eq!(array.validity().cloned(), Some([false, false].into()));
}

#[test]
fn wrong_size() {
    let values = Int32Array::from_slice([10, 20, 0]);
    assert!(FixedSizeListArray::try_new(
        DataType::FixedSizeList(
            std::sync::Arc::new(Field::new("a", DataType::Int32, true)),
            2
        ),
        values.boxed(),
        None
    )
    .is_err());
}

#[test]
fn wrong_len() {
    let values = Int32Array::from_slice([10, 20, 0]);
    assert!(FixedSizeListArray::try_new(
        DataType::FixedSizeList(
            std::sync::Arc::new(Field::new("a", DataType::Int32, true)),
            2
        ),
        values.boxed(),
        Some([true, false, false].into()), // it should be 2
    )
    .is_err());
}

#[test]
fn wrong_data_type() {
    let values = Int32Array::from_slice([10, 20, 0]);
    assert!(FixedSizeListArray::try_new(
        DataType::Binary,
        values.boxed(),
        Some([true, false, false].into()), // it should be 2
    )
    .is_err());
}
