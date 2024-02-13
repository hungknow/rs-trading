use crate::errors::Result;
use crate::{indicators::ExponentialMovingAverage, Next, Period};

#[derive(Clone, Debug)]
pub struct RelativeStrengthIndex {
    period: usize,
    up_ema_indicator: ExponentialMovingAverage,
    down_ema_indicator: ExponentialMovingAverage,
    // prev_val: f64,
    // is_new: bool,
}

impl RelativeStrengthIndex {
    pub fn new(period: usize) -> Result<Self> {
        Ok(Self {
            period,
            up_ema_indicator: ExponentialMovingAverage::new().unwrap(),
            down_ema_indicator: ExponentialMovingAverage::new().unwrap(),
            // prev_val: 0.0,
            // is_new: true,
        })
    }

    // pub fn calc<'a>(&self, inputs: &'a [f64]) -> Vec<f64> {
    //     // Initialize with some small seed numbers to avoid division by zero
    //     let mut up = 0.1;
    //     let mut down = 0.1;
    //     let mut prev_val = -1.0;

    //     let mut result = vec![0.0; inputs.len()];
    //     let up_ema = self.up_ema_indicator.calc(inputs);
    //     let down_ema = self.down_ema_indicator.calc(inputs);

    //     for i in 0..inputs.len() {
    //         let input = inputs[i];
    //         if prev_val < 0.0 {
    //             // Initialize with some small seed numbers to avoid division by zero
    //             up = 0.1;
    //             down = 0.1;
    //         } else if input > prev_val {
    //             up = input - prev_val;
    //         } else {
    //             down = prev_val - input;
    //         }
    //         prev_val = input;

    //         let v = 100.0 * up_ema[i] / (up_ema[i] + down_ema[i]);
    //         result.push(v);
    //     }
    //     result
    // }
}

impl Period for RelativeStrengthIndex {
    fn period(&self) -> usize {
        self.period
    }
}

// impl Next<f64> for RelativeStrengthIndex {
//     type Output = f64;

//     fn next(&mut self, input: f64) -> Self::Output {
//         let mut up = 0.0;
//         let mut down = 0.0;

//         if self.is_new {
//             self.is_new = false;
//             // Initialize with some small seed numbers to avoid division by zero
//             up = 0.1;
//             down = 0.1;
//         } else {
//             if input > self.prev_val {
//                 up = input - self.prev_val;
//             } else {
//                 down = self.prev_val - input;
//             }
//         }

//         self.prev_val = input;
//         let up_ema = self.up_ema_indicator.next(up);
//         let down_ema = self.down_ema_indicator.next(down);
//         100.0 * up_ema / (up_ema + down_ema)
//     }
// }
