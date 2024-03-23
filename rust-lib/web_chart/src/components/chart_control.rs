use leptos::{component, view, IntoView};
stylance::import_crate_style!(style, "src/components/chart_control.module.scss");

#[component]
pub fn ChartControl() -> impl IntoView {
    view! {
        <div class=style::chart_controls>
            // Combo box to select time range

            <label for="fromtimestamp">From:</label>
            <input type="datetime-local" id="fromtimestamp" name="fromtimestamp"/>

            <label for="totimestamp">To:</label>
            <input type="datetime-local" id="totimestamp" name="totimestamp"/>

            // Resolution
            <div class=style::resolution_buttons>
                <span>M1</span>
                <span>M5</span>
                <span>M15</span>
                <span>M30</span>
                <span>H1</span>
                <span>H4</span>
            </div>
        </div>
    }
}
