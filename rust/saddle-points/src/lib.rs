pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let row_len = match input[0].len() {
        // It's an empty matrix, just return empty Vec
        0 => return vec![],
        len => len,
    };

    // Get minimum values from rows
    let mins: Vec<_> = (0..row_len)
        .map(|x| input.iter().map(|row| row[x]).min().unwrap())
        .collect();

    // Get saddle points in a row
    let map_row = |(y, row): (usize, &Vec<_>)| -> Vec<_> {
        // Find max in the row
        let max = row.iter().max().unwrap();

        // Filter max values in the row that are minimum in the col
        row.iter()
            .enumerate()
            .filter(|(x, v)| **v == mins[*x] && *v == max)
            .map(|(x, _)| (y, x))
            .collect()
    };

    input.iter().enumerate().map(map_row).flatten().collect()
}
