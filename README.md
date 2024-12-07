# balatro-rs

Game engine and move generator for a simplified version of [balatro](https://www.playbalatro.com/)
## Overview

This library implements a subset of balatro's rules allowing execution of games or simulations. It provides an exhaustive list of actions a user could take at any given stage, as well as an engine to execute those actions and progress the game.

The goal of providing all valid actions is to serve as a move generator. This would be the first step to apply reinforcement learning for balatro.

## Example

```rust
use balatro_rs::core::{action::Action, game::Game, stage::Stage};
use rand::Rng;

fn main() {
    let mut g = Game::new();
    g.start();
    while !g.is_over() {
        // Get all available moves
        let actions: Vec<Action> = g.gen_moves().collect();
        if actions.len() == 0 {
            break;
        }

        // Pick a random move and execute it
        let i = rand::thread_rng().gen_range(0..actions.len());
        let action = actions[i].clone();
        g.handle_action(action);
    }
    let result = g.result();
}
```

## Features

This library does not implement all aspects of balatro and likely never will.

The follow features are implemented (including move generation)
- [x] identification and scoring of poker hands
- [x] playing/discarding/reordering of cards
- [x] blind pass/fail and game win/lose conditions
- [x] money/interest generation
- [x] ante progression (up to ante 8)
- [x] blind progression (small, big, boss)
- [x] stage transition (pre-blind, blind, post-blind, shop)

The following features are missing and may or may not be added
- [ ] buying/selling/using jokers
- [ ] buying/selling/using tarots
- [ ] buying/selling/using planets
- [ ] buying/selling/using spectrals
- [ ] boss blind modifiers
- [ ] skip blind/tags
- [ ] alternative decks
- [ ] alternative stakes

