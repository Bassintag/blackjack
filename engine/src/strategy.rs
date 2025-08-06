use std::{hash::Hash, ops::RangeInclusive};

use ahash::AHashMap;

use crate::{
    card::{Card, Rank},
    game::GameState,
    hand::{Hand, Outcome},
    rules::{Rules, SurrenderType},
    shoe::Shoe,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayerAction {
    Hit,
    Stand,
    DoubleOrHit,
    DoubleOrStand,
    Split,
    Surrender,
}

#[derive(Clone, Copy)]
pub struct RoundEvs {
    pub hit: f64,
    pub stand: f64,
    pub double: f64,
    pub surrender: Option<f64>,
    pub split: Option<f64>,
}

impl RoundEvs {
    pub fn best(&self) -> (PlayerAction, f64) {
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
                best_value = split_val;
                best_action = PlayerAction::Split;
            }
        }

        if let Some(surrender_val) = self.surrender {
            if surrender_val > best_value {
                best_value = surrender_val;
                best_action = PlayerAction::Surrender;
            }
        }

        (best_action, best_value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct DealerHandKey<S: Shoe> {
    upcard: Card,
    shoe: S,
}

pub struct StrategyGenerator<S: Shoe> {
    rules: Rules,
    shoe: S,
    hit_cache: AHashMap<GameState<S>, f64>,
    stand_cache: AHashMap<GameState<S>, f64>,
    double_cache: AHashMap<GameState<S>, f64>,
    split_cache: AHashMap<GameState<S>, f64>,
    dealer_hand_cache: AHashMap<DealerHandKey<S>, Vec<(Hand, f64)>>,
    epsilon: f64,
}

impl<S: Shoe + Clone + Eq + Hash> StrategyGenerator<S> {
    pub fn new(rules: Rules, shoe: S) -> Self {
        Self {
            rules,
            shoe,
            hit_cache: AHashMap::new(),
            stand_cache: AHashMap::new(),
            double_cache: AHashMap::new(),
            split_cache: AHashMap::new(),
            dealer_hand_cache: AHashMap::new(),
            epsilon: 1e-5,
        }
    }

    pub fn get_dealer_hands(&mut self, state: &mut GameState<S>) -> Vec<(Hand, f64)> {
        let key = DealerHandKey {
            upcard: state.dealer_upcard,
            shoe: state.shoe.clone(),
        };
        if let Some(cached) = self.dealer_hand_cache.get(&key) {
            return cached.clone();
        }

        let mut map = AHashMap::<Hand, f64>::with_capacity(200);

        let mut stack = Vec::with_capacity(100);
        let mut start_hand = Hand::new();
        start_hand.add_card(&state.dealer_upcard);
        stack.push((start_hand, 1.0));

        while let Some((hand, weight)) = stack.pop() {
            if weight < self.epsilon {
                continue;
            }
            if self.rules.dealer_must_stand(&hand) {
                map.entry(hand)
                    .and_modify(|w| *w += weight)
                    .or_insert(weight);
                continue;
            }

            for (card, draw_weight) in state.shoe.get_draws() {
                state.shoe.remove_card(&card);

                let mut next_hand = hand.clone();
                next_hand.add_card(&card);
                stack.push((next_hand, weight * draw_weight));

                state.shoe.add_card(&card);
            }
        }

        let result: Vec<_> = map.into_iter().collect();

        self.dealer_hand_cache.insert(key, result.clone());

        result
    }

    pub fn expected_value_stand(&mut self, state: &mut GameState<S>, branch_weight: f64) -> f64 {
        if let Some(item) = self.stand_cache.get(state) {
            return *item;
        }
        let mut total_ev = 0.0;
        let mut total_weight = 0.0;

        for (dealer_hand, hand_weight) in self.get_dealer_hands(state) {
            let weight = branch_weight * hand_weight;
            if weight < self.epsilon {
                continue;
            }
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
            total_weight += hand_weight;
            total_ev += ev * hand_weight;
        }

        let ev = total_ev / total_weight;
        self.stand_cache.insert(state.clone(), ev);
        ev
    }

    pub fn expected_value_hit(&mut self, state: &mut GameState<S>, branch_weight: f64) -> f64 {
        if let Some(item) = self.hit_cache.get(&state) {
            return *item;
        }
        let mut total_ev = 0.0;
        let mut total_weight = 0.0;

        for (card, draw_weight) in state.shoe.get_draws() {
            let weight = branch_weight * draw_weight;
            if weight < self.epsilon {
                continue;
            }
            state.player_hand.add_card(&card);
            let ev = if state.player_hand.is_bust() {
                -1.0
            } else {
                state.shoe.remove_card(&card);
                let ev = if state.player_hand.value() == 21 {
                    self.expected_value_stand(state, weight)
                } else {
                    let ev_hit = self.expected_value_hit(state, weight);
                    let ev_stand = self.expected_value_stand(state, weight);
                    f64::max(ev_hit, ev_stand)
                };
                state.shoe.add_card(&card);
                ev
            };
            state.player_hand.remove_card(&card);

            total_ev += ev * draw_weight;
            total_weight += draw_weight;
        }

        let ev = total_ev / total_weight;
        self.hit_cache.insert(state.clone(), ev);
        ev
    }

    pub fn expected_value_double(&mut self, state: &mut GameState<S>, branch_weight: f64) -> f64 {
        if let Some(item) = self.double_cache.get(state) {
            return *item;
        }

        let mut total_ev = 0.0;
        let mut total_weight = 0.0;

        for (card, draw_weight) in state.shoe.get_draws() {
            let weight = branch_weight * draw_weight;
            if weight < self.epsilon {
                continue;
            }
            state.player_hand.add_card(&card);
            state.shoe.remove_card(&card);
            let ev = self.expected_value_stand(state, weight) * 2.0;
            state.shoe.add_card(&card);
            state.player_hand.remove_card(&card);

            total_ev += draw_weight * ev;
            total_weight += draw_weight;
        }

        let ev = total_ev / total_weight;
        self.double_cache.insert(state.clone(), ev);
        ev
    }

    pub fn expected_value_split(&mut self, state: &mut GameState<S>, branch_weight: f64) -> f64 {
        if let Some(item) = self.split_cache.get(state) {
            return *item;
        }

        let mut total_ev = 0.0;
        let mut total_weight = 0.0;

        state.split();
        for (card, draw_weight) in state.shoe.get_draws() {
            let weight = branch_weight * draw_weight;
            if weight < self.epsilon {
                continue;
            }
            let initial_value = state.player_hand.value();
            state.player_hand.add_card(&card);
            state.shoe.remove_card(&card);
            let mut ev = f64::max(
                self.expected_value_hit(state, weight),
                self.expected_value_stand(state, weight),
            );
            if self.rules.double_after_split_allowed {
                ev = ev.max(self.expected_value_double(state, weight))
            }
            if card.rank.value() == initial_value && state.splits < self.rules.max_splits {
                ev = ev.max(self.expected_value_split(state, weight))
            }
            state.shoe.add_card(&card);
            state.player_hand.remove_card(&card);

            total_ev += draw_weight * ev * 2.0;
            total_weight += draw_weight;
        }
        state.unsplit();

        let ev = total_ev / total_weight;
        self.split_cache.insert(state.clone(), ev);
        ev
    }

    pub fn expected_value_surrender(&mut self, state: &mut GameState<S>) -> f64 {
        match self.rules.surrender {
            SurrenderType::Early => -0.5,
            SurrenderType::Late => {
                let mut hand = Hand::new();
                hand.add_card(&state.dealer_upcard);

                let mut total_ev = 0.0;
                let mut total_weight = 0.0;

                for (card, draw_weight) in state.shoe.get_draws() {
                    hand.add_card(&card);

                    let ev = if hand.is_blackjack() { -1.0 } else { -0.5 };
                    total_ev += draw_weight * ev;
                    total_weight += draw_weight;

                    hand.remove_card(&card);
                }

                total_ev / total_weight
            }
            SurrenderType::None => unreachable!(),
        }
    }

    pub fn eval_round(
        &mut self,
        player_hand: Hand,
        dealer_upcard: Card,
        is_pair: bool,
    ) -> RoundEvs {
        let mut state = GameState {
            dealer_upcard,
            player_hand,
            shoe: self.shoe.clone(),
            splits: 0,
        };

        state.shoe.remove_card(&dealer_upcard);

        let evs = RoundEvs {
            hit: self.expected_value_hit(&mut state, 1.0),
            stand: self.expected_value_stand(&mut state, 1.0),
            double: self.expected_value_double(&mut state, 1.0),
            surrender: if self.rules.surrender != SurrenderType::None {
                Some(self.expected_value_surrender(&mut state))
            } else {
                None
            },
            split: if is_pair {
                Some(self.expected_value_split(&mut state, 1.0))
            } else {
                None
            },
        };

        state.shoe.add_card(&dealer_upcard);

        evs
    }

    pub fn hard_table(&mut self) -> StrategyTable {
        let mut table = StrategyTable::new(5, 21);

        for player_value in table.player_value_range() {
            for dealer_value in table.dealer_value_range() {
                let dealer_upcard = Card {
                    rank: Rank::from_value(dealer_value),
                };

                let player_hand = Hand::hard_from_value(player_value);

                let evs = self.eval_round(player_hand, dealer_upcard, false);
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

                let evs = self.eval_round(player_hand, dealer_upcard, false);
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

                let evs = self.eval_round(player_hand, dealer_upcard, true);
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
        let (action, ev) = evs.best();
        Self { action, ev, evs }
    }
}

pub struct StrategyTable {
    pub values: Vec<StrategyValue>,
    pub from: u8,
    pub to: u8,
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
                        surrender: None,
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
