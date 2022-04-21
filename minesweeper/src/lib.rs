pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let minefield: Vec<&[u8]> = minefield.iter().map(|&x| x.as_bytes()).collect();
    minefield
        .iter()
        .enumerate()
        .map(|(j, &row)| {
            row.iter()
                .enumerate()
                .map(|(i, &ch)| match ch {
                    b'*' => '*',
                    _ => match count_adjacent_mines(&minefield[..], i as i32, j as i32) {
                        0 => ' ',
                        c => char::from(c as u8 + b'0'),
                    },
                })
                .collect()
        })
        .collect()
}

fn count_adjacent_mines(minefield: &[&[u8]], x: i32, y: i32) -> usize {
    let possible_indices = vec![
        (x - 1, y - 1),
        (x, y - 1),
        (x + 1, y - 1),
        (x - 1, y),
        (x + 1, y),
        (x - 1, y + 1),
        (x, y + 1),
        (x + 1, y + 1),
    ];

    possible_indices
        .iter()
        .filter(|(i, j)| {
            i >= &0
                && j >= &0
                && i < &(minefield[0].len() as i32)
                && j < &(minefield.len() as i32)
                && minefield[*j as usize][*i as usize] == b'*'
        })
        .count()
}
