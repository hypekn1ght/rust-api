import os
import numpy as np
from tensorflow import keras
from keras.datasets import mnist
from scipy.ndimage import zoom
import tensorflow as tf

def main():
    # Load data
    (_, _), (x_test_image, y_test_label) = mnist.load_data()

    # Load the TFLite model and allocate tensors
    interpreter = tf.lite.Interpreter(model_path="output/q_aware_model.tflite")
    interpreter.allocate_tensors()

    # Prepare test set for inference (normalization)
    def resize_images(images):
        return np.array([zoom(image, 0.5) for image in images])

    x_test_image = resize_images(x_test_image)
    x_test_image_norm = (x_test_image / 255.0 * 255 - 128).astype(np.int8)

    # Create an object with all tensors (an input + all weights and biases)
    tensors = {
        "input": x_test_image[0].flatten(),
        "fc1_weights": interpreter.get_tensor(1), 
        "fc1_bias": interpreter.get_tensor(2), 
        "fc2_weights": interpreter.get_tensor(4), 
        "fc2_bias": interpreter.get_tensor(5)
    }

    # Generate Cairo Files for each object in tensor
    os.makedirs('src/generated', exist_ok=True)

    for tensor_name, tensor in tensors.items():
        with open(os.path.join('src', 'generated', f"{tensor_name}.cairo"), "w") as f:
            f.write(
                "use array::ArrayTrait;\n" +
                "use orion::operators::tensor::core::{TensorTrait, Tensor, ExtraParams};\n" +
                "use orion::operators::tensor::implementations::impl_tensor_i32::Tensor_i32;\n" +
                "use orion::numbers::fixed_point::core::FixedImpl;\n" +
                "use orion::numbers::signed_integer::i32::i32;\n\n" +
                "fn {0}() -> Tensor<i32> ".format(tensor_name) + "{\n" +
                "    let mut shape = ArrayTrait::<usize>::new();\n"
            )
            for dim in tensor.shape:
                f.write("    shape.append({0});\n".format(dim))
            f.write(
                "    let mut data = ArrayTrait::<i32>::new();\n"
            )
            for val in np.nditer(tensor.flatten()):
                f.write("    data.append(i32 {{ mag: {0}, sign: {1} }});\n".format(abs(int(val)), str(val < 0).lower()))
            f.write(
                "let extra = ExtraParams { fixed_point: Option::Some(FixedImpl::FP16x16(())) }; \n" +
                "    TensorTrait::new(shape.span(), data.span(), Option::Some(extra))\n" +
                "}\n"
            )

    with open(os.path.join('src', 'generated.cairo'), 'w') as f:
        for param_name in tensors.keys():
            f.write(f"mod {param_name};\n")

    print("Cairo files have been successfully generated.")

if __name__ == "__main__":
    print("Generating Neccessary Cairo Files")
    try:
        main()
    except Exception as e:
        print("Error generating the Cairo Files : ", e)

