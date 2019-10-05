// Copyright 2019 Zachary Bush.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

import { CategoriesCategory, Timeseries } from "util/data";

export class CategoriesData {
  public static parseTsDatum(datum: {}): {} {
    const retval: { [date: string]: number } = {};
    Object.entries(datum).forEach(([k, v]) => {
      if (typeof k === "string" && typeof v === "string") {
        retval[k] = parseFloat(v);
      }
    });
    return retval;
  }
  public categories: Map<string, CategoriesCategory>;
  public timeseries?: Timeseries<{}>;

  constructor(data: { [part: string]: any }) {
    this.categories = new Map();
    if (data.categories && typeof data.categories === "object") {
      Object.entries(data.categories).forEach(([k, v]) => {
        if (typeof k === "string" && typeof v === "object" && v !== null) {
          this.categories.set(k, new CategoriesCategory(v));
        }
      });
    }
    if (data.timeseries && Array.isArray(data.timeseries)) {
      this.timeseries = new Timeseries(
        data.timeseries,
        CategoriesData.parseTsDatum,
      );
    }
  }

  public total(): number {
    return [...this.categories.values()].reduce(
      (total, category) => total + parseFloat(category.amount),
      0.0,
    );
  }
}
