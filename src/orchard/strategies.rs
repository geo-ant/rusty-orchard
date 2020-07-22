use super::game::GameState;

pub trait PickingStrategy {
    fn try_pick_two_fruits(gs : &mut GameState) -> ();
}

pub struct InOrderPickingStrategy;

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

fn find_first_nonempty_index(fruits : &[u32]) -> Option<usize>{
     for idx in 0..fruits.len()
     {
        if fruits[idx] > 0
        {
            return Some(idx)
        }
     }
    return None;
}

