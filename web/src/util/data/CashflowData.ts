// Copyright 2019 Zachary Bush.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

import { CashflowTsDatum, Timeseries } from "util/data";

export class CashflowData {
  public credit: string;
  public debit: string;
  public net: string;
  public timeseries?: Timeseries<CashflowTsDatum>;

  constructor(data: { [part: string]: any }) {
    if (typeof data.credit === "string") { this.credit = data.credit; }
    if (typeof data.debit === "string") { this.debit = data.debit; }
    if (typeof data.net === "string") { this.net = data.net; }
    if (data.timeseries && Array.isArray(data.timeseries)) {
      this.timeseries = new Timeseries(data.timeseries, CashflowTsDatum.parse);
    }
  }
}
