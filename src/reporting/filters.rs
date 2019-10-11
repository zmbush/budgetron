// Copyright 2019 Zachary Bush.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use {
    crate::loading::{Transaction, TransactionType},
    std::{borrow::Cow, iter::Filter},
};

type Filtered<T> = Filter<T, Box<dyn FnMut(&<T as Iterator>::Item) -> bool>>;

pub trait IterExt<'t>: Iterator<Item = Cow<'t, Transaction>> + Sized {
    fn only_type(self, ty: TransactionType) -> Filtered<Self>;
    fn excluding_type(self, ty: TransactionType) -> Filtered<Self>;

    fn only_tags(self, tags: Vec<String>) -> Filtered<Self>;
    fn excluding_tags(self, tags: Vec<String>) -> Filtered<Self>;

    fn only_owners(self, owners: Vec<String>) -> Filtered<Self>;
    fn excluding_owners(self, owners: Vec<String>) -> Filtered<Self>;
}

impl<'t, I> IterExt<'t> for I
where
    I: Iterator<Item = Cow<'t, Transaction>> + Sized,
{
    fn only_type(self, ty: TransactionType) -> Filtered<Self> {
        self.filter(Box::new(move |t| t.transaction_type == ty))
    }
    fn excluding_type(self, ty: TransactionType) -> Filtered<Self> {
        self.filter(Box::new(move |t| t.transaction_type != ty))
    }

    fn only_tags(self, tags: Vec<String>) -> Filtered<Self> {
        self.filter(Box::new(move |t| {
            tags.iter().any(|tag| t.tags.contains(tag))
        }))
    }
    fn excluding_tags(self, tags: Vec<String>) -> Filtered<Self> {
        self.filter(Box::new(move |t| {
            !tags.iter().any(|tag| t.tags.contains(tag))
        }))
    }

    fn only_owners(self, owners: Vec<String>) -> Filtered<Self> {
        self.filter(Box::new(move |t| {
            owners.iter().any(|owner| t.person == *owner)
        }))
    }
    fn excluding_owners(self, owners: Vec<String>) -> Filtered<Self> {
        self.filter(Box::new(move |t| {
            !owners.iter().any(|owner| t.person == *owner)
        }))
    }
}
