pub struct Allocation {
    pub soldiers: Vec<i64>,
}

impl Allocation {
    pub fn new(battle_count: usize, soldier_count: usize) -> Self {
        let mut soldiers: Vec<i64> = vec![0; battle_count];
        soldiers[0] = soldier_count as i64;
        return Allocation { soldiers: soldiers };
    }
}
