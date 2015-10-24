use phf::{Map, Set};

pub static CATEGORIES: Map<&'static str, Set<&'static str>> = phf_map! {
    "Auto Payment" => phf_set![],
    "Automobile" => phf_set![
        "Auto & Transport", "Service & Parts", "Gas & Fuel", "Parking"],
    "Bills" => phf_set![
        "Bills & Utilities", "BillsCable", "BillsElectric", "BillsPhone", "BillsRent", "Internet",
        "Mortgage & Rent", "Utilities", "Security", "Mobile Phone"],
    "Business" => phf_set!["Advertising","Business Services", "Office Supplies"],
    "Cash" => phf_set!["ATM", "Cash & ATM"],
    "Charity" => phf_set![],
    "Education" => phf_set!["Tuition", "Student Loan"],
    "Entertainment" => phf_set!["Movies & DVDs", "Music", "Arts", "Amusement"],
    "Fees" => phf_set!["Fees & Charges", "Finance Charge", "Bank Fee", "Shipping", "ATM Fee"],
    "Financial" => phf_set![],
    "Food & Dining" => phf_set![
        "Alcohol & Bars", "Fast Food", "Food Trucks", "DiningOut", "Delivery/Takeout",
        "Coffee Shops", "Restaurants", "Seamless"],
    "Gifts" => phf_set!["Gift"],
    "Groceries" => phf_set![],
    "Health" => phf_set!["Pharmacy", "HealthDental", "Health & Fitness", "Dentist", "Doctor", "Gym"],
    "Hobbies" => phf_set!["Books", "Books & Magazines", "Conventions"],
    "Household" => phf_set![
        "Furnishings", "Home Improvement", "Home Services", "Lawn & Garden", "Pets",
        "Home Supplies"],
    "Income" => phf_set!["Wages", "Paycheck", "Rental Income"],
    "Insurance" => phf_set!["Auto Insurance", "BillsInsurance"],
    "Interest" => phf_set!["Interest Income", "Dividends"],
    "Investments" => phf_set![],
    "Kids" => phf_set!["Toys"],
    "Legal" => phf_set![],
    "Personal Care" => phf_set!["PersonalCare", "Hair", "Spa & Massage"],
    "Crowdfunding" => phf_set!["Kickstarter"],
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
    "Hide" => phf_set!["Hide from Budgets & Trends", "Pending"]
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
