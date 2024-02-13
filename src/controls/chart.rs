pub struct ChartArea {
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32,
}

pub struct ChartState {
    DisplayRange: (chrono::DateTime, chrono::DateTime),
    MaxRange: (chrono::DateTime, chrono::DateTime),
}

// One chart can have many sub chart
// Each subchart has its own data source
// Chart emit some events
trait Chart {
    // Get
    fn GetState(&self) -> ChartState;
}

fn UpdateChart() {
    // Fetch new data
    // After receiving the data, stream it to the pipeline
    // Pipeline process the data, then trigger to draw on chart
}

fn UpdateChartOnScale() {
    // Because dvslr onlyd
}
