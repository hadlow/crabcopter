mod game;
mod copter;

fn main()
{
    let game: game::Game = game::Game::new();

    game.start();
}
