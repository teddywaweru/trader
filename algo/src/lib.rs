use mt5::OHLC;
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

pub fn calculate_atr(ticks: &Vec<OHLC>) -> f32 {
    //caclulate average TR for all data, store the last one to use later
    // Calculate ATR as ( Prev_ATR(n-1) + last_ATR ) / n

    let mut true_ranges: Vec<f32> = vec![];
    for (idx, tick) in ticks.iter().enumerate() {
        if idx == 0 {
            continue;
        }
        let h_l = f32::abs(tick.high - tick.low);
        let h_cp = f32::abs(tick.high - ticks[idx - 1].close);
        let l_cp = f32::abs(tick.low - ticks[idx - 1].close);

        let true_range = f32::max(f32::max(h_l, h_cp), l_cp);
        true_ranges.push(true_range);
    }
    let atr = true_ranges.iter().sum::<f32>() / true_ranges.len() as f32;

    atr
}
