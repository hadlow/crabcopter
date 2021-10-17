mod game;
mod canvas;
mod object;
mod wall;
mod copter;

fn main()
{
    let mut game: game::Game = game::Game::new();

    game.start();
}
