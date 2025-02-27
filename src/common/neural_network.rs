pub struct Perceptron {
    pub weights: Vec<f64>,
    pub bias: f64,
}

impl Perceptron {
    pub fn new(num_inputs: usize) -> Self {
        Perceptron {
            weights: vec![0.0; num_inputs],
            bias: 0.0,
        }
    }

    fn step_activation(x: f64) -> f64 {
        if x > 0.0 {
            1.0
        } else {
            0.0
        }
    }

    pub fn predict(&self, inputs: &[f64]) -> f64 {
        let sum: f64 = self.weights.iter().zip(inputs).map(|(w, i)| w * i).sum();
        Self::step_activation(sum + self.bias)
    }

    pub fn train(&mut self, inputs: &[f64], target: f64, learning_rate: f64) {
        let prediction = self.predict(inputs);
        let error = target - prediction;
        for (i, _item) in inputs.iter().enumerate().take(self.weights.len()) {
            self.weights[i] += learning_rate * error * inputs[i];
        }
        self.bias += learning_rate * error;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_perceptron_prediction() {
        let perceptron = Perceptron::new(2);
        let input = vec![1.0, 1.0];
        let prediction = perceptron.predict(&input);
        assert!(
            prediction == 0.0,
            "Initial prediction should be 0.0 with zero weights"
        );
    }

    #[test]
    fn test_perceptron_training() {
        let mut perceptron = Perceptron::new(2);
        let input = vec![1.0, 1.0];
        let target = 1.0;

        // Train multiple times to ensure learning
        for _ in 0..10 {
            perceptron.train(&input, target, 0.1);
        }

        let prediction = perceptron.predict(&input);
        assert_eq!(
            prediction, target,
            "Perceptron should learn to output the target value"
        );
    }
}
