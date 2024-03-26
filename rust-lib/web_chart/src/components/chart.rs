use std::{borrow::BorrowMut, cell::RefCell, ops::Deref, sync::{Arc, RwLock}};

use crate::{
    components::{ohlc::ohlc_chart, shanhai_index::shanhai_chart, ChartControl},
    OhlcFeedServiceContext,
};
use charming::WasmRenderer;
use chrono::DateTime;
use hk_trading::data::{
    datafeed_service::{OhlcFeedService, OhlcFeedServiceImpl},
    Resolution,
};
use hktrading_client::types::SymbolTicker;
use leptos::{component, create_action, logging, use_context, view, IntoView};

use super::ohlc_gen::ohlc_gen;

async fn create_chart_for_candle(ohlcFeedService: Arc<RwLock<OhlcFeedServiceImpl>>) {
    let from_time = DateTime::from_timestamp(1654646400, 0);
    let result = ohlcFeedService        
        // .borrow_mut()
        .write()
        .unwrap()
        .get_ohlc_by_symbol_resolution_time_range(
            SymbolTicker::MockXauusd,
            Resolution::M1,
            from_time,
            None,
        )
        .await;
    match result {
        Ok(candles) => {
            logging::log!("Candles: {}", candles.open_times.len());
            // console_log(s!("Candles: "), candles.open_times.len());
            let chart = ohlc_gen(&candles);
            let renderer = WasmRenderer::new(1200, 700);
            renderer.render("chart", &chart).unwrap();
        }
        Err(e) => {
            logging::error!("Error: {}", e);
        }
    }
}
#[component]
pub fn Chart() -> impl IntoView {
    // let action = create_action(|_input: &()| async {
    //    let chart = shanhai_chart();

    //     let renderer = WasmRenderer::new(1200, 700);
    //     renderer.render("chart",&chart).unwrap();
    // });
    // let ohlcFeedService = use_context::<OhlcFeedServiceContext>()
    //     .unwrap()
    //     .ohlc_feed_service
    //     .clone();
    // create_effect(move |_| {
    //     let chart = ohlc_chart();

    //     let renderer = WasmRenderer::new(1200, 700);
    //     renderer.render("chart", &chart).unwrap();
    // });

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
    let action = create_action(|_input: &()| async {
        let ohlcFeedService = use_context::<OhlcFeedServiceContext>()
            .unwrap()
            .ohlc_feed_service
            .clone();
        
        create_chart_for_candle(ohlcFeedService).await
    });
    action.dispatch(());

    view! {
        <section>
            <ChartControl />
            // <button on:click=move |_| {action.dispatch(());}>"Show chart"</button>
            <div id="chart"></div>
        </section>
    }
}
