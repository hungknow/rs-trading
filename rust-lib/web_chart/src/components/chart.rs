use leptos_use::use_resize_observer;
use serde_wasm_bindgen::to_value;
use std::sync::{Arc, Mutex};

use crate::{components::ChartControl, OhlcFeedServiceContext};
use charming::{Animation, Echarts, WasmRenderer};
use chrono::DateTime;
use hk_trading::data::{
    datafeed_service::{OhlcFeedService, OhlcFeedServiceImpl},
    Resolution,
};
use hktrading_client::types::SymbolTicker;
use leptos::{
    component, create_action, create_node_ref, create_signal, html::Div, logging, use_context,
    view, IntoView, SignalGet, SignalSet,
};

stylance::import_crate_style!(style, "src/components/chart.module.scss");

use super::ohlc_gen::ohlc_gen;

enum CandleData {
    Candles(Arc<Box<Echarts>>),
    None,
}

impl Clone for CandleData {
    fn clone(&self) -> Self {
        match self {
            CandleData::Candles(echart) => CandleData::Candles(echart.clone()),
            CandleData::None => CandleData::None,
        }
    }
}
// impl Copy for CandleData {}

#[derive(serde::Serialize)]
pub struct ChartResize1 {
    /// New width in px
    pub width: u32,
    /// New height in px
    pub height: u32,
    /// If true, emits events on resize
    pub silent: bool,
    /// Resize animation options
    pub animation: Option<Animation>,
}

async fn create_chart_for_candle(
    ohlcFeedService: Arc<Mutex<OhlcFeedServiceImpl>>,
    width: u32,
    height: u32,
) -> Option<Echarts> {
    logging::log!("height: {}, width: {}", height, width);
    let from_time = DateTime::from_timestamp(1654646400, 0);

    match ohlcFeedService
        // .borrow_mut()
        .try_lock()
    {
        Err(e) => {
            logging::error!("Error: {}", e);
        }
        Ok(mut setter) => {
            let result = setter
                .get_ohlc_by_symbol_resolution_time_range(
                    SymbolTicker::MockXauusd,
                    Resolution::M1,
                    from_time,
                    None,
                )
                .await;
            match result {
                Ok(candles) => {
                    logging::log!("Candles: {} {} {}", candles.open_times.len(), width, height);
                    // console_log(s!("Candles: "), candles.open_times.len());
                    let chart = ohlc_gen(&candles);
                    let renderer = WasmRenderer::new(width, height);
                    let echart = renderer
                        .theme(charming::theme::Theme::Dark)
                        .render("chart", &chart)
                        .unwrap();
                    return Some(echart);
                }
                Err(e) => {
                    logging::error!("Error: {}", e);
                }
            }
        }
    }
    None
}

#[component]
pub fn Chart() -> impl IntoView {
    let el = create_node_ref::<Div>();
    let (echarts_s, set_echarts) = create_signal::<CandleData>(CandleData::None);

    // create_effect(move |_| {
    //     create_chart_for_candle(ohlcFeedService.deref().as_ref())
    // });
    // let once = create_resource(
    //     || (),
    //     |_| async move {
    //         let ohlcFeedService = use_context::<OhlcFeedServiceContext>()
    //             .unwrap()
    //             .ohlc_feed_service
    //             .clone();
    //         create_chart_for_candle(ohlcFeedService.deref().as_ref()).await
    //     },
    // );
    // once.get();
    // create_effect(|_| async move {

    // let UseElementBoundingReturn { width, height, .. } = use_element_bounding(el);
    // let UseElementBoundingReturn { width, height,.. } = use_element_bounding(el);
    // let width_u32 = width.get() as u32;
    // let height_u32 = height.get() as u32;
    // let echart_v = echarts_s.get();

    let action = create_action(move |(width_u32, height_u32): &(u32, u32)| {
        let w = *width_u32;
        let h = *height_u32;
        async move {
            match echarts_s.get() {
                CandleData::Candles(echart) => {
                    let c = ChartResize1 {
                        width: w,
                        height: h,
                        silent: true,
                        animation: None,
                    };
                    echart.resize(to_value(&c).unwrap());
                }
                CandleData::None => {
                    let ohlcFeedService = use_context::<OhlcFeedServiceContext>()
                        .unwrap()
                        .ohlc_feed_service
                        .clone();

                    let r = create_chart_for_candle(ohlcFeedService, w, h).await;

                    match r {
                        Some(echart) => {
                            set_echarts.set(CandleData::Candles(Arc::new(Box::new(echart))));
                        }
                        None => {
                            logging::error!("Error: {}", "No chart found");
                        }
                    }
                }
            }
        }
    });

    use_resize_observer(el, move |entries, observer| {
        let rect = entries[0].content_rect();
        let w = rect.width() as u32;
        let h = rect.height() as u32;

        action.dispatch((w, h));
    });

    view! {
        <section>
            <ChartControl />
            // <button on:click=move |_| {action.dispatch(());}>"Show chart"</button>
            <div id="chart" node_ref=el class=style::chart></div>
        </section>
    }
}
