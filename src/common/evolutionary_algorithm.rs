use super::rng::SimpleRng;

pub struct GeneticAlgorithm {
    pub population_size: usize,
    pub mutation_rate: f64,
    pub crossover_rate: f64,
    rng: SimpleRng,
}

impl GeneticAlgorithm {
    pub fn new(population_size: usize, mutation_rate: f64, crossover_rate: f64) -> Self {
        GeneticAlgorithm {
            population_size,
            mutation_rate,
            crossover_rate,
            rng: SimpleRng::new_from_time(),
        }
    }

    pub fn evolve(&self) {
        // Placeholder for evolution logic
    }

    pub fn select(&self, population: &[f64]) -> Vec<f64> {
        // Simple selection logic (e.g., roulette wheel)
        population.to_vec().clone()
    }

    pub fn crossover(&self, parent1: &[f64], parent2: &[f64]) -> Vec<f64> {
        // Simple one-point crossover
        let midpoint = parent1.len() / 2;
        let mut child = parent1[..midpoint].to_vec();
        child.extend_from_slice(&parent2[midpoint..]);
        child
    }

    pub fn mutate(&mut self, individual: &mut [f64]) {
        // Simple mutation logic
        for gene in individual.iter_mut() {
            if self.rng.gen_float() < self.mutation_rate {
                *gene += self.rng.gen_float() - 0.5;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_genetic_algorithm_selection() {
        let ga = GeneticAlgorithm::new(10, 0.1, 0.5);
        let population = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
        // let fitness_scores = vec![0.5, 0.8];
        let selected = ga.select(&population.concat());
        assert_eq!(selected.len(), 4);
    }

    #[test]
    fn test_genetic_algorithm_crossover() {
        let ga = GeneticAlgorithm::new(10, 0.1, 0.5);
        let parent1 = vec![1.0, 2.0];
        let parent2 = vec![3.0, 4.0];
        let child = ga.crossover(&parent1, &parent2);
        assert_eq!(child.len(), parent1.len());
    }

    #[test]
    fn test_genetic_algorithm_mutation() {
        let mut ga = GeneticAlgorithm::new(10, 1.0, 0.5);
        let mut individual = vec![1.0, 2.0];
        ga.mutate(&mut individual);
        assert_eq!(individual.len(), 2);
    }
}
