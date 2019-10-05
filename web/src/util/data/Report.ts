// Copyright 2019 Zachary Bush.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

import { ReportData, ReportInfo, TimedReportData } from "util/data";

export class Report {
  public static parseReport(key: string, report: {}, data: { [part: string]: any }) {
    const me = new Report();
    me.key = key;
    me.report = new ReportInfo(report);

    const parsedData = me.report.parseData(data);
    if (parsedData === null) {
      return null;
    }
    me.data = parsedData;

    return me;
  }
  public key: string;
  public report: ReportInfo;
  public data: TimedReportData | ReportData;
}
