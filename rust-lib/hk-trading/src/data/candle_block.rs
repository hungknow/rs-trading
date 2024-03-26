use chrono::{DateTime, Utc};

use super::Candles;
use crate::errors::HkError;
use std::cell::RefCell;

pub struct OhlcOrderBlock {
    // pub order: Vec<i64>,
    pub ohlc_list: Vec<RefCell<Box<Candles>>>,
    // pub recent: Option<bool>,
    // pub oldest: Option<bool>,
}

impl OhlcOrderBlock {
    pub fn new() -> OhlcOrderBlock {
        OhlcOrderBlock {
            ohlc_list: Vec::new(),
        }
    }

    // pub fn get_ohlc_by_time_range(
    //     &self,
    //     from_time: &DateTime<Utc>,
    //     to_time: &DateTime<Utc>,
    // ) -> Vec<Candles> {
    //     let mut result = Vec::new();
    //     for candles in &self.ohlc_list {
    //         let candles = candles.borrow();
    //         let from_time_cmp = candles.open_times[0].cmp(from_time);
    //         let to_time_cmp = candles.open_times[candles.open_times.len() - 1].cmp(to_time);
    //         if (from_time_cmp == std::cmp::Ordering::Greater
    //             || from_time_cmp == std::cmp::Ordering::Equal)
    //             && to_time_cmp == std::cmp::Ordering::Less
    //         {
    //             result.push(candles.clone());
    //         }
            
    //     }
    //     result
    // }

