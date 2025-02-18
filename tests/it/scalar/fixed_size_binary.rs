use re_arrow2::{
    datatypes::DataType,
    scalar::{FixedSizeBinaryScalar, Scalar},
};

#[allow(clippy::eq_op)]
#[test]
fn equal() {
    let a = FixedSizeBinaryScalar::new(DataType::FixedSizeBinary(1), Some("a"));
    let b = FixedSizeBinaryScalar::new(DataType::FixedSizeBinary(1), None::<&str>);
    assert_eq!(a, a);
    assert_eq!(b, b);
    assert!(a != b);
    let b = FixedSizeBinaryScalar::new(DataType::FixedSizeBinary(1), Some("b"));
    assert!(a != b);
    assert_eq!(b, b);
}

#[test]
fn basics() {
    let a = FixedSizeBinaryScalar::new(DataType::FixedSizeBinary(1), Some("a"));

    assert_eq!(a.value(), Some(b"a".as_ref()));
    assert_eq!(a.data_type(), &DataType::FixedSizeBinary(1));
    assert!(a.is_valid());

    let _: &dyn std::any::Any = a.as_any();
}
