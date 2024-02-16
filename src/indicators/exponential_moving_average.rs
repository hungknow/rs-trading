use serde::{Deserialize, Serialize};

use crate::{errors::Result, Reset};
use std::fmt;

use super::traits::Indicator;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExponentialMovingAverageState {
    period: usize,
    k: f64,
    current: f64,
    // current_timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExponentialMovingAverage {
    // period: usize,
    // k: f64,
    // current: f64,
    // is_new: bool,
}

impl ExponentialMovingAverageState {
    pub fn new(period: usize) -> ExponentialMovingAverageState {
        ExponentialMovingAverageState {
            period: period,
            k: 2.0 / (period + 1) as f64,
            current: -1.0,
            // current_timestamp: Utc::now(),
        }
    }
}

impl ExponentialMovingAverage {
    pub fn new() -> Self {
        // match period {
        //     0 => Err(TaError::InvalidParameter),
        //     _ => Ok(Self {
        //         // period,
        //         // k: 2.0 / (period + 1) as f64,
        //         // current: 0.0,
        //         // is_new: true,
        //     }),
        // }
        Self {}
    }

    // pub fn new_state(&self) -> Rc<ExponentialMovingAverageState> {
    //     Rc::new(ExponentialMovingAverageState {
    //         period: self.period(),
    //         k: 2.0 / (self.period() + 1) as f64,
    //         current: -1.0,
    //         // current_timestamp: Utc::now(),
    //     })
    // }

    // pub fn calc<'a>(&self, inputs: &'a [f64]) -> Vec<f64> {
    //     let data_len = inputs.len();
    //     let mut current = inputs[data_len - 1];
    //     let mut result = vec![0.0; data_len];
    //     result[data_len - 1] = current;
    //     for i in (0..data_len - 1).rev() {
    //         current = self.k * inputs[i] + (1.0 - self.k) * current;
    //         result[i] = current;
    //     }
    //     result
    // }
}

impl Indicator for ExponentialMovingAverage {
    type InputType = f64;
    type OutputType = f64;
    type StateType = ExponentialMovingAverageState;

    fn next(
        &self,
        input: (&ExponentialMovingAverageState, Self::InputType),
    ) -> (ExponentialMovingAverageState, Self::OutputType) {
        let (state, value) = input;
        if state.current < 0.0 {
            (
                ExponentialMovingAverageState {
                    current: value,
                    ..*state
                },
                value,
            )
        } else {
            let new_value = state.k * value + (1.0 - state.k) * state.current;
            (
                ExponentialMovingAverageState {
                    current: new_value,
                    ..*state
                },
                new_value,
            )
        }
    }

    // fn new_state(&self) -> Rc<Self::StateType> {
    //     Rc::new(ExponentialMovingAverageState {
    //         period: self.period(),
    //         k: 2.0 / (self.period() + 1) as f64,
    //         current: -1.0,
    //     })
    // }
}

// impl
//     Next<(
//         Rc<<ExponentialMovingAverage as TA>::StateType>,
//         <ExponentialMovingAverage as TA>::ValueType,
//     )> for ExponentialMovingAverage
// {
//     type Output = (
//         Rc<<ExponentialMovingAverage as TA>::StateType>,
//         <ExponentialMovingAverage as TA>::ValueType,
//     );

//     fn next(&self, input: (Rc<ExponentialMovingAverageState>, f64)) -> Self::Output {
//         let (state, value) = input;
//         if state.current < 0.0 {
//             (
//                 Rc::new(ExponentialMovingAverageState {
//                     current: value,
//                     ..*state
//                 }),
//                 value,
//             )
//         } else {
//             let new_value = state.k * value + (1.0 - state.k) * state.current;
//             (
//                 Rc::new(ExponentialMovingAverageState {
//                     current: new_value,
//                     ..*state
//                 }),
//                 new_value,
//             )
//         }
//     }
// }

// impl<T: CandleDataSource> Next<T> for ExponentialMovingAverage {
//     type Output = f64;

//     fn next(&mut self, input: T) -> Self::Output {
//         self.calc(input.take(self.period).iter().map(|x| x.close))
//     }
// }

impl Reset for ExponentialMovingAverageState {
    fn reset(&mut self) {
        self.k = 2.0 / (self.period + 1) as f64;
        self.current = -1.0;
    }
}

// impl Period for ExponentialMovingAverage {
//     fn period(&self) -> usize {
//         self.period
//     }
// }

// impl<T: Close> Next<&T> for ExponentialMovingAverage {
//     type Output = f64;

//     fn next(&mut self, input: &T) -> Self::Output {
//         self.next(input.close())
//     }
// }

impl Default for ExponentialMovingAverage {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for ExponentialMovingAverageState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "EMAState({})", self.period)
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use super::*;
    use crate::test_helper::*;

    #[test]
    fn test_next() {
        let ema = ExponentialMovingAverage::new();
        let ema_state = Rc::new(ExponentialMovingAverageState::new(3));

        let (ema_state, new_value) = ema.next((&ema_state, 2.0));
        assert_eq!(new_value, 2.0);
        let (ema_state, new_value) = ema.next((&ema_state, 5.0));
        assert_eq!(new_value, 3.5);
        let (ema_state, new_value) = ema.next((&ema_state, 1.0));
        assert_eq!(new_value, 2.25);
        let (_, new_value) = ema.next((&ema_state, 6.25));
        assert_eq!(new_value, 4.25);

        // let mut ema = ExponentialMovingAverage::new(3).unwrap();
        // let bar1 = Bar::new().close(2);
        // let bar2 = Bar::new().close(5);
        // assert_eq!(ema.next(&bar1), 2.0);
        // assert_eq!(ema.next(&bar2), 3.5);
    }

    // #[test]
    // fn test_calc() {
    //     let ema = ExponentialMovingAverage::new(3).unwrap();
    //     assert_eq!(ema.calc(&[6.25, 1.0, 5.0, 2.0]), [4.25, 2.25, 3.5, 2.0]);

    //     let ema4 = ExponentialMovingAverage::new(4).unwrap();
    //     assert_eq!(ema4.calc(&[6.25, 1.0, 5.0, 2.0]), [2.86, 4.1, 3.5]);
    // }
}
