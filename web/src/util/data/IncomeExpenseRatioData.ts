// Copyright 2019 Zachary Bush.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

import { IncomeExpenseRatioDatum } from "util/data";

export class IncomeExpenseRatioData {
  public credit: IncomeExpenseRatioDatum;
  public debit: IncomeExpenseRatioDatum;

  constructor(data: { [part: string]: any }) {
    if (typeof data.credit === "object") {
      this.credit = new IncomeExpenseRatioDatum(data.credit);
    }
    if (typeof data.debit === "object") {
      this.debit = new IncomeExpenseRatioDatum(data.debit);
    }
  }
}
