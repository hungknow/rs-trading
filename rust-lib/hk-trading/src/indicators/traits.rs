use crate::Reset;

pub enum IndicatorType {
    RSI,
}

// Generate the indicator output data type from the input data
pub trait Indicator {
    type InputType: Clone;
    type OutputType: Clone;
    type StateType: Clone + Reset + Default;

    // fn new_state(&self) -> Rc<Self::StateType>;
    fn next(
        &self,
        input: (&Self::StateType, Self::InputType),
    ) -> (Self::StateType, Self::OutputType);

    // fn get_event_receiver() -> mpsc::Receiver<Self::StateType>;
}

// Contains the value of TA
// and their time range
// pub struct IndicatorContainer<T: Indicator> {
//     pub indicator: T,
//     pub symbol_identity: SymbolIdentity,
//     pub state: T::StateType,
//     pub timestamp: Vec<DateTime<Utc>>,
//     pub values: Vec<T::OutputType>,
// }

// impl<T: Indicator> IndicatorContainer<T> {
//     pub fn new(indicator: T, symbol_identity: SymbolIdentity) -> Self {
//         Self {
//             indicator,
//             symbol_identity,
//             state: T::StateType::default(),
//             timestamp: Vec::new(),
//             values: Vec::new(),
//         }
//     }
// }

// pub struct IndicatorDataGetReq<'a> {
//     pub symbol_identity: &'a SymbolIdentity,
//     pub start_time: Option<DateTime<Utc>>,
//     pub end_time: Option<DateTime<Utc>>,
//     pub indicator_type: IndicatorType,
// }

// pub struct IndicatorDataGetRes {
//     pub symbol_identity: SymbolIdentity,
//     pub start_time: Option<DateTime<Utc>>,
//     pub end_time: Option<DateTime<Utc>>,
//     pub indicator_type: IndicatorType,
//     pub timestamp: Vec<DateTime<Utc>>,
// }

// pub trait IndicatorManager {
//     fn get_data<'a>(&self, option: IndicatorDataGetReq<'a>) -> HkFutureResult<IndicatorDataGetRes, HkError>;
// }

// pub struct OhlcPattern {
//     aprice: f64,
//     "atime": 1567458000,
//     "bprice": 1.1109,
//     "btime": 1568322000,
//     "cprice": 1.09897,
//     "ctime": 1568667600,
//     "dprice": 0,
//     "dtime": 0,
//     "end_price": 1.1109,
//     "end_time": 1568926800,
//     "entry": 1.1109,
//     "eprice": 0,
//     "etime": 0,
//     "mature": 0,
//     patternname: String,// "Double Bottom",
//     patterntype: String, // "bullish",
//     "profit1": 1.1294,
//     "profit2": 0,
//     "sortTime": 1568926800,
//     "start_price": 1.1109,
//     "start_time": 1566853200,
//     "status": "incomplete",
//     "stoploss": 1.0905,
//     "symbol": "EUR_USD",
//     "terminal": 0
//   },
