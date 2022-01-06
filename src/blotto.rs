pub struct Allocation {
    pub soldiers: Vec<i64>,
}

#[derive(Debug, PartialEq)]
pub enum Result {
    Win,
    Lose,
    Draw,
}

impl Allocation {
    pub fn new(battle_count: usize, soldier_count: usize) -> Self {
        let mut soldiers: Vec<i64> = vec![0; battle_count];
        soldiers[0] = soldier_count as i64;
        return Allocation { soldiers: soldiers };
    }

    pub fn compare(&self, another: &Allocation) -> Result {
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
            return Result::Draw;
        }

        if win > lose {
            return Result::Win;
        }

        return Result::Lose;
    }
}
