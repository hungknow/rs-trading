use hk_infra::{future::HkFutureResult, HkError};

use super::TradingChartData;

pub struct ChartSVGDrawingContext {
    pub width: u32,
    pub height: u32,
    // pub data: &'a TradingChartData,
}

pub trait ChartSVGDrawing {
    fn draw_chart(&self, c: &ChartSVGDrawingContext) -> HkFutureResult<Vec<u8>, HkError>;
}

pub struct TradingChartDrawing {
     
}

impl ChartSVGDrawing for TradingChartDrawing {
    fn draw_chart(&self, c: &ChartSVGDrawingContext) -> HkFutureResult<Vec<u8>, HkError> {
        unimplemented!()
        // 
    }
}
