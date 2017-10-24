pub struct Game {
    pins: Vec<u32>
}

impl Game {
    pub fn new() -> Game {
        Game {
            pins: Vec::with_capacity(20)
        }
    }

    pub fn roll(&mut self, pin: u32) {
        self.pins.push(pin);
    }

    pub fn score(&self) -> u32 {
        let mut score = 0;
        let mut frame = 0;
        for _ in 0..10 {
            if self.is_strike(frame) {
                score += 10 + self.strike_bonus(frame);
                frame += 1;
            } else if self.is_spare(frame) {
                score += 10 + self.spare_bonus(frame);
                frame += 2;
            } else {
                score += self.pins[frame] + self.pins[frame + 1];
                frame += 2;
            }
        }
        score
    }

    fn is_strike(&self, frame: usize) -> bool {
        self.pins[frame] == 10
    }

    fn strike_bonus(&self, frame: usize) -> u32 {
        self.pins[frame + 1] + self.pins[frame + 2]
    }

    fn is_spare(&self, frame: usize) -> bool {
        self.pins[frame] + self.pins[frame + 1] == 10
    }

    fn spare_bonus(&self, frame: usize) -> u32 {
        self.pins[frame + 2]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn roll_many(game: &mut Game, times: u32, pin: u32) {
        for _ in 0..times {
            game.roll(pin);
        }
    }

    fn roll_spare(game: &mut Game) {
        game.roll(5);
        game.roll(5);
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
        game.roll(4);
        game.roll(3);
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
