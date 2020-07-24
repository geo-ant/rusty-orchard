mod orchard;

//use game::GameState;
use crate::orchard::strategies::{InOrderPickingStrategy,RandomPickingStrategy};
use crate::orchard::generator::GameGenerator;
use crate::orchard::statistics::accumulate_statistics;
use crate::orchard::statistics::WinLossStatistic;

fn main() {

   let num_games : usize = 10000;

   let in_order_stat = accumulate_statistics(WinLossStatistic::default(), GameGenerator::<InOrderPickingStrategy>::new(num_games));
   println!("In Order Strategy = {:?}",in_order_stat);

   let random_stat = accumulate_statistics(WinLossStatistic::default(), GameGenerator::<RandomPickingStrategy>::new(num_games));
   println!("Random Strategy = {:?}",random_stat);

}

