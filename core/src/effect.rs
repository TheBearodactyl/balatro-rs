use crate::game::Game;
use crate::hand::MadeHand;
use crate::joker::{Joker, Jokers};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct EffectRegistry {
    pub on_play: Vec<Effects>,
    pub on_discard: Vec<Effects>,
    pub on_score: Vec<Effects>,
    pub on_rank_hand: Vec<Effects>,
}

impl EffectRegistry {
    pub fn new() -> Self {
        Self {
            on_play: Vec::new(),
            on_discard: Vec::new(),
            on_score: Vec::new(),
            on_rank_hand: Vec::new(),
        }
    }

    pub(crate) fn register_jokers(&mut self, jokers: Vec<Jokers>, game: &Game) {
        for j in jokers.clone() {
            for e in j.effects(game) {
                match e {
                    Effects::OnPlay(_) => self.on_play.push(e),
                    Effects::OnDiscard(_) => self.on_discard.push(e),
                    Effects::OnScore(_) => self.on_score.push(e),
                    Effects::OnHandRank(_) => self.on_rank_hand.push(e),
                }
            }
        }
    }
}

impl Default for EffectRegistry {
    fn default() -> Self {
        Self::new()
    }
}

type GMHFn = Arc<Mutex<dyn Fn(&mut Game, MadeHand) + Send + 'static>>;
type GFn = Arc<Mutex<dyn Fn(&mut Game) + Send + 'static>>;

#[derive(Clone)]
// signature of these callbacks are more complicated so they
// can be used by pyo3 as part of python class.
pub enum Effects {
    OnPlay(GMHFn),
    OnDiscard(GMHFn),
    OnScore(GMHFn),
    OnHandRank(GFn),
}

impl std::fmt::Debug for Effects {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::OnPlay(_) => write!(f, "OnPlay"),
            Self::OnDiscard(_) => write!(f, "OnDiscard"),
            Self::OnScore(_) => write!(f, "OnScore"),
            Self::OnHandRank(_) => write!(f, "OnHandRank"),
        }
    }
}
