use leptos::{component, create_signal, view, IntoView};

use crate::components::ChartControl;

#[component]
pub fn Chart() -> impl IntoView {
    // let (candles_signal, set_candles) = create_signal(Vec<int>::new());

    // let candles = create_resource(
    //     move || candles_signal.get(),
    //     move || async move { load_candles("mock:xauusd".to_owned(), "m1".to_owned(), 0, 12).await },
    // );
    view! {
        <section>
            <ChartControl />
            // Display the c
        </section>
    }
}
