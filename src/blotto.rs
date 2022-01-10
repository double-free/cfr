use std::collections::HashMap;

#[derive(Hash, PartialEq, Eq)]
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

pub struct BlottoGame {
    battle_count: usize,
    soldier_count: usize,

    // key: strategy, value: strategy id
    strategies: HashMap<Allocation, usize>,
    game_matrix: Vec<Vec<BattleResult>>,
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
        for i in 0..allocations.len() {
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

        let mut strategies = HashMap::new();
        for (idx, allocation) in allocations.into_iter().enumerate() {
            strategies.insert(allocation, idx);
        }

        return BlottoGame {
            battle_count: battle_count,
            soldier_count: soldier_count,
            strategies: strategies,
            game_matrix: game_matrix,
        };
    }
}
