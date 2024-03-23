use leptos::*;
use leptos_meta::*;
use web_chart::components::Chart;

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
