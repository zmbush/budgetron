extern crate budgetron;
extern crate budgetronlib;

use budgetronlib::config;
use budgetron::processing::ConfiguredProcessors;
use budgetron::reporting::ConfiguredReports;

#[test]
fn test_loading_budgetronrc_example() {
    let _: ConfiguredProcessors =
        config::load_cfg("budgetronrc.example.toml").expect("Failed to load configured processors");
    let _: ConfiguredReports =
        config::load_cfg("budgetronrc.example.toml").expect("Configured Reports failed to load");
}
