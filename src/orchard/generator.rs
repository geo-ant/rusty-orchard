use std::iter::Iterator;
use std::iter::IntoIterator;
use crate::orchard::game::Game;
use crate::orchard::play::play;
use super::strategies::PickingStrategy;
use std::marker::PhantomData;
use core::marker::Send;

pub struct GameIterator<S:PickingStrategy> {
    num_games : usize,
    current_pos : usize,
    phantom : PhantomData<S>,
}

unsafe impl<S:PickingStrategy> Send for GameIterator<S> {}

impl<S:PickingStrategy> ExactSizeIterator for GameIterator<S> {
    fn len (&self) -> usize {
        self.num_games
    }
}

impl<S:PickingStrategy> Iterator for GameIterator<S> {
    type Item = Game;

    fn next(& mut self) -> Option<Game> {
        if self.current_pos < self.num_games {
            self.current_pos += 1;
            Some(play::<S>(Game::new()))
        }
        else {
            None
        }
    }
}

pub struct GameGenerator<S:PickingStrategy>
{
    num_games : usize,
    phantom : PhantomData<S>,
}

impl<S:PickingStrategy> GameGenerator<S> {
    pub fn new(num_games : usize) -> Self {
        GameGenerator::<S> {num_games, phantom : PhantomData}
    }
}

impl<S:PickingStrategy> IntoIterator for GameGenerator<S> {
    type Item = Game;
    type IntoIter = GameIterator<S>;
    fn into_iter(self) -> GameIterator<S> {
        GameIterator::<S> {num_games : self.num_games, current_pos : 0, phantom : PhantomData}
    }
}