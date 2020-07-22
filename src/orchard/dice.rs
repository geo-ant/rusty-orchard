use super::game::TREE_COUNT;

use rand::Rng;

pub enum DiceResult {
    Raven,
    Basket,
    FruitIndex(usize)
}

impl DiceResult
{
    //create new random dice result
    pub fn new_random() -> DiceResult {
        //see https://rust-lang-nursery.github.io/rust-cookbook/algorithms/randomness.html
        let number = rand::thread_rng().gen_range(0, TREE_COUNT+2) as usize;

        if number < TREE_COUNT{
            DiceResult::FruitIndex(number)
        }
        else if number == TREE_COUNT
        {
            DiceResult::Raven
        }
        else if number == TREE_COUNT+1
        {
            DiceResult::Basket
        }
        else
        {
            panic!("Number out of range for dice result!");
        }
    }
}

