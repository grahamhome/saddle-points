use std::cmp::Ordering;
use std::collections::HashMap;
mod tests;

struct Outlier {
    value: u64,
    indices: Vec<usize>,
}

impl Outlier {
    fn new(value: u64, index: usize) -> Self {
        Outlier {
            value,
            indices: vec![index],
        }
    }

    fn replace(&mut self, value: u64, index: usize) {
        self.value = value;
        self.indices = vec![index]
    }

    fn update(&mut self, index: usize) {
        self.indices.push(index)
    }
}

pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut row_peaks: HashMap<usize, Outlier> = HashMap::new();
    let mut column_valleys: HashMap<usize, Outlier> = HashMap::new();
    input.iter().enumerate().for_each(|(row_index, row)| {
        row.iter().enumerate().for_each(|(col_index, &value)| {
            row_peaks
                .entry(row_index)
                .and_modify(|outlier| match value.cmp(&outlier.value) {
                    Ordering::Greater => outlier.replace(value, col_index),
                    Ordering::Equal => outlier.update(col_index),
                    Ordering::Less => (),
                })
                .or_insert(Outlier::new(value, col_index));
            column_valleys
                .entry(col_index)
                .and_modify(|outlier| match value.cmp(&outlier.value) {
                    Ordering::Less => outlier.replace(value, row_index),
                    Ordering::Equal => outlier.replace(value, row_index),
                    Ordering::Greater => (),
                })
                .or_insert(Outlier::new(value, col_index));
        })
    });

    row_peaks
        .iter()
        .flat_map(|(row_index, outlier)| {
            outlier.indices.iter().filter_map(|column_index| {
                if column_valleys.get(column_index).unwrap().value == outlier.value {
                    Some((row_index.clone(), column_index.clone()))
                } else {
                    None
                }
            })
        })
        .collect()
}
