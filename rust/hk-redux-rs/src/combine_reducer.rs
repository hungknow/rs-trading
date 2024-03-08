use std::collections::HashMap;

use redux_rs::Reducer;

// fn combine_reducer<State, Action>(
//     reducers: HashMap<String, Box<dyn Reducer<State, Action>>>,
// ) -> Box<dyn Reducer<State, Action>> {
//     Box::new(move |state: &State, action: &Action| {
//         let mut new_state = state.clone();
//         for reducer in reducers.values() {
//             new_state = reducer.reduce(&new_state, action);
//         }
//         new_state
//     })
// }
