// mod nn;
// mod generated;

use core::array::SpanTrait;
use debug::PrintTrait;

// Cairo Layers
use api_new::nn::fc1;
use api_new::nn::fc2;

// Cairo generated Inputs from MNIST???
// use api_new::generated::input::input;

// Cairo Generated Layers from the Q Aware model
use api_new::generated::input::input;
use api_new::generated::fc1_bias::fc1_bias;
use api_new::generated::fc1_weights::fc1_weights;
use api_new::generated::fc2_bias::fc2_bias;
use api_new::generated::fc2_weights::fc2_weights;

use orion::operators::tensor::implementations::impl_tensor_fp::Tensor_fp;

#[test]
#[available_gas(99999999999999999)]
fn mnist_nn_test() {
    let input = input();
    let fc1_bias = fc1_bias();
    let fc1_weights = fc1_weights();
    let fc2_bias = fc2_bias();
    let fc2_weights = fc2_weights();

    

    let x = fc1(input, fc1_weights, fc1_bias);
    let x = fc2(x, fc2_weights, fc2_bias);
    let mut i = 0;
    loop {
       if !(*x.data.at(i).mag == 0) {
        i.print();
        break;
       } else {
        i += 1;
       };
    };
    // Stack Trace??
    // assert(*x.data.at(0).mag == 0, 'proba x is 0 -> 0');
    // assert(*x.data.at(1).mag == 0, 'proba x is 1 -> 0');
    // assert(*x.data.at(2).mag == 0, 'proba x is 2 -> 0');
    // assert(*x.data.at(3).mag > 62259, 'proba x is 3 -> 1');
    // assert(*x.data.at(4).mag == 0, 'proba x is 4 -> 0');
    // assert(*x.data.at(5).mag == 0, 'proba x is 5 -> 0');
    // assert(*x.data.at(6).mag == 0, 'proba x is 6 -> 0');
    // assert(*x.data.at(7).mag == 0, 'proba x is 7 -> 1'); // 62259 represents ONE in fp16x16.
    // assert(*x.data.at(8).mag == 0, 'proba x is 8 -> 0');
    // assert(*x.data.at(9).mag == 0, 'proba x is 9 -> 0');
}