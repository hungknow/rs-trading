use std::{cell::RefCell, sync::{Arc, Mutex, RwLock}};

use hk_trading::data::datafeed_service::{OhlcFeedService, OhlcFeedServiceImpl};

#[derive(Clone)]
pub struct OhlcFeedServiceContext {
    pub ohlc_feed_service: Arc<Mutex<OhlcFeedServiceImpl>>,
    //     // pub Arc<Box<dyn OhlcFeedService>>
}

impl OhlcFeedServiceContext {
    pub fn new(ohlc_feed_service: Arc<Mutex<OhlcFeedServiceImpl>>) -> Self {
        Self {
            //             // ohlc_feed_service
            ohlc_feed_service: ohlc_feed_service.clone(),
        }
    }
}

// impl<T: OhlcFeedService> Clone for OhlcFeedServiceContext<T> {
//     fn clone(&self) -> Self {
//         Self {
//             ohlc_feed_service: self.ohlc_feed_service.clone(),
//         }
//     }
// }
