use std::collections::HashMap;

use redux_rs::Selector;

// pub struct SliceSelectors<State> {
//     // Define your selectors here
// }

pub struct SliceOption<State> {
    name: String,
    initialState: State,
    // reducers: SliceReducers<State>,
    // selectors: HashMap<String, Box<dyn Selector<State, Result = _>>>,
}
//
//
// pub fn create_slice() {

// }
