
use super::strategies::PickingStrategy;

/// number of trees in game
pub const TREE_COUNT : usize = 4;
/// maximum number of ravens (i.e. the number of ravens at which the game is lost)
pub const MAX_RAVEN_COUNT : u32 = 9;
/// number of initial fruit on each tree
pub const INITIAL_FRUIT_COUNT : u32 = 10;

#[derive(Debug)]
pub enum Game
{
    Won (WinningState),
    Lost (LosingState),
    Intermediate (GameState)
}

impl Game
{
    pub fn new() -> Self    {
        Self::from(GameState::new())
    }

    pub fn from(gs: GameState) -> Self {
        assert!(!(gs.is_won() && gs.is_lost()), "Game cannot be won and lost at the same time");
        if gs.is_won()
        {
            Self::Won(WinningState{turn_count:gs.turn_count, raven_count:gs.raven_count})
        }
        else if gs.is_lost() {
            Self::Lost(LosingState{turn_count:gs.turn_count, total_fruit_count:gs.fruit_count.iter().sum()})
        }
        else
        {
            Self::Intermediate(gs)
        }
    }
}

#[derive(Debug)]
pub struct WinningState
{
    pub turn_count : u32,
    pub raven_count : u32
}

#[derive(Debug)]
pub struct LosingState
{
    pub turn_count : u32,
    pub total_fruit_count : u32
}

#[derive(Debug)]
pub struct GameState {
    pub fruit_count: [u32; TREE_COUNT as usize],
    pub turn_count: u32,
    pub raven_count: u32
}

impl GameState {
    fn new() -> Self
    {
        GameState {fruit_count : [INITIAL_FRUIT_COUNT;TREE_COUNT], raven_count : 0, turn_count : 0}
    }

    pub fn add_raven(&mut self) -> ()
    {
        assert!(self.raven_count<MAX_RAVEN_COUNT, "Raven count already maxed out!");
        self.raven_count = self.raven_count+1;
        self.turn_count = self.turn_count+1;
    }

    ///picks single fruit if tree is not empty
    pub fn try_pick_single_fruit(&mut self, index : usize) -> () {
        assert!(index < TREE_COUNT, "Index is out of bounds!");
        if self.fruit_count[index]>0
        {
            self.fruit_count[index] = self.fruit_count[index] - 1;
        }
        self.add_turn();
    }

    pub fn pick_with_strategy<S:PickingStrategy> (&mut self) {
        let prev_sum : u32= self.fruit_count.iter().sum();
        S::try_pick_two_fruits(self);
        self.add_turn();
        let sum_after : u32 = self.fruit_count.iter().sum();
        assert!(prev_sum-sum_after<=2, "More than two fruit have been picked by a strategy");
    }

    pub fn is_won(&self) -> bool {
        self.fruit_count.iter().sum::<u32>() == 0
    }

    pub fn is_lost(&self) -> bool {
        self.raven_count >= super::game::MAX_RAVEN_COUNT
    }

    fn add_turn(&mut self){
        self.turn_count+=1;
    }
}