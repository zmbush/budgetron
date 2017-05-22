use loading::Transaction;

pub trait Reporter {
    type OutputType;

    fn report(&self, transactions: &Vec<Transaction>) -> Self::OutputType;
}

mod net_worth;

pub use reporting::net_worth::NetWorth;
