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
            if self.rolls[i] + self.rolls[i+1] == 10 {
                score += 10 + self.rolls[i+2];
            }
            else {
                score += self.rolls[i] + self.rolls[i+1];
            }
            i += 2;
        }
        score
    }
}
