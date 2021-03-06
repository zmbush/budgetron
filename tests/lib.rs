// Copyright 2017 Zachary Bush.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use {
    budgetron::{processing::ConfiguredProcessors, reporting::ConfiguredReports},
    budgetronlib::config,
};

#[test]
fn test_loading_budgetronrc_example() {
    let _: ConfiguredProcessors =
        config::load_cfg("budgetronrc.example.toml").expect("Failed to load configured processors");
    let _: ConfiguredReports =
        config::load_cfg("budgetronrc.example.toml").expect("Configured Reports failed to load");
}
