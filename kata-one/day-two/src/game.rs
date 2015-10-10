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
        for frame in 0..10 {
            if self.rolls[frame_index] == 10 {
                score += 10 + self.strike_bonus(&frame_index);
                frame_index += 1;
            }
            else if self.is_spare(&frame_index) {
                score += 10 + self.spare_bonus(&frame_index);
                frame_index += 2;
            }
            else {
                score += self.sum_of_balls_in_frame(&frame_index);
                frame_index += 2;
            }
        }
        score
    }

    fn is_spare(&self, frame_index: &usize) -> bool {
        self.rolls[*frame_index] + self.rolls[*frame_index+1] == 10
    }

    fn strike_bonus(&self, frame_index: &usize) -> i32 {
        self.rolls[*frame_index+1] + self.rolls[*frame_index+2]
    }

    fn spare_bonus(&self, frame_index: &usize) -> i32 {
        self.rolls[*frame_index+2]
    }

    fn sum_of_balls_in_frame(&self, frame_index: &usize) -> i32 {
        self.rolls[*frame_index] + self.rolls[*frame_index+1]
    }
}
