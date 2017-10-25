pub struct Game {
    rolls: Vec<u32>
}

impl Game {
    pub fn new() -> Self {
        Game { rolls: Vec::with_capacity(20) }
    }

    pub fn roll(&mut self, pin: u32) {
        self.rolls.push(pin);
    }

    pub fn score(&self) -> u32 {
        let mut score = 0;
        let mut frame_index = 0;
        for _ in 0..10 {
            if self.rolls[frame_index] == 10 {
                score += 10 + self.rolls[frame_index + 1] + self.rolls[frame_index + 2];
                frame_index += 1;
            } else if self.rolls[frame_index] + self.rolls[frame_index + 1] == 10 {
                score += 10 + self.rolls[frame_index + 2];
                frame_index += 2;
            } else {
                score += self.rolls[frame_index] + self.rolls[frame_index + 1];
                frame_index += 2;
            }
        }

        score
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn roll_many(game: &mut Game, times: usize, pin: u32) {
        for _ in 0..times {
            game.roll(pin);
        }
    }

    fn roll_spare(game: &mut Game) {
        game.roll(6);
        game.roll(4);
    }

    fn roll_strike(game: &mut Game) {
        game.roll(10);
    }

    #[test]
    fn gutter_game() {
        let mut game = Game::new();

        roll_many(&mut game, 20, 0);

        assert_eq!(game.score(), 0);
    }

    #[test]
    fn all_ones() {
        let mut game = Game::new();

        roll_many(&mut game, 20, 1);

        assert_eq!(game.score(), 20);
    }

    #[test]
    fn one_spare() {
        let mut game = Game::new();

        roll_spare(&mut game);
        game.roll(3);
        roll_many(&mut game, 17, 0);

        assert_eq!(game.score(), 16);
    }

    #[test]
    fn one_strike() {
        let mut game = Game::new();

        roll_strike(&mut game);
        game.roll(3);
        game.roll(4);
        roll_many(&mut game, 16, 0);

        assert_eq!(game.score(), 24);
    }

    #[test]
    fn perfect_game() {
        let mut game = Game::new();

        roll_many(&mut game, 13, 10);

        assert_eq!(game.score(), 300);
    }
}
