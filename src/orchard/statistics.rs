use std::default::Default;
use std::ops::Add;
//use rayon::prelude::*;
//use rayon::iter::ParallelBridge;

use super::game::Game;
use super::generator::GameGenerator;
use crate::orchard::strategies::PickingStrategy;

pub trait Statistic : Default + Add<Output=Self> + Send + Sync + Copy + Clone{
    fn new() -> Self {
        Self::default()
    }

    fn from_game(g: &Game) -> Result<Self,String>;
}

//
pub fn accumulate_statistics<STAT,STRAT> (initial : STAT, gamegen : GameGenerator<STRAT>) -> <STAT as std::ops::Add>::Output
where STRAT:PickingStrategy + Send , STAT:Statistic + Send{
    //sequential
    gamegen.into_iter().fold(initial , |cumulative_stat, game| cumulative_stat + STAT::from_game(&game).expect("Game was unfinised!"))
    //parallel todo: get this to work....
    //gamegen.into_iter().par_bridge().fold(||initial , |cumulative_stat, game| cumulative_stat + STAT::from_game(&game).expect("Game was unfinised!"));
}

#[derive(Debug,Copy,Clone)]
pub struct WinLossStatistic {
    pub wins : usize,
    pub losses : usize,
}

impl Statistic for WinLossStatistic {
    fn from_game(g:&Game) -> Result<Self,String> {
        match g {
            Game::Lost(_) => Ok(WinLossStatistic{wins :0 , losses :1}),
            Game::Won(_)  => Ok(WinLossStatistic{wins :1, losses : 0}),
            _ => Err("Cannot generate WinLossStatistic for unfinished game".to_string())
        }
    }
}

unsafe impl Send for WinLossStatistic {}
unsafe impl Sync for WinLossStatistic {}

impl Default for WinLossStatistic {
    fn default() -> Self {
        Self {wins :0 , losses : 0}
    }
}

impl Add for WinLossStatistic {
    type Output = WinLossStatistic;
    fn add(self, rhs: Self) -> Self {
        WinLossStatistic {wins : self.wins + rhs.wins, losses : self.losses+rhs.losses}
    }
}