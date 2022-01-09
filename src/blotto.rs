pub struct Allocation {
    pub soldiers: Vec<i64>,
}

#[derive(Debug, PartialEq)]
pub enum Result {
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
    pub fn new(alloc: Vec<i64>) -> Self {
        return Allocation { soldiers: alloc };
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
