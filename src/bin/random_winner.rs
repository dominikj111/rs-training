use rs_training::common::rng::SimpleRng;

/**
 * https://help.smarkets.com/hc/en-gb/articles/214058369-How-to-calculate-implied-probability-in-betting
 */
pub struct Game {
    rng: SimpleRng,
    my_money: f64,
    count_of_games: u32,
    count_of_wins: u32,
}

impl Game {
    pub fn new() -> Self {
        Game {
            rng: SimpleRng::new_from_time(),
            my_money: 1000.0,
            count_of_games: 0,
            count_of_wins: 0,
        }
    }

    fn generate_probabilities(&mut self) -> (f64, f64, f64) {
        let win_prob = self.rng.gen_float() * 100.0;
        let lose_prob = self.rng.gen_float() * (100.0 - win_prob);
        let draw_prob = 100.0 - win_prob - lose_prob;
        (win_prob, lose_prob, draw_prob)
    }

    fn play_game(&mut self) -> usize {
        let game_result = self.rng.gen_float() * 100.0;
        let (win_prob, _, _) = self.generate_probabilities();

        if game_result < win_prob {
            0
        } else if game_result < win_prob + (100.0 - win_prob) / 3.0 {
            1
        } else {
            2
        }
    }

    fn make_bet(&mut self) -> usize {
        self.rng.gen_range(3u64) as usize
    }

    fn play_round(&mut self) {
        let (win_prob, lose_prob, draw_prob) = self.generate_probabilities();
        let win_odd = 100.0 / win_prob;
        let lose_odd = 100.0 / lose_prob;
        let draw_odd = 100.0 / draw_prob;
        let odds = [win_odd, lose_odd, draw_odd];

        let game_result = self.play_game();
        let my_bet_index = self.make_bet();

        let bet_money = 10.0;

        self.count_of_games += 1;
        if game_result == my_bet_index {
            self.my_money += bet_money * odds[game_result];
            self.count_of_wins += 1;
        } else {
            self.my_money -= bet_money;
        }
    }

    pub fn run(&mut self) {
        while self.my_money > 0.0 && self.count_of_games < 10000 {
            self.play_round();
        }

        println!("Count of games: {}", self.count_of_games);
        println!("Count of wins: {}", self.count_of_wins);
        println!(
            "Count of loss: {}",
            self.count_of_games - self.count_of_wins
        );

        println!("My money: {}", self.my_money);
        println!("Profit: {}", self.my_money - 1000.0);
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

//tests

#[cfg(test)]
mod tests {
    use super::*;

    fn round_to_decimal(value: f64, precision: u32) -> f64 {
        let base: u32 = 10;
        let pow = base.pow(precision);
        (value * pow as f64).round() / pow as f64
    }

    #[test]
    fn test_round_to_decimal() {
        assert_eq!(round_to_decimal(100.00000000000002, 5), 100.0);
    }

    #[test]
    fn test_game_default() {
        let game = Game::default();
        assert_eq!(game.count_of_games, 0);
        assert_eq!(game.count_of_wins, 0);
        assert_eq!(game.my_money, 1000.0);
    }
}

fn main() {
    let mut game = Game::new();
    game.run();
}
