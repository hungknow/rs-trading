use crate::components::{ohlc::ohlc_chart, shanhai_index::shanhai_chart, ChartControl};
use charming::WasmRenderer;
use leptos::{component, create_action, create_effect, view, IntoView};

#[component]
pub fn Chart() -> impl IntoView {
    // let action = create_action(|_input: &()| async {
    //    let chart = shanhai_chart();

    //     let renderer = WasmRenderer::new(1200, 700);
    //     renderer.render("chart",&chart).unwrap();
    // });
    create_effect(move |_| {
        let chart = ohlc_chart();

        let renderer = WasmRenderer::new(1200, 700);
        renderer.render("chart", &chart).unwrap();
    });

    view! {
        <section>
            <ChartControl />
            // <button on:click=move |_| {action.dispatch(());}>"Show chart"</button>
            <div id="chart"></div>
        </section>
    }
}
