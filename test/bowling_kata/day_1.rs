use tdd_kata::bowling_kata::day_1::Game;

fn roll_many(game: &mut Game, pins: i32) {
    for i in 0..20 {
        game.roll(pins);
    }
}

#[test]
fn test_gutter_game() {
    let mut game = Game::new();
    roll_many(&mut game, 0);
    assert_eq!(game.score(), 0);
}

#[test]
fn test_all_once() {
    let mut game = Game::new();
    roll_many(&mut game, 1);
    assert_eq!(game.score(), 20);
}

#[test]
// #[ignore]
fn test_one_spare() {
    let mut game = Game::new();
    game.roll(5);
    game.roll(5);
    game.roll(3);
    for i in 0..17 {
        game.roll(0);
    }
    assert_eq!(game.score(), 16)
}
