// Copyright 2019 Zachary Bush.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

export class IncomeExpenseRatioDatum {
  public byTag: Map<string, string>;
  public other: string;

  constructor(data: { [part: string]: any }) {
    if (typeof data.other === "string") { this.other = data.other; }
    if (typeof data.other === "number") { this.other = data.other.toString(); }
    this.byTag = new Map();
    if (typeof data.by_tag === "object") {
      Object.entries(data.by_tag).forEach(([k, v]) => {
        if (typeof k === "string" && typeof v === "string") {
          this.byTag.set(k, v);
        }
      });
    }
  }

  public total() {
    let retval = parseInt(this.other, 10);
    for (const amount of this.byTag.values()) {
      retval += parseInt(amount, 10);
    }
    return retval;
  }
}
