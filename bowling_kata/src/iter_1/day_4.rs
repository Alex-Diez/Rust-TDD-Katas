#[derive(Default)]
#[allow(dead_code)]
pub struct Game {
    score: i32,
    rolls: Vec<i32>,
}

impl Game {
    pub fn new() -> Game {
        Game { score: 0, rolls: Vec::with_capacity(21) }
    }

    pub fn roll(&mut self, pins: i32) {
        self.rolls.push(pins);
    }

    pub fn score(&self) -> i32 {
        let mut score = 0;
        let mut frame_index = 0;
        for _ in 0..10 {
            if self.is_strike(frame_index) {
                score += 10 + self.strike_bonus(frame_index);
                frame_index += 1;
            } else if self.is_spare(frame_index) {
                score += 10 + self.spare_bonus(frame_index);
                frame_index += 2;
            } else {
                score += self.frame_points(frame_index);
                frame_index += 2;
            }
        }
        score
    }

    fn is_spare(&self, frame_index: usize) -> bool {
        self.rolls[frame_index] + self.rolls[frame_index + 1] == 10
    }

    fn spare_bonus(&self, frame_index: usize) -> i32 {
        self.rolls[frame_index + 2]
    }

    fn frame_points(&self, frame_index: usize) -> i32 {
        self.rolls[frame_index] + self.rolls[frame_index + 1]
    }

    fn is_strike(&self, frame_index: usize) -> bool {
        self.rolls[frame_index] == 10
    }

    fn strike_bonus(&self, frame_index: usize) -> i32 {
        self.rolls[frame_index + 1] + self.rolls[frame_index + 2]
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

    fn roll_spare(game: &mut Game) {
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
        roll_spare(&mut game);
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

    #[test]
    fn test_perfect_game() {
        let mut game = Game::new();
        roll_many(&mut game, 12, 10);
        assert_eq!(game.score(), 300);
    }
}
