// Copyright 2019 Zachary Bush.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

import { Timeseries } from "util/data";

export class RollingBudgetData {
  public static parseTsDatum(datum: {}): {} {
    const retval: { [date: string]: number } = {};
    Object.entries(datum).forEach(([k, v]) => {
      if (typeof k === "string" && typeof v === "string") {
        retval[k] = parseFloat(v);
      }
    });
    return retval;
  }
  public budgets: Map<string, string>;
  public transactions: string[];
  public timeseries?: Timeseries<{}>;

  constructor(data: { [part: string]: any }) {
    this.budgets = new Map();
    if (typeof data.budgets === "object") {
      Object.entries(data.budgets).forEach(([k, v]) => {
        if (typeof k === "string" && typeof v === "string") {
          this.budgets.set(k, v);
        }
      });
    }
    this.transactions = [];
    if (Array.isArray(data.transactions)) {
      data.transactions.forEach((t) => {
        if (typeof t === "string") {
          this.transactions.push(t);
        }
      });
    }
    if (data.timeseries && Array.isArray(data.timeseries)) {
      this.timeseries = new Timeseries(
        data.timeseries,
        RollingBudgetData.parseTsDatum,
      );
    }
  }
}
