use phf::{Map, Set};

pub static CATEGORIES: Map<&'static str, Set<&'static str>> = phf_map! {
    "Auto Payment" => phf_set![],
    "Automobile" => phf_set![
        "Auto & Transport", "Service & Parts", "Gas & Fuel", "Parking"],
    "Business" => phf_set!["Advertising","Business Services", "Office Supplies"],
    "Cash" => phf_set!["ATM", "Cash & ATM"],
    "Charity" => phf_set![],
    "Crowdfunding" => phf_set!["Kickstarter"],
    "Education" => phf_set!["Tuition", "Student Loan"],
    "Entertainment" => phf_set!["Movies & DVDs", "Music", "Arts", "Amusement"],
    "Fees" => phf_set!["Fees & Charges", "Finance Charge", "Bank Fee", "Shipping", "ATM Fee"],
    "Financial" => phf_set![],
    "Food & Dining" => phf_set![
        "Alcohol & Bars", "Fast Food", "Food Trucks", "DiningOut", "Delivery/Takeout",
        "Coffee Shops", "Restaurants", "Seamless"],
    "Gifts" => phf_set!["Gift"],
    "Groceries" => phf_set![],
    "Health" => phf_set!["Pharmacy", "HealthDental", "Health & Fitness", "Dentist", "Doctor",
        "Gym"],
    "Hobbies" => phf_set!["Books", "Books & Magazines", "Conventions"],
    "Household" => phf_set![
        "Furnishings", "Home Improvement", "Home Services", "Lawn & Garden", "Pets",
        "Home Supplies", "Home"],
    "Income" => phf_set!["Wages", "Paycheck", "Rental Income"],
    "Insurance" => phf_set!["Auto Insurance", "BillsInsurance"],
    "Interest" => phf_set!["Interest Income", "Dividends"],
    "Investments" => phf_set!["Trade Commissions"],
    "Kids" => phf_set!["Toys"],
    "Legal" => phf_set![],
    "Online Subscriptions" => phf_set![],
    "Personal Care" => phf_set!["PersonalCare", "Hair", "Spa & Massage"],
    "Rent" => phf_set!["BillsRent", "Mortgage & Rent"],
    "Shopping" => phf_set![
        "Amazon", "Clothing", "Sporting Goods", "Sports", "Costco", "Warehouse Clubs"],
    "Taxes" => phf_set!["State Tax", "Federal Tax"],
    "Technology" => phf_set!["Computer/Video Games", "Electronics & Software", "VPS Hosting"],
    "Television" => phf_set![],
    "Transfers" => phf_set!["Transfer", "Credit Card Payment"],
    "Travel" => phf_set!["Hotel", "Public Transportation", "Rental Car & Taxi"],
    "Uncategorized" => phf_set![
        "Transfer", "Check", "CreditMiscellaneous", "CreditNone", "CreditTransfer",
        "DebitMiscellaneous", "DebitNone", "DebitTransfer", "Reimbursement"],
    "Bills & Utilities" => phf_set!["Bills", "BillsCable", "BillsElectric", "BillsPhone",
        "Internet", "Utilities", "Security", "Mobile Phone"],
    "Hide" => phf_set!["Hide from Budgets & Trends", "Pending", "Exclusions"]
};

pub static LIMITS: Map<&'static str, f64> = phf_map! {
    "Automobile" => 250.0,
    "Bills & Utilities" => 524.29 - 92.50,
    "Rent" => 2175.71 + 92.50,
    "Charity" => 34.0,
    "Entertainment" => 50.0,
    "Food & Dining" => 300.0,
    "Groceries" => 400.0,
    "Health" => 200.0,
    "Hobbies" => 100.0,
    "Household" => 50.0,
    "Insurance" => 260.0,
    "Online Subscriptions" => 50.0,
    "Shopping" => 175.0
};

pub fn find_category(cat: &str) -> Option<&'static str> {
    for (&key, values) in CATEGORIES.entries() {
        if key == cat || (values.len() > 0 && values.contains(cat)) {
            return Some(key);
        }
    }
    println!("Unable to categorize transaction. {}", cat);
    None
}
