use super::game::GameState;
use super::game::TREE_COUNT;
use rand::Rng;
use core::marker::Send;

pub trait PickingStrategy : Send {
    fn try_pick_two_fruits(gs : &mut GameState) -> ();
}

pub struct InOrderPickingStrategy;
pub struct RandomPickingStrategy;

unsafe impl Send for InOrderPickingStrategy {}
unsafe impl Send for RandomPickingStrategy {}

//todo Unify the code for the Picking Strategies based on the indexing functions
impl PickingStrategy for InOrderPickingStrategy {
    fn try_pick_two_fruits(gs: &mut GameState) -> () {
        for _ in 0..2 {
            match find_first_nonempty_index(&gs.fruit_count) {
                Some(idx) => gs.fruit_count[idx] -= 1,
                None => break,
            }
        }
    }
}

impl PickingStrategy for RandomPickingStrategy {
    fn try_pick_two_fruits(gs: &mut GameState) -> () {
        for _ in 0..2 {
            match find_random_nonempty_index(&gs.fruit_count) {
                Some(idx) => gs.fruit_count[idx] -= 1,
                None => break,
            }
        }
    }
}

fn find_first_nonempty_index(fruits : &[u8]) -> Option<usize>{
     for idx in 0..fruits.len()
     {
        if fruits[idx] > 0
        {
            return Some(idx)
        }
     }
    return None;
}

fn find_random_nonempty_index(fruits : &[u8]) -> Option<usize> {
    if fruits.iter().sum::<u8>() == 0 {
        return None;
    }

    let mut rand_idx :usize = rand::thread_rng().gen_range(0, TREE_COUNT);
    while fruits[rand_idx] == 0 {
        rand_idx = rand::thread_rng().gen_range(0, TREE_COUNT);
    }
    Some(rand_idx)
}
