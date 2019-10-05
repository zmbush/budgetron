// Copyright 2019 Zachary Bush.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

export class Timeseries<TSData> {
  public data: Array<{ date: number } & TSData>;

  constructor(data: any[], dataConstructor: ({ }) => TSData | null) {
    this.data = [];
    data.forEach((datum) => {
      if (!datum || typeof datum !== "object") { return; }
      if (!datum.value || typeof datum.value !== "object") { return; }
      const innerData = dataConstructor(datum.value);
      if (!innerData) { return; }
      if (typeof datum.date !== "string") { return; }
      this.data.push(
        Object.assign({ date: new Date(datum.date).getTime() }, innerData),
      );
    });
  }
}
