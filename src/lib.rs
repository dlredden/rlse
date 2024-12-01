pub mod config;
pub mod feature;
mod neon_wrapper;

use neon::prelude::*;
use neon_wrapper::is_enabled;

// Neon module initialization
#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("is_enabled", is_enabled)?;
    Ok(())
}
