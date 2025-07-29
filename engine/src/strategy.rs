use std::{
    collections::{HashMap, VecDeque},
    hash::Hash,
    iter,
    ops::RangeInclusive,
};

use crate::{
    card::{Card, Rank},
    game::GameState,
    hand::{Hand, Outcome},
    rules::{Rules, Soft17Rule},
    shoe::Shoe,
};

#[derive(Clone, Copy)]
pub struct RoundEvs {
    pub hit: f64,
    pub stand: f64,
    pub double: f64,
    pub split: Option<f64>,
}

impl RoundEvs {
    pub fn best(&self) -> PlayerAction {
        let mut best_action = PlayerAction::Stand;
        let mut best_value = self.stand;

        if self.hit > best_value {
            best_value = self.hit;
            best_action = PlayerAction::Hit;
        }

        if self.double > best_value {
            best_value = self.double;
            if best_action == PlayerAction::Stand {
                best_action = PlayerAction::DoubleOrStand;
            } else {
                best_action = PlayerAction::DoubleOrHit;
            }
        }

        if let Some(split_val) = self.split {
            if split_val > best_value {
                best_action = PlayerAction::Split;
            }
        }

        best_action
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayerAction {
    Hit,
    Stand,
    DoubleOrHit,
    DoubleOrStand,
    Split,
}

pub struct StrategyGenerator<S: Shoe> {
    rules: Rules,
    shoe: S,
    hit_cache: HashMap<GameState<S>, f64>,
    stand_cache: HashMap<GameState<S>, f64>,
    double_cache: HashMap<GameState<S>, f64>,
    split_cache: HashMap<GameState<S>, f64>,
}

impl<S: Shoe + Clone + Eq + Hash> StrategyGenerator<S> {
    pub fn new(rules: Rules, shoe: S) -> Self {
        Self {
            rules,
            shoe,
            hit_cache: HashMap::new(),
            stand_cache: HashMap::new(),
            double_cache: HashMap::new(),
            split_cache: HashMap::new(),
        }
    }

    pub fn iter_dealer_hands(&self, state: &GameState<S>) -> impl Iterator<Item = (Hand, f64)> {
        let rules = self.rules.clone();
        let mut starting_hand = Hand::new();
        let mut queue = VecDeque::with_capacity(100_000);

        starting_hand.add_card(state.dealer_upcard);
        queue.push_back((starting_hand, self.shoe.clone(), 1.0));

        iter::from_fn(move || {
            while let Some((hand, shoe, probability)) = queue.pop_front() {
                let value = hand.value();

                let must_stand = if value > 17 {
                    true
                } else if value == 17 {
                    if hand.is_soft() {
                        match rules.dealer_soft_17 {
                            Soft17Rule::Stand => true,
                            Soft17Rule::Hit => false,
                        }
                    } else {
                        true
                    }
                } else {
                    false
                };

                if must_stand {
                    return Some((hand, probability));
                }

                for (card, weight) in shoe.iter_draws() {
                    let mut new_hand = hand.clone();
                    new_hand.add_card(card);

                    let mut new_shoe = shoe.clone();
                    new_shoe.remove_card(&card);

                    queue.push_back((new_hand, new_shoe, probability * weight));
                }
            }

            None
        })
    }

    pub fn expected_value_stand(&mut self, state: &GameState<S>) -> f64 {
        if let Some(item) = self.stand_cache.get(state) {
            return *item;
        }
        let mut total_ev = 0.0;
        let mut total_weight = 0.0;

        for (dealer_hand, weight) in self.iter_dealer_hands(state) {
            let ev = match Hand::compare(&state.player_hand, &dealer_hand) {
                Outcome::Win => {
                    if state.player_hand.is_blackjack() {
                        1.5
                    } else {
                        1.0
                    }
                }
                Outcome::Push => 0.0,
                Outcome::Lose => -1.0,
            };
            total_weight += weight;
            total_ev += ev * weight;
        }

        let ev = total_ev / total_weight;
        self.stand_cache.insert(state.clone(), ev);
        ev
    }

    pub fn expected_value_hit(&mut self, state: &GameState<S>) -> f64 {
        if let Some(item) = self.hit_cache.get(&state) {
            return *item;
        }
        let mut total_ev = 0.0;
        let mut total_weight = 0.0;

        for (card, weight) in state.shoe.iter_draws() {
            let mut state_copy = state.clone();
            state_copy.player_hand.add_card(card);
            state_copy.shoe.remove_card(&card);

            let ev = if state_copy.player_hand.is_bust() {
                -1.0
            } else if state_copy.player_hand.value() == 21 {
                self.expected_value_stand(&state_copy)
            } else {
                let ev_hit = self.expected_value_hit(&state_copy);
                let ev_stand = self.expected_value_stand(&state_copy);
                f64::max(ev_hit, ev_stand)
            };

            total_ev += ev * weight;
            total_weight += weight;
        }

        let ev = total_ev / total_weight;
        self.hit_cache.insert(state.clone(), ev);
        ev
    }

    pub fn expected_value_double(&mut self, state: &GameState<S>) -> f64 {
        if let Some(item) = self.double_cache.get(state) {
            return *item;
        }

        let mut total_ev = 0.0;
        let mut total_weight = 0.0;

        for (card, weight) in state.shoe.iter_draws() {
            let mut state_copy = state.clone();
            state_copy.player_hand.add_card(card);
            state_copy.shoe.remove_card(&card);

            let ev = self.expected_value_stand(&state_copy) * 2.0;
            total_ev += weight * ev;
            total_weight += weight;
        }

        let ev = total_ev / total_weight;
        self.double_cache.insert(state.clone(), ev);
        ev
    }

    pub fn expected_value_split(&mut self, state: &GameState<S>) -> f64 {
        if let Some(item) = self.split_cache.get(state) {
            return *item;
        }

        let mut total_ev = 0.0;
        let mut total_weight = 0.0;

        for (card, weight) in state.shoe.iter_draws() {
            let mut state_copy = state.clone();
            state_copy.player_hand.split();
            state_copy.player_hand.add_card(card);
            state_copy.shoe.remove_card(&card);

            let mut ev = f64::max(
                self.expected_value_hit(&state_copy),
                self.expected_value_stand(&state_copy),
            );
            if self.rules.double_after_split_allowed {
                ev = ev.max(self.expected_value_double(&state_copy))
            }
            total_ev += weight * ev * 2.0;
            total_weight += weight;
        }

        let ev = total_ev / total_weight;
        self.split_cache.insert(state.clone(), ev);
        ev
    }

    pub fn eval_round(&mut self, player_hand: Hand, dealer_upcard: Card) -> RoundEvs {
        let mut state = GameState {
            dealer_upcard,
            player_hand,
            shoe: self.shoe.clone(),
        };

        state.shoe.remove_card(&dealer_upcard);

        RoundEvs {
            hit: self.expected_value_hit(&state),
            stand: self.expected_value_stand(&state),
            double: self.expected_value_double(&state),
            split: if player_hand.is_pair() {
                Some(self.expected_value_split(&state))
            } else {
                None
            },
        }
    }

    pub fn hard_table(&mut self) -> StrategyTable {
        let mut table = StrategyTable::new(5, 21);

        for player_value in table.player_value_range() {
            for dealer_value in table.dealer_value_range() {
                let dealer_upcard = Card {
                    rank: Rank::from_value(dealer_value),
                };

                let player_hand = Hand::hard_from_value(player_value);

                let evs = self.eval_round(player_hand, dealer_upcard);
                let value = StrategyValue::from_evs(evs);

                table.set(player_value, dealer_value, value);
            }
        }

        table
    }

    pub fn soft_table(&mut self) -> StrategyTable {
        let mut table = StrategyTable::new(13, 21);

        for player_value in table.player_value_range() {
            for dealer_value in table.dealer_value_range() {
                let dealer_upcard = Card {
                    rank: Rank::from_value(dealer_value),
                };

                let player_hand = Hand::soft_from_value(player_value);

                let evs = self.eval_round(player_hand, dealer_upcard);
                let value = StrategyValue::from_evs(evs);

                table.set(player_value, dealer_value, value);
            }
        }

        table
    }

    pub fn pair_table(&mut self) -> StrategyTable {
        let mut table = StrategyTable::new(2, 11);

        for player_value in table.player_value_range() {
            for dealer_value in table.dealer_value_range() {
                let dealer_upcard = Card {
                    rank: Rank::from_value(dealer_value),
                };

                let player_hand = Hand::pair_from_single_value(player_value);

                let evs = self.eval_round(player_hand, dealer_upcard);
                let value = StrategyValue::from_evs(evs);

                table.set(player_value, dealer_value, value);
            }
        }

        table
    }
}

#[derive(Clone, Copy)]
pub struct StrategyValue {
    pub ev: f64,
    pub evs: RoundEvs,
    pub action: PlayerAction,
}

impl StrategyValue {
    fn from_evs(evs: RoundEvs) -> Self {
        if let Some(split) = evs.split {
            if split > evs.hit && split > evs.stand && split > evs.double {
                return Self {
                    action: PlayerAction::Split,
                    ev: split,
                    evs,
                };
            }
        }
        if evs.double >= evs.hit && evs.double >= evs.stand {
            let fallback_action = if evs.hit >= evs.stand {
                PlayerAction::DoubleOrHit
            } else {
                PlayerAction::DoubleOrStand
            };
            Self {
                action: fallback_action,
                ev: evs.double,
                evs,
            }
        } else if evs.hit >= evs.stand {
            Self {
                action: PlayerAction::Hit,
                ev: evs.hit,
                evs,
            }
        } else {
            Self {
                action: PlayerAction::Stand,
                ev: evs.stand,
                evs,
            }
        }
    }
}

pub struct StrategyTable {
    values: Vec<StrategyValue>,
    from: u8,
    to: u8,
}

impl StrategyTable {
    pub fn new(from: u8, to: u8) -> Self {
        let size = (to - from + 1) as usize * 10;
        Self {
            from,
            to,
            values: vec![
                StrategyValue {
                    action: PlayerAction::Hit,
                    ev: 0.0,
                    evs: RoundEvs {
                        hit: 0.0,
                        stand: 0.0,
                        double: 0.0,
                        split: None,
                    }
                };
                size
            ],
        }
    }

    fn index(&self, player_value: u8, dealer_value: u8) -> usize {
        let x = (dealer_value - 2) as usize;
        let y = (player_value - self.from) as usize;
        return x + y * 10;
    }

    pub fn set(&mut self, player_value: u8, dealer_value: u8, value: StrategyValue) {
        let index = self.index(player_value, dealer_value);
        self.values[index] = value;
    }

    pub fn get(&self, player_value: u8, dealer_value: u8) -> &StrategyValue {
        let index = self.index(player_value, dealer_value);
        &self.values[index]
    }

    pub fn player_from(&self) -> u8 {
        return self.from;
    }

    pub fn player_to(&self) -> u8 {
        return self.to;
    }

    pub fn player_value_range(&self) -> RangeInclusive<u8> {
        self.from..=self.to
    }

    pub fn dealer_value_range(&self) -> RangeInclusive<u8> {
        2..=11
    }
}
