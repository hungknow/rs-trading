use leptos::{component, view};

#[component]
fn ChartContainer() -> impl View {
    view! {
        <div>
            <h1>"Chart"</h1>
            <Chart />
        </div>
    }
}

fn main() {
    leptos::mount_to_body(|| {
        view! { <ChartContainer /> }
    })
}