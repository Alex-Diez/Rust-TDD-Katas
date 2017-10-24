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
        let mut frame_index = 0;
        for _ in 0..10 {
            if self.rolls[frame_index] == 10 {
                score += 10 + self.strike_bonus(&frame_index);
                frame_index += 1;
            } else if self.is_spare(&frame_index) {
                score += 10 + self.spare_bonus(&frame_index);
                frame_index += 2;
            } else {
                score += self.sum_of_balls_in_frame(&frame_index);
                frame_index += 2;
            }
        }
        score
    }

    fn is_spare(&self, frame_index: &usize) -> bool {
        self.rolls[*frame_index] + self.rolls[*frame_index + 1] == 10
    }

    fn strike_bonus(&self, frame_index: &usize) -> i32 {
        self.rolls[*frame_index + 1] + self.rolls[*frame_index + 2]
    }

    fn spare_bonus(&self, frame_index: &usize) -> i32 {
        self.rolls[*frame_index + 2]
    }

    fn sum_of_balls_in_frame(&self, frame_index: &usize) -> i32 {
        self.rolls[*frame_index] + self.rolls[*frame_index + 1]
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    fn roll_many(game: &mut Game, times: i32, pins: i32) {
        for _ in 0..times {
            game.roll(pins);
        }
    }

    fn spare(game: &mut Game) {
        game.roll(5);
        game.roll(5);
    }

    #[test]
    fn test_gutter_game() {
        let mut game = Game::new();
        roll_many(&mut game, 20, 0);

        assert_eq!(game.score(), 0);
    }

    #[test]
    fn test_all_ones() {
        let mut game = Game::new();
        roll_many(&mut game, 20, 1);

        assert_eq!(game.score(), 20);
    }

    #[test]
    fn test_one_spare() {
        let mut game = Game::new();
        spare(&mut game);
        game.roll(3);
        roll_many(&mut game, 17, 0);

        assert_eq!(game.score(), 16);
    }

    #[test]
    fn test_one_strike() {
        let mut game = Game::new();
        game.roll(10);
        game.roll(3);
        game.roll(4);
        roll_many(&mut game, 16, 0);

        assert_eq!(game.score(), 24);
    }
}
