use neon::prelude::*;
use crate::config::get_config;
use crate::feature::is_feature_enabled;

pub fn is_enabled(mut cx: FunctionContext) -> JsResult<JsBoolean> {
    // Convert Neon `JsString` arguments into Rust `String`
    let feature_name: String = cx.argument::<JsString>(0)?.value(&mut cx);
    
    // Handle the optional `env` argument
    let env_arg: Option<Handle<'_, JsValue>> = cx.argument_opt(1); // Get the optional argument
    let env: String = if let Some(env_value) = env_arg {
        // If an argument is provided, try converting it to `String`
        env_value.downcast_or_throw::<JsString, _>(&mut cx)?.value(&mut cx)
    } else {
        // If no argument is provided, check the `APP_ENV` variable
        std::env::var("APP_ENV").unwrap_or_else(|_| "dev".to_string())
    };

    // Get the configuration
    let config = get_config();

    // Call the core logic function with `&str` references
    let result = is_feature_enabled(&feature_name, &env, config);

    // Return the result as a `JsBoolean`
    Ok(cx.boolean(result))
}
