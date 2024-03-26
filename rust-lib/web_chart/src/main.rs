use std::sync::{Arc, RwLock};

use hk_trading::data::datafeed_service::OhlcFeedServiceImpl;
use leptos::*;
use leptos_meta::*;
use web_chart::{components::Chart, OhlcFeedServiceContext};

#[component]
fn ChartContainer() -> impl IntoView {
    view! {
        <div>
            <h1>"Chart"</h1>
            <Chart />
        </div>
    }
}

#[component]
fn App() -> impl IntoView {
    provide_meta_context();
    let fallback = || view! { "Page not found." }.into_view();

    let mut client = hktrading_client::Client::new("http://localhost:9001");
    let mut ohldFeedService = OhlcFeedServiceImpl::new(client);
    let ohlcFeedServiceCtx = OhlcFeedServiceContext::new(Arc::new(RwLock::new(ohldFeedService)));

    provide_context(ohlcFeedServiceCtx);
    view! {
        // <Stylesheet id="leptos" href="/styles/bundle.css"/>
        <ChartContainer />
    }
}

fn main() {
    leptos::mount_to_body(|| {
        view! {
            <App />
        }
    })
}
