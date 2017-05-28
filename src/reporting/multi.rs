use data_store;
use loading::{Person, TransactionType};
use loading::Transaction;
use reporting::Reporter;
use std::borrow::Cow;

macro_rules! tuple_impls {
    ($(
        $Tuple:ident {
            $(($idx:tt) -> $T:ident)+
        }
    )+) => {
        $(
            impl<$($T:Reporter),+> Reporter for ($($T),+) {
                type OutputType = ($($T::OutputType),+);

                fn report<'a, It>(&self, transactions: It) -> ($($T::OutputType),+)
                    where It: Iterator<Item = Cow<'a, Transaction>> + Clone {
                        ($(self.$idx.report(transactions.clone())),+)
                }
            }
        )+
    }
}

tuple_impls! {
    Tuple2 {
        (0) -> A
        (1) -> B
    }
    Tuple3 {
       (0) -> A
       (1) -> B
       (2) -> C
   }
   Tuple4 {
       (0) -> A
       (1) -> B
       (2) -> C
       (3) -> D
   }
   Tuple5 {
       (0) -> A
       (1) -> B
       (2) -> C
       (3) -> D
       (4) -> E
   }
   Tuple6 {
       (0) -> A
       (1) -> B
       (2) -> C
       (3) -> D
       (4) -> E
       (5) -> F
   }
   Tuple7 {
       (0) -> A
       (1) -> B
       (2) -> C
       (3) -> D
       (4) -> E
       (5) -> F
       (6) -> G
   }
   Tuple8 {
       (0) -> A
       (1) -> B
       (2) -> C
       (3) -> D
       (4) -> E
       (5) -> F
       (6) -> G
       (7) -> H
   }
   Tuple9 {
       (0) -> A
       (1) -> B
       (2) -> C
       (3) -> D
       (4) -> E
       (5) -> F
       (6) -> G
       (7) -> H
       (8) -> I
   }
   Tuple10 {
       (0) -> A
       (1) -> B
       (2) -> C
       (3) -> D
       (4) -> E
       (5) -> F
       (6) -> G
       (7) -> H
       (8) -> I
       (9) -> J
   }
   Tuple11 {
       (0) -> A
       (1) -> B
       (2) -> C
       (3) -> D
       (4) -> E
       (5) -> F
       (6) -> G
       (7) -> H
       (8) -> I
       (9) -> J
       (10) -> K
   }
   Tuple12 {
       (0) -> A
       (1) -> B
       (2) -> C
       (3) -> D
       (4) -> E
       (5) -> F
       (6) -> G
       (7) -> H
       (8) -> I
       (9) -> J
       (10) -> K
       (11) -> L
   }
}
