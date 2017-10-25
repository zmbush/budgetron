// Copyright 2017 Zachary Bush.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use loading::Transaction;
use reporting::Reporter;
use serde_json::Value;
use serde_json::map::Map;
use std::borrow::Cow;

macro_rules! tuple_impls {
    ($(
        $Tuple:ident {
            $(($idx:tt) -> $T:ident)+
        }
    )+) => {
        $(
            impl<$($T:Reporter),+> Reporter for ($($T),+) {
                fn report<'a, It>(&self, transactions: It) -> Value
                    where It: Iterator<Item = Cow<'a, Transaction>> + Clone {
                        let mut retval = Map::new();
                        $(
                            if let Some(v) = self.$idx.key() {
                                retval.insert(v.to_owned(), self.$idx.report(transactions.clone()));
                            } else {
                                match self.$idx.report(transactions.clone()) {
                                    Value::Object(o) => for (k, v) in o {
                                        retval.insert(k, v);
                                    },
                                    Value::Null => {},
                                    other => {
                                        retval.insert("UNNAMED".to_owned(), other);
                                    }
                                }
                            }
                        )+
                        Value::Object(retval)
                }

                fn key(&self) -> Option<String> {
                    None
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
