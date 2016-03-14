#![allow(new_without_default)]

pub struct Game {
    rolls: Vec<i32>,
}

impl Game {

    pub fn new() -> Game {
        Game { rolls: Vec::with_capacity(21) }
    }

    pub fn roll(&mut self, pins: i32) {
        self.rolls.push(pins);
    }

    pub fn score(&self) -> i32 {
        let mut score = 0;
        let mut frame_index = 0;
        for _ in 0..10 {
            if self.is_strike(frame_index) {
                score += self.rolls[frame_index] + self.strike_bonus(frame_index);
                frame_index += 1;
            }
            else if self.is_sapre(frame_index) {
                score += 10 + self.spare_bonus(frame_index);
                frame_index += 2;
            }
            else {
                score += self.calculate_frame(frame_index);
                frame_index += 2;
            }
        }
        score
    }

    fn is_strike(&self, frame_index: usize) -> bool {
        self.rolls[frame_index] == 10
    }

    fn strike_bonus(&self, frame_index: usize) -> i32 {
        self.rolls[frame_index+1] + self.rolls[frame_index+2]
    }

    fn is_sapre(&self, frame_index: usize) -> bool {
        self.rolls[frame_index] + self.rolls[frame_index+1] == 10
    }

    fn spare_bonus(&self, frame_index: usize) -> i32 {
        self.rolls[frame_index+2]
    }

    fn calculate_frame(&self, frame_index: usize) -> i32 {
        self.rolls[frame_index] + self.rolls[frame_index+1]
    }
}
