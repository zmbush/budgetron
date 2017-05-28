use loading::{Transaction, TransactionType};
use reporting::Reporter;
use std::borrow::Cow;
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
        where I: Iterator<Item = Cow<'b, Transaction>>
    {
        let (transactions, _): (Vec<_>, Vec<_>) =
            transactions
                .into_iter()
                .map(|t| if let TransactionType::Transfer = t.transaction_type {
                         if t.account_name == self.account {
                             let mut t = t.into_owned();
                             t.transaction_type = TransactionType::Debit;
                             t.transfer_destination_account = None;
                             Cow::Owned(t)
                         } else if *t.transfer_destination_account
                                        .as_ref()
                                        .expect("all transfers should have destinations") ==
                                   self.account {
                             let mut t = t.into_owned();
                             t.transaction_type = TransactionType::Credit;
                             t.account_name = t.transfer_destination_account.take().unwrap();
                             Cow::Owned(t)
                         } else {
                             t
                         }
                     } else {
                         t
                     })
                .partition(|t| t.account_name == self.account);

        ByAccountReport {
            by_account: self.inner.report(transactions.into_iter()),
            account: self.account.clone(),
        }
    }
}
