// Copyright 2019 Zachary Bush.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

export class CategoriesCategory {
  public amount: string;
  public transactions: string[];

  constructor(data: { [part: string]: any }) {
    if (data.amount && typeof data.amount === "string") {
      this.amount = data.amount;
    }
    this.transactions = [];
    if (data.transactions && Array.isArray(data.transactions)) {
      data.transactions.forEach((t) => {
        if (typeof t === "string") {
          this.transactions.push(t);
        }
      });
    }
  }
}
