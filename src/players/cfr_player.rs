use crate::blotto;

use rand::{thread_rng, Rng};
use std::collections::HashMap;

pub struct CfrPlayer {
    player_id: usize,

    // key: player id, value: regret sum for each strategy id
    regret_sum: HashMap<usize, Vec<i64>>,
}

impl CfrPlayer {
    pub fn new() -> Self {
        return Self {
            player_id: 0,
            regret_sum: HashMap::new(),
        };
    }

    fn get_cfr_action(&self) -> Option<usize> {
        // TODO: deal with multi players
        let mut actions = Vec::new();

        // return action id from regrets
        let get_action_from_regrets = |regrets: &Vec<i64>| -> Option<usize> {
            let mut actions = Vec::new();
            let mut positive_regrets = Vec::new();

            let mut regret_sum: u64 = 0;
            for (action_id, regret) in regrets.iter().enumerate() {
                if *regret <= 0 {
                    continue;
                }
                regret_sum += *regret as u64;
                actions.push(action_id);
                positive_regrets.push(*regret as u64);
            }

            if regret_sum == 0 {
                // no positive regret
                return None;
            }

            // only contains positive regret actions
            let mut rng = thread_rng();
            let n: u64 = rng.gen_range(0..regret_sum);

            let mut s: u64 = 0;
            for i in 0..actions.len() {
                s += positive_regrets[i];
                if s > n {
                    return Some(actions[i]);
                }
            }

            return None;
        };

        if self.regret_sum.len() == 0 {
            return None;
        }
        for (_, action_regrets) in self.regret_sum.iter() {
            actions.push(get_action_from_regrets(action_regrets));
        }

        // assumption: only 1 opponent
        return actions[0];
    }
}

impl blotto::BlottoPlayer for CfrPlayer {
    fn decide_allocation(
        &self,
        game: &blotto::BlottoGameMeta,
        _round: usize,
    ) -> blotto::Allocation {
        let cfr_action_id = self.get_cfr_action();

        if cfr_action_id.is_some() {
            return game.id_to_strategy[&cfr_action_id.unwrap()].clone();
        }

        // no valid cfr action, use default
        let mut alloc = vec![0; game.battle_count];
        alloc[0] = game.soldier_count;
        // first round, return something random
        return blotto::Allocation { soldiers: alloc };
    }

    fn handle_result(&mut self, game: &blotto::BlottoGameMeta, result: &blotto::BlottoGameResult) {
        let my_action = &result.allocations[&self.player_id];
        let my_action_id = game.strategy_to_id[&my_action];

        for (oppo_id, action) in result.allocations.iter() {
            if oppo_id == &self.player_id {
                // skip self
                continue;
            }

            let oppo_action_id = game.strategy_to_id[action];
            let my_action_payoff = match game.game_matrix[oppo_action_id][my_action_id] {
                // oppo wins
                blotto::BattleResult::Win => -1,
                blotto::BattleResult::Draw => 0,
                // oppo loses
                blotto::BattleResult::Lose => 1,
            };

            if self.regret_sum.contains_key(oppo_id) == false {
                self.regret_sum
                    .insert(oppo_id.clone(), vec![0; game.game_matrix.len()]);
            }

            for (strategy_id, result) in game.game_matrix[oppo_action_id].iter().enumerate() {
                // reverse the payoff because we use "oppo to me" result
                let payoff: i64 = match result {
                    // oppo wins
                    blotto::BattleResult::Win => -1,
                    blotto::BattleResult::Draw => 0,
                    // oppo loses
                    blotto::BattleResult::Lose => 1,
                };

                self.regret_sum.get_mut(oppo_id).unwrap()[strategy_id] += payoff - my_action_payoff;
            }
        }
    }

    fn on_register(&mut self, player_id: usize, _game: &blotto::BlottoGameMeta) {
        self.player_id = player_id;
    }

    fn on_exit_game(&self, game: &blotto::BlottoGameMeta) {
        // print regret matrix
        for (oppo_id, action_regrets) in self.regret_sum.iter() {
            let mut action_candidates = Vec::new();
            for (action_id, regret) in action_regrets.iter().enumerate() {
                if *regret > 0 {
                    action_candidates.push(game.id_to_strategy[&action_id].clone());
                }
            }

            println!(
                "player {}: for opponent {}, regret sum for each action {:?}, \
                action candidates {:?}",
                self.player_id, oppo_id, action_regrets, action_candidates
            );
        }
    }
}
