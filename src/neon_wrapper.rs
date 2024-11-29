use neon::prelude::*;
use crate::config::get_config;
use crate::feature::is_feature_enabled;

pub fn is_enabled(mut cx: FunctionContext) -> JsResult<JsBoolean> {
    // Convert Neon `JsString` arguments into Rust `String`
    let feature_name: String = cx.argument::<JsString>(0)?.value(&mut cx);
    let env: String = cx.argument::<JsString>(1)?.value(&mut cx);

    // Get the configuration
    let config = get_config();

    // Call the core logic function with `&str` references
    let result = is_feature_enabled(&feature_name, &env, config);

    // Return the result as a `JsBoolean`
    Ok(cx.boolean(result))
}
