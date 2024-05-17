use cf3::is_enabled;

fn main() {
    let env = std::env::var("APP_ENV").unwrap_or("dev".to_string());

    let f_name = "testFeature2";
    let feature = is_enabled(f_name, &env);
    println!("Feature {} found: {}", f_name, feature);
}
