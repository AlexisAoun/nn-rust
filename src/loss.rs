use crate::{activation::softmax, Layer, Matrix};

pub fn one_hot_encoding(input: &Matrix, labels: &Matrix) -> Matrix {
    assert_eq!(
        input.height, labels.width,
        "Input height and labels width should be equal"
    );
    let mut output = Matrix::new(1, input.height);
    for c in 0..input.height {
        output.data[0][c] = input.data[c][labels.data[0][c] as usize];
    }

    output
}

pub fn cross_entropy(output: &Matrix, labels: &Matrix) -> f64 {
    let output_one_hot: Matrix = one_hot_encoding(&output, &labels);
    let output_softmax: Matrix = softmax(&output_one_hot);

    let mut loss: f64 = 0.0;
    for c in 0..output_softmax.width {
        loss += -output_softmax.data[0][c].ln();
    }

    loss / output_softmax.width as f64
}

pub fn l2_reg(layers: &Vec<Layer>, lambda: f64) -> f64 {
    let mut l2: f64 = 0.0;
    for layer in layers {
        l2 += 0.5 * lambda * (layer.weights_t.pow(2).sum());
    }

    l2
}
