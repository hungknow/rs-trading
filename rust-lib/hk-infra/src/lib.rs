mod errors;
pub use errors::*;

mod platform_utils;
pub use platform_utils::*;

if_native! {
  mod native;
//   pub mod file_util;
  pub mod future {
   pub use crate::native::future::*;
  }
}

if_wasm! {
  mod wasm;
  pub mod future {
  pub use crate::wasm::future::*;
  }
}
