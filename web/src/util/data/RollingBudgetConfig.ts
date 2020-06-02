// Copyright 2019 Zachary Bush.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

export class RollingBudgetConfig {
  public type: "RollingBudget" = "RollingBudget";
  public split: string;
  public startDate: Date;
  public amounts: Map<Date, Map<string, string>>;

  constructor({
    split,
    start_date: startDate,
    amounts,
  }: {
    split: any;
    start_date: any;
    amounts: any;
  }) {
    if (typeof split === "string") { this.split = split; }
    if (typeof startDate === "string") { this.startDate = new Date(startDate); }
    this.amounts = new Map();
    if (amounts && typeof amounts === "object") {
      Object.entries(amounts).forEach(([k, v]) => {
        if (typeof k === "string" && typeof v === "object") {
          const innerAmounts = new Map();
          if (v) {
            Object.entries(v).forEach(([k2, v2]) => {
              if (typeof k2 === "string" && typeof v2 === "string") {
                innerAmounts.set(k2, v2);
              }
            });
          }
          this.amounts.set(new Date(k), innerAmounts);
        }
      });
    }
  }
}
