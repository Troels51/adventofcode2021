use crate::{day4, day4_data};

#[derive(Clone, Copy, Debug)]
pub struct Board { 
    rows : [[u32; 5] ;5],
    checks : [[bool; 5] ;5]
}
impl Board {
    pub fn new(rows : [[u32;5];5]) -> Board {
        Board { 
            rows : rows,
            checks : [
                [false, false, false, false, false],
                [false, false, false, false, false],
                [false, false, false, false, false],
                [false, false, false, false, false],
                [false, false, false, false, false] ]
        }
    }
    pub fn score(&self, number_that_was_called: u32) -> u32{
        let d : [ [(u32,bool);5]; 5] = self.rows.zip(self.checks).map(|(a,b)| a.zip(b));
        let sum : u32 = d.into_iter().flatten().filter_map(|(cell, mark)| (!mark).then(||cell)).sum();
        sum * number_that_was_called
    }

    pub fn mark(&mut self, number: u32) {
        match self.rows.into_iter().flatten().position(|&test| test == number) {
            Some(position) => {
                self.checks[position / 5][position % 5 ] = true;
            }
            None => (),
        }
    }
    fn check_rows(row : &[[bool;5];5] ) -> bool{
        row.iter().any(|row| row.iter().all(|&x|x))
    }
    pub fn is_winning(&self) -> bool {
        //Check rows
        Board::check_rows(&self.checks) || Board::check_rows(&transpose(self.checks))        
        //Check columns
    }
}
fn transpose<T>(v: [[T;5];5]) -> [[T;5];5]
    where T: Copy + Default
{
    let mut transposed = [[T::default();5];5];
    for i in 0..5 {
        for j in 0..5 {
            transposed[i][j] = v[j][i];
        }
    }
    transposed
}
#[derive(Clone)]
pub struct BingoData {
    pub numbers_drawn : Vec<u32>,
    pub boards : Vec<Board>,
}
impl BingoData {
    fn check_boards_for_number(&mut self, number: u32) -> Option<Board> {
        for (index,board) in self.boards.iter_mut().enumerate() {
            board.mark(number);
            if board.is_winning() {
                return Some(self.boards.remove(index));
            }
        }
        None
    }

    pub fn get_first_winning_board(&mut self) -> Option<(u32, Board)> {
        for number in self.numbers_drawn.clone() {
            let maybe_winning_board = self.check_boards_for_number(number);
            match maybe_winning_board {
                Some(board) => return Some((number, board)),
                None => (),
            }
        }
        None
    }
    pub fn get_last_winning_board(&mut self) -> Option<(u32, Board)> {
        let mut last_board_and_number: Option<(u32, Board)> = None;
        for number in self.numbers_drawn.clone() {
            for board in self.boards.iter_mut().filter(|b| !b.is_winning()) {
                board.mark(number);
                if board.is_winning() {
                    last_board_and_number = Some((number, board.clone()));
                }
            }
        }
        last_board_and_number
    }
}


#[test]
fn bingo_test() {
    let mut data : BingoData = BingoData { 
        numbers_drawn : vec![7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1],
        boards : vec![ Board::new(
            [
            [22 ,13, 17, 11, 0], 
            [8, 2, 23, 4, 24],
            [21, 9, 14, 16, 7],
            [6, 10, 3, 18, 5],
            [1, 12, 20, 15, 19],
            ]),
            Board::new(
            [
            [3,15,0,2,22],
            [9,18,13,17,5],
            [19,8,7,25,23],
            [20,11,10,24,4],
            [14,21,16,12,6],
            ]),
            Board::new(
            [
            [14,21,17,24,4],
            [10,16,15,9,19],
            [18,8,23,26,20],
            [22,11,13,6,5],
            [2,0,12,3,7],
            ])
            ]
     };
     let (number_that_was_called_last, board) = data.get_first_winning_board().unwrap();
     assert_eq!(board.score(number_that_was_called_last), 4512);

}
#[test]
fn mark_test() {
    let mut example = Board::new(
        [
        [22 ,13, 17, 11, 0], 
        [8, 2, 23, 4, 24],
        [21, 9, 14, 16, 7],
        [6, 10, 3, 18, 5],
        [1, 12, 20, 15, 19],
        ]);
    example.mark(1);
    assert_eq!(example.checks[4][0], true);
}

#[test]
fn score_test() {
    let mut example = Board::new(
        [
        [22 ,13, 17, 11, 0], 
        [8, 2, 23, 4, 24],
        [21, 9, 14, 16, 7],
        [6, 10, 3, 18, 5],
        [1, 12, 20, 15, 19],
        ]);
    example.mark(1);
    assert_eq!(example.checks[4][0], true);
    assert_eq!(example.score(1), example.rows.into_iter().flatten().sum::<u32>() - 1); //Sum everything except 1
    example.mark(2);
    assert_eq!(example.checks[1][1], true);
    assert_eq!(example.score(2), 2* (example.rows.into_iter().flatten().sum::<u32>() - 3)); //Sum everything except 1,2 and multiply by 2
}
#[test]
fn winning_row_test() {
    let mut example = Board::new(
        [
        [22 ,13, 17, 11, 0], 
        [8, 2, 23, 4, 24],
        [21, 9, 14, 16, 7],
        [6, 10, 3, 18, 5],
        [1, 12, 20, 15, 19],
        ]);
    assert_eq!(example.is_winning(), false);
    example.mark(22);
    example.mark(13);
    example.mark(17);
    example.mark(11);
    example.mark(0);
    assert_eq!(example.is_winning(), true);
}
#[test]
fn winning_column_test() {
    let mut example = Board::new(
        [
        [22 ,13, 17, 11, 0], 
        [8, 2, 23, 4, 24],
        [21, 9, 14, 16, 7],
        [6, 10, 3, 18, 5],
        [1, 12, 20, 15, 19],
        ]);
    assert_eq!(example.is_winning(), false);
    example.mark(22);
    example.mark(8);
    example.mark(21);
    example.mark(6);
    example.mark(1);
    assert_eq!(example.is_winning(), true);
}
#[test]
fn final_data_test() {
    let numbers  = vec![93,18,74,26,98,52,94,23,15,2,34,75,13,31,39,76,96,16,84,12,38,27,8,85,86,43,4,79,57,19,40,59,14,21,35,0,90,11,32,17,78,83,54,42,66,82,99,45,55,63,24,5,89,46,80,49,3,48,67,47,50,60,81,51,71,33,72,6,9,30,56,20,77,29,28,69,25,36,91,92,65,22,62,58,64,88,10,7,87,41,44,37,73,70,68,97,61,95,53,1];
    let mut bingodata : day4::BingoData = day4::BingoData{numbers_drawn: numbers, boards: day4_data::data4data()};
    let (score, winning_board) = bingodata.get_first_winning_board().unwrap();
    let (last_score, last_winning_board) = bingodata.get_last_winning_board().unwrap();

    println!("Winning score is : {}", winning_board.score(score));
    println!("Last Winning score is : {}", last_winning_board.score(last_score));
}