    pub fn merge_block(&mut self, candles: RefCell<Box<Candles>>) -> Result<(), HkError> {
        self.ohlc_list.push(candles);

        let ohlc_time_desc = self.ohlc_list[0].borrow().time_desc();
        if ohlc_time_desc == None {
            return Err(HkError::HkDataError(
                format!("merge_block: failed to detect the time order for candles").to_owned(),
            ));
        }

        let is_desc = ohlc_time_desc.unwrap();

        // Sort blocks so that the most recent one come first
        self.ohlc_list.sort_by(|a, b| {
            if is_desc {
                b.borrow().open_times[0].cmp(&a.borrow().open_times[0])
            } else {
                a.borrow().open_times[0].cmp(&b.borrow().open_times[0])
            }
        });

        let resolution_seconds = self.ohlc_list[0]
            .borrow()
            .resolution()
            .unwrap()
            .to_seconds();

        // Merge adjacent blocks
        let mut i = 0;
        while i < self.ohlc_list.len() - 1 {
            let mut should_remove: i64 = -1;

            // Since we know the start of a is more recent than the start of b, they'll overlap if the last post in a is
            // older than the first post in b
            {
                let (left, right) = self.ohlc_list.split_at_mut(i + 1);
                let mut current_candles = left[0].borrow_mut();
                let current_candles_end_at =
                    current_candles.open_times[current_candles.open_times.len() - 1];

                let next_candles = right[0].borrow();
                let next_candles_start_at = next_candles.open_times[0];

                let mut should_merge = false;
                let time_diff =
                    current_candles_end_at.timestamp() - next_candles_start_at.timestamp();
                // Check if the last ohlc of the first candles is the continous of the first ohlc of the next candles
                if time_diff.abs() == resolution_seconds {
                    should_merge = true;
                }

                // decide merge
                let order = current_candles_end_at.cmp(&next_candles_start_at);
                should_merge |= order == std::cmp::Ordering::Equal;
                match ohlc_time_desc.unwrap() {
                    true => {
                        should_merge |= order == std::cmp::Ordering::Less;
                    }
                    false => {
                        should_merge |= order == std::cmp::Ordering::Greater;
                    }
                }
                if should_merge {
                    // The blocks overlap, so combine them and remove the second block
                    should_remove = i as i64 + 1;
                    for j in 0..next_candles.open_times.len() {
                        current_candles
                            .push_data_overlapped(
                                next_candles.open_times[j],
                                next_candles.opens[j],
                                next_candles.highs[j],
                                next_candles.lows[j],
                                next_candles.closes[j],
                                next_candles.volumes[j],
                            )
                            .unwrap();
                    }
                    // Do another iteration on this index since it may need to be merged into the next
                } else {
                    // The blocks don't overlap, so move on to the next one
                    i += 1;
                }
            }

            if should_remove >= 0 {
                self.ohlc_list.remove(should_remove as usize);
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::convert_i64_to_datetime_utc;

    use super::*;

    #[test]
    fn test_merge_block_asc_order() {
        let mut block = OhlcOrderBlock {
            ohlc_list: Vec::new(),
        };
        let data = vec![
            (1, 1.0, 2.0, 3.0, 4.0, Some(1.0)),
            (2, 1.0, 2.0, 3.0, 4.0, Some(1.0)),
            (3, 1.0, 2.0, 3.0, 4.0, Some(1.0)),
            (4, 1.0, 2.0, 3.0, 4.0, Some(1.0)),
            (5, 1.0, 2.0, 3.0, 4.0, Some(1.0)),
            (6, 1.0, 2.0, 3.0, 4.0, Some(1.0)),
            // The opentime overlap with the last candle
            (6, 1.0, 2.0, 3.0, 5.0, Some(1.0)),
            (7, 1.0, 2.0, 3.0, 4.0, Some(1.0)),
            (8, 1.0, 2.0, 3.0, 4.0, Some(1.0)),
        ];

        let mut candles = Box::new(Candles::new());
        for i in 0..3 {
            candles
                .push_data_non_overlapped(
                    convert_i64_to_datetime_utc(data[i].0),
                    data[i].1,
                    data[i].2,
                    data[i].3,
                    data[i].4,
                    data[i].5,
                )
                .unwrap();
        }

        block.merge_block(RefCell::new(candles));
        assert_eq!(block.ohlc_list.len(), 1);
        assert_eq!(block.ohlc_list[0].borrow().open_times.len(), 3);

        let mut candles = Box::new(Candles::new());
        for i in 3..6 {
            candles
                .push_data_non_overlapped(
                    convert_i64_to_datetime_utc(data[i].0),
                    data[i].1,
                    data[i].2,
                    data[i].3,
                    data[i].4,
                    data[i].5,
                )
                .unwrap();
        }

        block.merge_block(RefCell::new(candles));
        assert_eq!(block.ohlc_list.len(), 1);
        assert_eq!(block.ohlc_list[0].borrow().open_times.len(), 6);

        let mut candles = Box::new(Candles::new());
        for i in 6..9 {
            candles
                .push_data_non_overlapped(
                    convert_i64_to_datetime_utc(data[i].0),
                    data[i].1,
                    data[i].2,
                    data[i].3,
                    data[i].4,
                    data[i].5,
                )
                .unwrap();
        }
        block.merge_block(RefCell::new(candles));
        assert_eq!(block.ohlc_list.len(), 1);
        assert_eq!(block.ohlc_list[0].borrow().open_times.len(), 8);
        // Check if the same candle is updated with the new candle
        assert_eq!(block.ohlc_list[0].borrow().closes[5], 5.0);
    }

    #[test]
    fn test_merge_block_desc_order() {
        let mut block = OhlcOrderBlock {
            ohlc_list: Vec::new(),
        };
        let data = vec![
            (8, 1.0, 2.0, 3.0, 4.0, Some(1.0)),
            (7, 1.0, 2.0, 3.0, 4.0, Some(1.0)),
            (6, 1.0, 2.0, 3.0, 5.0, Some(1.0)),
            (5, 1.0, 2.0, 3.0, 4.0, Some(1.0)),
            (4, 1.0, 2.0, 3.0, 4.0, Some(1.0)),
            (3, 1.0, 2.0, 3.0, 5.0, Some(1.0)),
            // The opentime overlap with the last candle
            (3, 1.0, 2.0, 3.0, 10.0, Some(1.0)),
            (2, 1.0, 2.0, 3.0, 4.0, Some(1.0)),
            (1, 1.0, 2.0, 3.0, 4.0, Some(1.0)),
        ];

        let mut candles = Box::new(Candles::new());
        for i in 0..3 {
            candles
                .push_data_non_overlapped(
                    convert_i64_to_datetime_utc(data[i].0),
                    data[i].1,
                    data[i].2,
                    data[i].3,
                    data[i].4,
                    data[i].5,
                )
                .unwrap();
        }

        block.merge_block(RefCell::new(candles));
        assert_eq!(block.ohlc_list.len(), 1);
        assert_eq!(block.ohlc_list[0].borrow().open_times.len(), 3);

        let mut candles = Box::new(Candles::new());
        for i in 3..6 {
            candles
                .push_data_non_overlapped(
                    convert_i64_to_datetime_utc(data[i].0),
                    data[i].1,
                    data[i].2,
                    data[i].3,
                    data[i].4,
                    data[i].5,
                )
                .unwrap();
        }

        block.merge_block(RefCell::new(candles));
        assert_eq!(block.ohlc_list.len(), 1);
        assert_eq!(block.ohlc_list[0].borrow().open_times.len(), 6);

        let mut candles = Box::new(Candles::new());
        for i in 6..9 {
            candles
                .push_data_non_overlapped(
                    convert_i64_to_datetime_utc(data[i].0),
                    data[i].1,
                    data[i].2,
                    data[i].3,
                    data[i].4,
                    data[i].5,
                )
                .unwrap();
        }
        block.merge_block(RefCell::new(candles));
        assert_eq!(block.ohlc_list.len(), 1);
        assert_eq!(block.ohlc_list[0].borrow().open_times.len(), 8);
        // Check if the same candle is updated with the new candle
        assert_eq!(block.ohlc_list[0].borrow().closes[5], 10.0);
    }
}
