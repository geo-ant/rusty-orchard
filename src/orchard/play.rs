use crate::orchard::game::{GameState, Game};
use super::dice::DiceResult;
use super::strategies::PickingStrategy;
use super::game::Game::Intermediate;

pub fn play<S:PickingStrategy>(g: Game) -> Game {
    if let Intermediate(mut gs) = g {
        play_to_finish::<S>(&mut gs);
        Game::from(gs)
    }
    else {
        panic!("Cannot simulate from a Game that is already lost or won.");
    }
}

fn play_to_finish<S : PickingStrategy>(gs : &mut GameState) -> () {
    while !(gs.is_won() || gs.is_lost()) {
        play_single_turn::<S>(gs);
    }
    debug_assert!(gs.is_lost() != gs.is_won(), "Game must be either won or lost after playing to finish")
}

fn play_single_turn<S : PickingStrategy>(gs : &mut GameState) -> ()
{
    let dice = DiceResult::new_random();
    match dice {
        DiceResult::Basket => {
            gs.pick_with_strategy::<S>();
        },
        DiceResult::Raven => gs.add_raven(),
        DiceResult::FruitIndex(idx) => gs.try_pick_single_fruit(idx)
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_find_first_nonempty_index() -> ()
    {
        assert!(true);
    }
}