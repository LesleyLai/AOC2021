const INPUT: &'static str = include_str!("../input.txt");

// When a number is marked, I simply use bitwise not on it.
// Thus, all negative numbers on a board are marked
type Board = [[i32; 5]; 5];

fn parse() -> (Vec<i32>, Vec<Board>) {
    let mut lines = INPUT.lines();
    let draw_numbers =
        lines
            .next().unwrap()
            .split(",")
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
    let mut boards: Vec<[[i32; 5]; 5]> = Vec::new();
    while lines.next().is_some() {
        let rows = [lines.next(), lines.next(), lines.next(), lines.next(), lines.next()]
            .map(|line| line.unwrap());
        let board = rows.map(|line| {
            let mut columns = line.split(" ")
                .filter(|token| token != &"")
                .map(|token| token.parse::<i32>().unwrap());
            [columns.next(), columns.next(), columns.next(), columns.next(), columns.next()]
                .map(|n| n.unwrap())
        });
        boards.push(board)
    }
    (draw_numbers, boards)
}

fn at_least_a_row_marked(board: &Board) -> bool
{
    board.iter().find(|row| row.iter().all(|num| num < &0)).is_some()
}

fn transpose(board: &Board) -> Board
{
    let mut result: Board = [[0; 5]; 5];
    for (y, col) in board.iter().enumerate() {
        for (x, num) in col.iter().enumerate() {
            result[x][y] = *num;
        }
    }
    result
}

fn is_winner(board: &Board) -> bool
{
    at_least_a_row_marked(&board) || at_least_a_row_marked(&transpose(&board))
}

fn advance(board: &Board, draw_number: i32) -> Board
{
    let mut result = board.clone();
    for (y, col) in board.iter().enumerate() {
        for (x, num) in col.iter().enumerate() {
            if num == &draw_number {
                result[y][x] = !(*num);
                break;
            }
        }
    }
    result
}

fn sum_unmarked(board: &Board) -> i32 {
    board.iter().flatten().filter(|n| **n >= 0).sum()
}

fn part1(draw_numbers: &[i32], boards: &Vec<Board>) {
    let mut boards = boards.clone();
    for draw_number in draw_numbers.iter() {
        boards = boards.iter().map(|board| advance(board, *draw_number)).collect();
        let winner = boards.iter().find(|board| is_winner(board));
        if winner.is_some() {
            let sum = sum_unmarked(winner.unwrap());
            println!("Part 1: {}", sum * draw_number);
            break;
        }
    }
}

fn part2(draw_numbers: &[i32], boards: &Vec<Board>) {
    let mut boards = boards.clone();
    for draw_number in draw_numbers.iter() {
        boards = boards
            .iter()
            .map(|board| advance(board, *draw_number)).collect();

        if boards.len() > 1 {
            boards = boards
                .into_iter()
                .filter(|board| !is_winner(board))
                .collect();
        } else if is_winner(&boards[0]) {
            let sum = sum_unmarked(&boards[0]);
            println!("Part 2: {}", sum * draw_number);
            break;
        }
    }
}


fn main() {
    let (draw_numbers, boards) = parse();
    part1(&*draw_numbers, &boards);
    part2(&*draw_numbers, &boards);
}
