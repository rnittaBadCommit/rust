

pub fn median(values: &[i32]) -> Option<f64> {
    if values.is_empty() { return None };

    let mut values = values.to_vec();
    values.sort();
    let len = values.len();
    if len % 2 == 0 {
        Some((values[(len / 2) - 1] + values[len / 2]) as f64 / 2f64)
    } else {
        Some(values[len / 2] as f64)
    }
}

use std::collections::HashMap;

pub fn mode(values: &[i32]) -> Option<i32> {
    if values.is_empty() { return None; }

    let mut values_hashmap: HashMap<i32, usize> = HashMap::new();

    for x in values {
        let count = values_hashmap.entry(*x).or_insert(0);
        *count += 1;
    }

    let mut max_freq = 0;
    let mut max_freq_value = None;

    for (value, freq) in values_hashmap {
        if freq > max_freq {
            max_freq = freq;
            max_freq_value = Some(value);
        }
    }
    max_freq_value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn median_returns_middle_value_after_sorting() {
        assert_eq!(median(&[3, 1, 2]), Some(2.0));
    }

    #[test]
    fn median_averages_two_middle_values_for_even_len() {
        assert_eq!(median(&[4, 1, 2, 3]), Some(2.5));
    }

    #[test]
    fn mode_returns_most_frequent_value() {
        assert_eq!(mode(&[1, 2, 2, 3]), Some(2));
    }

    #[test]
    fn empty_input_has_no_median_or_mode() {
        assert_eq!(median(&[]), None);
        assert_eq!(mode(&[]), None);
    }
}
