fn ChartControl() -> View {
    view! {
        <div>
            // Combo box to select time range

            <label for="birthdaytime">From:</label>
            <input type="datetime-local" id="birthdaytime" name="birthdaytime">

            <label for="">To:</label>
            <span>2022-01-12 13:30</span>

            // Resolution
            <span>M1</span>
            <span>M5</span>
            <span>M15</span>
            <span>M30</span>
            <span>H1</span>
            <span>H4</span>
        </div>
    }
}
