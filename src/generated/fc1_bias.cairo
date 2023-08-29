use array::ArrayTrait;
use orion::operators::tensor::core::{TensorTrait, Tensor, ExtraParams};
use orion::operators::tensor::implementations::impl_tensor_i32::Tensor_i32;
use orion::numbers::fixed_point::core::FixedImpl;
use orion::numbers::signed_integer::i32::i32;

fn fc1_bias() -> Tensor<i32> {
    let mut shape = ArrayTrait::<usize>::new();
    shape.append(10);
    let mut data = ArrayTrait::<i32>::new();
    data.append(i32 { mag: 7531, sign: true });
    data.append(i32 { mag: 6611, sign: false });
    data.append(i32 { mag: 4305, sign: true });
    data.append(i32 { mag: 3972, sign: false });
    data.append(i32 { mag: 5236, sign: true });
    data.append(i32 { mag: 13535, sign: false });
    data.append(i32 { mag: 2338, sign: false });
    data.append(i32 { mag: 15852, sign: false });
    data.append(i32 { mag: 1238, sign: false });
    data.append(i32 { mag: 3692, sign: false });
let extra = ExtraParams { fixed_point: Option::Some(FixedImpl::FP16x16(())) }; 
    TensorTrait::new(shape.span(), data.span(), Option::Some(extra))
}
