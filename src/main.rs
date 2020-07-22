mod orchard;

//use game::GameState;
use orchard::game::Game;
use crate::orchard::strategies::InOrderPickingStrategy;
use orchard::play::play;

fn main() {

    let g  = Game::new();
    let g_finished = play::<InOrderPickingStrategy>(g);

    println!("Finished game = {:?}", g_finished);
}

