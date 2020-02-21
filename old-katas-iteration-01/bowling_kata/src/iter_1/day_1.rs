#[derive(Default)]
pub struct Game {
    rolls: Vec<i32>,
    current_roll: usize,
}

impl Game {
    pub fn new() -> Game {
        Game { rolls: Vec::with_capacity(21), current_roll: 0 }
    }

    pub fn roll(&mut self, pins: i32) {
        self.rolls.push(pins);
        self.current_roll += 1;
    }

    pub fn score(&self) -> i32 {
        let mut score = 0;
        let mut i = 0;
        for _ in 0..10 {
            if self.rolls[i] + self.rolls[i + 1] == 10 {
                score += 10 + self.rolls[i + 2];
            } else {
                score += self.rolls[i] + self.rolls[i + 1];
            }
            i += 2;
        }
        score
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    fn roll_many(game: &mut Game, times: usize, pins: i32) {
        for _ in 0..times {
            game.roll(pins);
        }
    }

    #[test]
    fn test_gutter_game() {
        let mut game = Game::new();
        roll_many(&mut game, 20, 0);
        assert_eq!(game.score(), 0);
    }

    #[test]
    fn test_all_once() {
        let mut game = Game::new();
        roll_many(&mut game, 20, 1);
        assert_eq!(game.score(), 20);
    }

    #[test]
    fn test_one_spare() {
        let mut game = Game::new();
        game.roll(5);
        game.roll(5);
        game.roll(3);
        roll_many(&mut game, 17, 0);

        assert_eq!(game.score(), 16)
    }
}
