// Copyright 2019 Zachary Bush.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

export class CashflowTsDatum {
  public static parse(datum: { [part: string]: any }): CashflowTsDatum | null {
    let credit;
    let debit;
    let net;
    if (typeof datum.credit === "string") {
      credit = parseFloat(datum.credit);
    } else {
      return null;
    }
    if (typeof datum.debit === "string") {
      debit = parseFloat(datum.debit);
    } else {
      return null;
    }
    if (typeof datum.net === "string") {
      net = parseFloat(datum.net);
    } else {
      return null;
    }
    return new CashflowTsDatum(credit, debit, net);
  }
  public credit: number;
  public debit: number;
  public net: number;

  constructor(credit: number, debit: number, net: number) {
    this.credit = credit;
    this.debit = debit;
    this.net = net;
  }
}
