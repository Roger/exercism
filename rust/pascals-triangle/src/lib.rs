use std::iter::once;

pub struct PascalsTriangle {
    row_count: usize,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        Self {
            row_count: row_count as usize,
        }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        let mut rows: Vec<Vec<u32>> = Vec::with_capacity(self.row_count);

        (0..self.row_count).for_each(|_| {
            let next = match rows.last() {
                None => vec![1],
                // Add ones to head/tail and sum pairs
                Some(prev) => once(1)
                    .chain(prev.windows(2).map(|pair| pair[0] + pair[1]))
                    .chain(once(1))
                    .collect(),
            };

            rows.push(next);
        });
        rows
    }
}
