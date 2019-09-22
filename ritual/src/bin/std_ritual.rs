use ritual::cli;
use ritual::config::{Config, CrateProperties, GlobalConfig};
use ritual_common::cpp_build_config::{CppBuildConfigData, CppLibraryType};
use ritual_common::errors::{FancyUnwrap, Result};
use ritual_common::{target, toml};

pub const CPP_STD_VERSION: &str = "0.0.0";

fn create_config() -> Result<Config> {
    let mut crate_properties = CrateProperties::new("cpp_std", CPP_STD_VERSION);
    let mut custom_fields = toml::value::Table::new();
    let mut package_data = toml::value::Table::new();
    package_data.insert(
        "authors".to_string(),
        toml::Value::Array(vec![toml::Value::String(
            "Pavel Strakhov <ri@idzaaus.org>".to_string(),
        )]),
    );
    package_data.insert(
        "description".to_string(),
        toml::Value::String("Bindings for C++ standard library".into()),
    );
    // TODO: doc url
    //package_data.insert("documentation".to_string(), toml::Value::String(doc_url));
    package_data.insert(
        "repository".to_string(),
        toml::Value::String("https://github.com/rust-qt/ritual".to_string()),
    );
    package_data.insert(
        "license".to_string(),
        toml::Value::String("MIT OR Apache-2.0".to_string()),
    );
    package_data.insert(
        "keywords".to_string(),
        toml::Value::Array(vec![
            toml::Value::String("ffi".to_string()),
            toml::Value::String("ritual".to_string()),
        ]),
    );
    package_data.insert(
        "categories".to_string(),
        toml::Value::Array(vec![toml::Value::String(
            "external-ffi-bindings".to_string(),
        )]),
    );

    custom_fields.insert("package".to_string(), toml::Value::Table(package_data));
    crate_properties.set_custom_fields(custom_fields);

    let mut config = Config::new(crate_properties);
    config.set_cpp_lib_version("11");

    //config.add_include_directive("...");

    config.add_cpp_parser_argument("-std=c++11");
    let mut data = CppBuildConfigData::new();
    data.set_library_type(CppLibraryType::Static);
    config
        .cpp_build_config_mut()
        .add(target::Condition::True, data);
    Ok(config)
}

fn main() {
    let mut config = GlobalConfig::new();
    config.set_all_crate_names(vec!["cpp_std".into()]);
    config.set_create_config_hook(|_crate_name| create_config());

    cli::run_from_args(config).fancy_unwrap();
}
