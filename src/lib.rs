mod blotto;

#[cfg(test)]
mod tests {
    use super::blotto;

    #[test]
    fn test_compare() {
        let mut alloc1 = blotto::Allocation::new(vec![5, 0, 0]);
        let mut alloc2 = blotto::Allocation::new(vec![5, 0, 0]);
        assert_eq!(alloc1.compare(&alloc2), blotto::BattleResult::Draw);

        alloc1.soldiers[0] = 2;
        alloc1.soldiers[1] = 2;
        alloc1.soldiers[2] = 1;
        // alloc1 lose battle 0, win 1 and 2
        assert_eq!(alloc1.compare(&alloc2), blotto::BattleResult::Win);

        alloc2.soldiers[0] = 1;
        alloc2.soldiers[1] = 2;
        alloc2.soldiers[2] = 2;
        // alloc1 win battle 0, draw 1 and lose 2
        assert_eq!(alloc1.compare(&alloc2), blotto::BattleResult::Draw);

        alloc2.soldiers[0] = 3;
        alloc2.soldiers[1] = 0;
        alloc2.soldiers[2] = 2;
        // alloc1 lose battle 0, win 1 and lose 2
        assert_eq!(alloc1.compare(&alloc2), blotto::BattleResult::Lose);
    }

    #[test]
    fn test_all_allocation() {
        let allocs = blotto::get_all_allocations(3, 5);
        assert_eq!(
            allocs,
            vec![
                [0, 0, 5],
                [0, 1, 4],
                [0, 2, 3],
                [0, 3, 2],
                [0, 4, 1],
                [0, 5, 0],
                [1, 0, 4],
                [1, 1, 3],
                [1, 2, 2],
                [1, 3, 1],
                [1, 4, 0],
                [2, 0, 3],
                [2, 1, 2],
                [2, 2, 1],
                [2, 3, 0],
                [3, 0, 2],
                [3, 1, 1],
                [3, 2, 0],
                [4, 0, 1],
                [4, 1, 0],
                [5, 0, 0],
            ]
        );

        // let allocs = blotto::get_all_allocations(5, 3);
        // for (idx, alloc) in allocs.iter().enumerate() {
        //     println!("{}: {:?}", idx, alloc);
        // }
    }

    #[test]
    fn test_game_construction() {
        let game = blotto::BlottoGame::new(3, 5);
    }
}
