// pivothigh(10,10) will search for high price 
// that was not exceeded during 10 bars to the left (past data) 
// and 10 bars to the right (future data).
// lag of this indicator = rightbars+1, pattern = leftbars+rightbars+1

use std::collections::VecDeque;

use crate::Next;

#[derive(Clone, Debug)]
pub struct PivotHigh {
    previous_num: u16,
    next_num: u16,
    bars: VecDeque<f64>,
}

impl PivotHigh {
    pub fn new(previous_num: u16, next_num: u16) -> Self {
        Self{
            previous_num: previous_num,
            next_num: next_num,
            bars: VecDeque::<f64>::new(),
        }
    }
}

impl Next<f64> for PivotHigh {
    type Output = Option<f64>;

    fn next(&mut self, input: f64) -> Self::Output {
        self.bars.push_front(input);
    
        // If the number of bars hasn't been enough,
        // Append to the array
        if self.bars.len() < (self.previous_num + self.next_num + 1).into() {
            return None
        }

        // self.bars.pop_back();

        // The number of bars is enough, finding the pivot bar
        let ref_val = self.bars[self.next_num.into()];
        for i in 0..self.next_num.into() {
            let v = *self.bars.get(i).unwrap();
            if ref_val <= v {
                return None;
            }
        }

        let starting_index: usize = (self.next_num + 1).into();
        for i in 0..self.previous_num.into() {
            let v = *self.bars.get(starting_index + i).unwrap();
            if ref_val < v {
                return None;
            }
        }
        
        Some(ref_val)
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_next() {
        let mut pivotHigh = PivotHigh::new(2, 2);

        assert_eq!(pivotHigh.next(1.0), None);
        assert_eq!(pivotHigh.next(2.0), None);
        assert_eq!(pivotHigh.next(3.0), None);
        assert_eq!(pivotHigh.next(2.0), None);
        assert_eq!(pivotHigh.next(1.0), Some(3.0));
        assert_eq!(pivotHigh.next(6.0), None);
        assert_eq!(pivotHigh.next(7.0), None);
        assert_eq!(pivotHigh.next(8.0), None);
        assert_eq!(pivotHigh.next(9.0), None);
        assert_eq!(pivotHigh.next(8.0), None);
        assert_eq!(pivotHigh.next(9.0), None);
        assert_eq!(pivotHigh.next(10.0), None);
        assert_eq!(pivotHigh.next(7.0), None);
        assert_eq!(pivotHigh.next(8.0), Some(10.0));
    }
}