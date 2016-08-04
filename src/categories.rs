use phf::Map;

pub static LIMITS: Map<&'static str, f64> = phf_map! {
    "Automobile" => 250.0,
    "Bills & Utilities" => 524.29 - 92.50,
    "Mortgage & Rent" => 3543.13 + 92.50,
    "Charity" => 34.0,
    "Entertainment" => 50.0,
    "Food & Dining" => 300.0,
    "Groceries" => 600.0,
    "Health" => 200.0,
    "Hobbies" => 100.0,
    "Household" => 50.0,
    "Insurance" => 321.0,
    "Online Subscriptions" => 50.0,
    "Shopping" => 115.0
};
