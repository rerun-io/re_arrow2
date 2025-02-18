use re_arrow2::array::*;
use re_arrow2::compute::if_then_else::if_then_else;
use re_arrow2::error::Result;

#[test]
fn basics() -> Result<()> {
    let lhs = Int32Array::from_slice([1, 2, 3]);
    let rhs = Int32Array::from_slice([4, 5, 6]);
    let predicate = BooleanArray::from_slice(vec![true, false, true]);
    let c = if_then_else(&predicate, &lhs, &rhs)?;

    let expected = Int32Array::from_slice([1, 5, 3]);

    assert_eq!(expected, c.as_ref());
    Ok(())
}

#[test]
fn basics_nulls() -> Result<()> {
    let lhs = Int32Array::from(&[Some(1), None, None]);
    let rhs = Int32Array::from(&[None, Some(5), Some(6)]);
    let predicate = BooleanArray::from_slice(vec![true, false, true]);
    let c = if_then_else(&predicate, &lhs, &rhs)?;

    let expected = Int32Array::from(&[Some(1), Some(5), None]);

    assert_eq!(expected, c.as_ref());
    Ok(())
}

#[test]
fn basics_nulls_pred() -> Result<()> {
    let lhs = Int32Array::from_slice([1, 2, 3]);
    let rhs = Int32Array::from_slice([4, 5, 6]);
    let predicate = BooleanArray::from(&[Some(true), None, Some(false)]);
    let result = if_then_else(&predicate, &lhs, &rhs)?;

    let expected = Int32Array::from(&[Some(1), None, Some(6)]);

    assert_eq!(expected, result.as_ref());
    Ok(())
}
