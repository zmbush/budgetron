// Copyright 2019 Zachary Bush.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

import { ReportData, ReportInfo } from "util/data";

export class TimedReportData {

  public static mapFromData(
    info: ReportInfo,
    data: { [date: string]: any },
  ): Map<Date, ReportData> {
    const map = new Map();
    Object.entries(data).forEach(([k, v]) => {
      if (typeof k === "string" && v && typeof v === "object") {
        const rd = info.parseReportData(v);
        if (rd) {
          map.set(new Date(k), rd);
        }
      }
    });
    return map;
  }
  public byWeek?: Map<Date, ReportData>;
  public byMonth?: Map<Date, ReportData>;
  public byQuarter?: Map<Date, ReportData>;
  public byYear?: Map<Date, ReportData>;

  constructor(info: ReportInfo, data: { [part: string]: any }) {
    if (data.by_week && typeof data.by_week === "object") {
      this.byWeek = TimedReportData.mapFromData(info, data.by_week);
    }
    if (data.by_month && typeof data.by_month === "object") {
      this.byMonth = TimedReportData.mapFromData(info, data.by_month);
    }
    if (data.by_quarter && typeof data.by_quarter === "object") {
      this.byQuarter = TimedReportData.mapFromData(info, data.by_quarter);
    }
    if (data.by_year && typeof data.by_year === "object") {
      this.byYear = TimedReportData.mapFromData(info, data.by_year);
    }
  }

  public by(
    timeframe: "Week" | "Month" | "Quarter" | "Year",
  ): Map<Date, ReportData> | undefined {
    switch (timeframe) {
      case "Week":
        return this.byWeek;
      case "Month":
        return this.byMonth;
      case "Quarter":
        return this.byQuarter;
      case "Year":
        return this.byYear;
      default:
        return undefined;
    }
  }
}
