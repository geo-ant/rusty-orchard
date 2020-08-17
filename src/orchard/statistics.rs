use std::default::Default;
use std::ops::Add;
use rayon::prelude::*;
use rayon::iter::ParallelBridge;

use super::game::Game;
use super::generator::GameGenerator;
use crate::orchard::strategies::PickingStrategy;

//use std::iter::Sum;

pub trait Statistic : Default + Add<Output=Self> + Copy + Clone + std::iter::Sum + Send {
    fn new() -> Self {
        Self::default()
    }

    fn from_game(g: &Game) -> Result<Self,String>;
}


///sequential version of accumulating statistics
pub fn accumulate_statistics_seq<STAT,STRAT> (initial : STAT, gamegen : GameGenerator<STRAT>) -> <STAT as std::ops::Add>::Output
    where STRAT:PickingStrategy + Send , STAT:Statistic + Send{
    initial+gamegen.into_iter().map(|g|STAT::from_game(&g).expect("Game was unfinished!")).sum()
}

///accumulate a statistics over all games in a game generator
pub fn accumulate_statistics_par<STAT,STRAT> (initial : STAT, gamegen : GameGenerator<STRAT>) -> <STAT as std::ops::Add>::Output
where STRAT:PickingStrategy + Send , STAT:Statistic + Send{
    //parallel
    //todo: this works now, but IS WAY SLOWER than the serial implementation...why? is it because par_bridge works suboptimal?
    //initial+gamegen.into_iter().par_bridge().map(|g|STAT::from_game(&g).expect("Game was unfinished!")).sum()
    //todo: other approach using rayon::split... this works much better. (Sequential: ~11283 for 10^7 elements, Parallel: ~1460 -> Speedup of x7.7. Not bad but I have 12 hw threads)

    rayon::iter::split(gamegen,|gg| {
        //todo: think about this splitter... would it be better to split off a chunk of given size?
        //why does it perform so well when the first chunk is half the input? does rayon already split more than the number of hw threads?
        let half = gg.len()/2;
        let rest = gg.len() - half;

        let halfgg : Option<GameGenerator<STRAT>>;
        if half == 0 {
            halfgg = None;
        } else {
            halfgg = Some(GameGenerator::new(half));
        }

        let restgg = GameGenerator::new(rest);

        (restgg, halfgg)
    }).into_par_iter().map(|gg|accumulate_statistics_seq(STAT::default(),gg)).sum()

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

//How do I write a blanket implementation for all types that implement statistic without
//the compiler complaining...?
impl std::iter::Sum for WinLossStatistic {
    fn sum<I: Iterator<Item=WinLossStatistic>>(iter: I) -> Self {
        iter.fold(WinLossStatistic::default(),|cumulative,next| cumulative+next)
    }
}

unsafe impl Send for WinLossStatistic{}