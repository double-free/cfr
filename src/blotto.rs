use std::collections::HashMap;

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct Allocation {
    pub soldiers: Vec<usize>,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum BattleResult {
    Win,
    Lose,
    Draw,
}

// helper function, recursive call
fn helper(result: &mut Vec<Vec<usize>>, arr: &mut Vec<usize>, box_index: usize, balls: usize) {
    if box_index == arr.len() - 1 {
        // push all remaining balls into the last box
        arr[box_index] = balls;
        result.push(arr.clone());
        return;
    }

    // try to place 0 to n balls to current box, and move to next box
    for i in 0..balls + 1 {
        arr[box_index] = i;
        helper(result, arr, box_index + 1, balls - i);
    }
}

// place n balls into m boxes
// total permutations: C_{n+m-1}^{m-1}
pub fn get_all_allocations(box_count: usize, ball_count: usize) -> Vec<Vec<usize>> {
    let mut arr = vec![0; box_count];
    let mut result = Vec::<Vec<usize>>::new();

    helper(&mut result, &mut arr, 0, ball_count);

    return result;
}

impl Allocation {
    pub fn new(alloc: Vec<usize>) -> Self {
        return Allocation { soldiers: alloc };
    }

    pub fn compare(&self, another: &Allocation) -> BattleResult {
        assert!(self.soldiers.len() == another.soldiers.len());
        let mut win = 0;
        let mut lose = 0;
        for i in 0..self.soldiers.len() {
            if self.soldiers[i] > another.soldiers[i] {
                win += 1;
            } else if self.soldiers[i] < another.soldiers[i] {
                lose += 1;
            }
        }

        if win == lose {
            return BattleResult::Draw;
        }

        if win > lose {
            return BattleResult::Win;
        }

        return BattleResult::Lose;
    }
}

pub trait BlottoPlayer {
    fn on_register(&mut self, player_id: usize, game: &BlottoGameMeta);
    fn decide_allocation(&self, game: &BlottoGameMeta, round: usize) -> Allocation;
    fn handle_result(&mut self, game: &BlottoGameMeta, result: &BlottoGameResult);
    fn on_exit_game(&self, game: &BlottoGameMeta);
}

pub struct BlottoGameResult {
    // key: player id, value: allocation
    pub allocations: HashMap<usize, Allocation>,
}

pub struct BlottoGameMeta {
    pub battle_count: usize,
    pub soldier_count: usize,
    // key: strategy, value: strategy id
    pub strategy_to_id: HashMap<Allocation, usize>,
    pub id_to_strategy: HashMap<usize, Allocation>,

    // strategy id as indexy
    pub game_matrix: Vec<Vec<BattleResult>>,
}

pub struct BlottoGame {
    // not changed throughout the game
    pub game_meta: BlottoGameMeta,

    players: Vec<Box<dyn BlottoPlayer>>,
}

impl BlottoGame {
    pub fn new(battle_count: usize, soldier_count: usize) -> Self {
        let allocations: Vec<Allocation> = get_all_allocations(battle_count, soldier_count)
            .into_iter()
            .map(|soldiers| {
                return Allocation { soldiers: soldiers };
            })
            .collect();

        let mut game_matrix: Vec<Vec<BattleResult>> = Vec::new();
        for _ in 0..allocations.len() {
            game_matrix.push(vec![BattleResult::Draw; allocations.len()]);
        }
        for i in 0..allocations.len() {
            for j in i..allocations.len() {
                let strategy_i = &allocations[i];
                let strategy_j = &allocations[j];
                game_matrix[i][j] = strategy_i.compare(strategy_j);
                game_matrix[j][i] = match game_matrix[i][j] {
                    BattleResult::Win => BattleResult::Lose,
                    BattleResult::Lose => BattleResult::Win,
                    BattleResult::Draw => BattleResult::Draw,
                };
            }
        }

        let mut strategy_to_id = HashMap::new();
        let mut id_to_strategy = HashMap::new();
        for (idx, allocation) in allocations.iter().enumerate() {
            strategy_to_id.insert(allocation.clone(), idx);
            id_to_strategy.insert(idx, allocation.clone());
        }

        return BlottoGame {
            game_meta: BlottoGameMeta {
                battle_count: battle_count,
                soldier_count: soldier_count,
                strategy_to_id: strategy_to_id,
                id_to_strategy: id_to_strategy,
                game_matrix: game_matrix,
            },

            players: Vec::new(),
        };
    }

    pub fn add_player(&mut self, mut player: Box<dyn BlottoPlayer>) {
        player.on_register(self.players.len(), &self.game_meta);
        self.players.push(player);
    }

    pub fn start(&mut self, total_round: usize) {
        for round in 0..total_round {
            let mut result = BlottoGameResult {
                allocations: HashMap::new(),
            };

            for (player_id, player) in self.players.iter_mut().enumerate() {
                let allocation = player.decide_allocation(&self.game_meta, round);
                result.allocations.insert(player_id, allocation);
            }

            for player in self.players.iter_mut() {
                player.handle_result(&self.game_meta, &result);
            }
        }

        for player in self.players.iter_mut() {
            player.on_exit_game(&self.game_meta);
        }
    }
}
