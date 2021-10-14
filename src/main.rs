mod game;
mod copter;

fn main()
{
    let mut game: game::Game = game::Game::new();

    game.start();
}
