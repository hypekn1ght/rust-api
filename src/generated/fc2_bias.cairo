use array::ArrayTrait;
use orion::operators::tensor::core::{TensorTrait, Tensor, ExtraParams};
use orion::operators::tensor::implementations::impl_tensor_i32::Tensor_i32;
use orion::numbers::fixed_point::core::FixedImpl;
use orion::numbers::signed_integer::i32::i32;

fn fc2_bias() -> Tensor<i32> {
    let mut shape = ArrayTrait::<usize>::new();
    shape.append(10);
    let mut data = ArrayTrait::<i32>::new();
    data.append(i32 { mag: 30, sign: false });
    data.append(i32 { mag: 426, sign: false });
    data.append(i32 { mag: 7, sign: false });
    data.append(i32 { mag: 301, sign: true });
    data.append(i32 { mag: 249, sign: false });
    data.append(i32 { mag: 852, sign: false });
    data.append(i32 { mag: 317, sign: true });
    data.append(i32 { mag: 63, sign: false });
    data.append(i32 { mag: 892, sign: true });
    data.append(i32 { mag: 42, sign: false });
let extra = ExtraParams { fixed_point: Option::Some(FixedImpl::FP16x16(())) }; 
    TensorTrait::new(shape.span(), data.span(), Option::Some(extra))
}
