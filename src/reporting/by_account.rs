use budgetronlib::fintime::Timeframe;
use loading::Transaction;
use reporting::Reporter;
use std::fmt;

pub struct ByAccount<'a, T>
    where T: 'a + Reporter
{
    inner: &'a T,
    account: String,
}

impl<'a, T> ByAccount<'a, T>
    where T: 'a + Reporter
{
    pub fn new(inner: &'a T, account: String) -> Self {
        ByAccount { inner, account }
    }
}

#[derive(Debug, Serialize)]
pub struct ByAccountReport<T> {
    account: String,
    by_account: T,
}

impl<T> ByAccountReport<T>
    where T: fmt::Display
{
    pub fn print(&self) {
        println!("{}", self)
    }
}

impl<T> fmt::Display for ByAccountReport<T>
    where T: fmt::Display
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "For the account {}", self.account)?;
        writeln!(f, "{}", self.by_account)
    }
}

impl<'a, T> Reporter for ByAccount<'a, T>
    where T: Reporter
{
    type OutputType = ByAccountReport<T::OutputType>;

    fn report<'b, I>(&self, transactions: I) -> ByAccountReport<T::OutputType>
        where I: Iterator<Item = &'b Transaction>
    {
        let (transactions, _): (Vec<_>, Vec<_>) = transactions
            .into_iter()
            .partition(|t| {
                           t.account_name == self.account ||
                           t.transfer_destination_account
                               .as_ref()
                               .map(|s| *s == self.account)
                               .unwrap_or(false)
                       });
        ByAccountReport {
            by_account: self.inner.report(transactions.into_iter()),
            account: self.account.clone(),
        }
    }
}
