fn main() {
    let res = exist(
        vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ],
        "SEE".to_owned(),
    );

    println!("{res}");

    let res = exist(
        vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ],
        "ABCB".to_owned(),
    );

    println!("{res}");
}

fn exist(board: Vec<Vec<char>>, word: String) -> bool {
    let first_letter = word.chars().next().unwrap();

    for y in 0..board.len() {
        for x in 0..board.get(0).unwrap().len() {
            if board[y][x] == first_letter {
                let mut visited = vec![vec![false; board.get(0).unwrap().len()]; board.len()];
                if find_next(&board, &mut visited, (x, y), word.clone(), 1) {
                    return true;
                }
            }
        }
    }

    false
}

fn find_next(
    board: &Vec<Vec<char>>,
    visited: &mut Vec<Vec<bool>>,
    start_pos: (usize, usize),
    word: String,
    current_char: usize,
) -> bool {
    if word.len() <= current_char {
        return true;
    }

    let (x, y) = start_pos;
    visited[y][x] = true;

    let target = word.chars().nth(current_char).unwrap();

    if let Some(left) = board
        .get(y)
        .unwrap_or(&vec![])
        .get(x.checked_sub(1).unwrap_or(usize::MAX))
    {
        if *left == target
            && !visited[y][x - 1]
            && find_next(board, visited, (x - 1, y), word.clone(), current_char + 1)
        {
            return true;
        }
    }

    if let Some(right) = board.get(y).unwrap_or(&vec![]).get(x + 1) {
        if *right == target
            && !visited[y][x + 1]
            && find_next(board, visited, (x + 1, y), word.clone(), current_char + 1)
        {
            return true;
        }
    }

    if let Some(top) = board
        .get(y.checked_sub(1).unwrap_or(usize::MAX))
        .unwrap_or(&vec![])
        .get(x)
    {
        if *top == target
            && !visited[y - 1][x]
            && find_next(board, visited, (x, y - 1), word.clone(), current_char + 1)
        {
            return true;
        }
    }

    if let Some(bottom) = board.get(y + 1).unwrap_or(&vec![]).get(x) {
        if *bottom == target
            && !visited[y + 1][x]
            && find_next(board, visited, (x, y + 1), word, current_char + 1)
        {
            return true;
        }
    }

    visited[y][x] = false;
    false
}
