use candle::{DType, Device, Result, Tensor};

#[test]
fn add() -> Result<()> {
    let tensor = Tensor::zeros(&[5, 2], DType::F32, Device::Cpu);
    let (dim1, dim2) = tensor.shape2()?;
    assert_eq!(dim1, 5);
    assert_eq!(dim2, 2);
    Ok(())
}
