// Copyright 2019 Zachary Bush.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

import {
  CashflowData,
  CategoriesData,
  IncomeExpenseRatioData,
  ReportData,
  RollingBudgetConfig,
  RollingBudgetData,
  TimedReportData,
  UIConfig
} from "util/data";

export class ReportInfo {
  public name: string;
  public config:
    | RollingBudgetConfig
    | {
      type: "Cashflow" | "Categories" | "IncomeExpenseRatio";
    };
  public uiConfig: UIConfig;
  public skipTags?: string[];
  public onlyType: string;
  public byWeek?: boolean;
  public byMonth?: boolean;
  public byQuarter?: boolean;
  public byYear?: boolean;

  constructor(report: { [part: string]: any }) {
    if (typeof report.name === "string") { this.name = report.name; }
    if (Array.isArray(report.skip_tags)) {
      this.skipTags = [];
      report.skip_tags.forEach((t) => {
        if (typeof t === "string" && this.skipTags) {
          this.skipTags.push(t);
        }
      });
    }
    if (typeof report.only_type === "string") { this.onlyType = report.only_type; }
    if (typeof report.by_week === "boolean") { this.byWeek = report.by_week; }
    if (typeof report.by_month === "boolean") { this.byMonth = report.by_month; }
    if (typeof report.by_quarter === "boolean") {
      this.byQuarter = report.by_quarter;
    }
    if (typeof report.by_year === "boolean") { this.byYear = report.by_year; }

    if (report.config && typeof report.config === "object") {
      if (report.config.type === "RollingBudget") {
        this.config = new RollingBudgetConfig(report.config);
      } else {
        this.config = {
          type: report.config.type,
        };
      }
    }
    if (report.ui_config && typeof report.ui_config === "object") {
      this.uiConfig = new UIConfig(report.ui_config);
    }
  }

  public parseReportData(data: { [part: string]: any }): ReportData | null {
    switch (this.config.type) {
      case "RollingBudget":
        return new RollingBudgetData(data);
      case "Cashflow":
        return new CashflowData(data);
      case "Categories":
        return new CategoriesData(data);
      case "IncomeExpenseRatio":
        return new IncomeExpenseRatioData(data);
      default:
        return null;
    }
  }

  public parseData(data: {
    [part: string]: any;
  }): TimedReportData | ReportData | null {
    if (data.by_week || data.by_month || data.by_quarter || data.by_year) {
      return new TimedReportData(this, data);
    }
    return this.parseReportData(data);
  }
}
