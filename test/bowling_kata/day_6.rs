use tdd_kata::bowling_kata::day_6::Game;

fn roll_many(game: &mut Game, times: i32, pins: i32) {
    for i in 0..times {
        game.roll(pins);
    }
}

fn roll_strike(game: &mut Game) {
    game.roll(10);
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
    roll_strike(&mut game);
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
