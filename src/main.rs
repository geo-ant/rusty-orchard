mod orchard;

//use game::GameState;
use crate::orchard::strategies::{InOrderPickingStrategy,RandomPickingStrategy};
use crate::orchard::generator::GameGenerator;
use crate::orchard::statistics::{accumulate_statistics_par,accumulate_statistics_seq};
use crate::orchard::statistics::WinLossStatistic;

fn main() {

   let num_games : usize = 10000000;


   let tic = std::time::SystemTime::now();
   let in_order_stat = accumulate_statistics_seq(WinLossStatistic::default(), GameGenerator::<InOrderPickingStrategy>::new(num_games));
   let toc = std::time::SystemTime::now();

   let duration = toc.duration_since(tic).expect("Error querying system time");

   println!("Duration for in order calculation= {} ms", duration.as_millis());

   println!("In Order Strategy = {:?}",in_order_stat);

   //let random_stat = accumulate_statistics_par(WinLossStatistic::default(), GameGenerator::<RandomPickingStrategy>::new(num_games));
   //println!("Random Strategy = {:?}",random_stat);

}

