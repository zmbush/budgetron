export * from "./transactions";

export * from "./CashflowData";
export * from "./CashflowTsDatum";
export * from "./CategoriesCategory";
export * from "./CategoriesData";
export * from "./IncomeExpenseRatioData";
export * from "./IncomeExpenseRatioDatum";
export * from "./Report";
export * from "./ReportData";
export * from "./ReportInfo";
export * from "./RollingBudgetConfig";
export * from "./RollingBudgetData";
export * from "./TimedReportData";
export * from "./Timeseries";
export * from "./UIConfig";

import { Report } from "./Report";

export function parseReports(reports: any[]): Report[] {
  const parsedReports: Report[] = [];

  reports.forEach((report) => {
    if (
      typeof report === "object" &&
      report != null &&
      typeof report.key === "string" &&
      report.report &&
      typeof report.report === "object" &&
      report.data &&
      typeof report.data === "object"
    ) {
      const parsedReport = Report.parseReport(
        report.key,
        report.report,
        report.data,
      );
      if (parsedReport !== null) {
        parsedReports.push(parsedReport);
      }
    }
  });

  return parsedReports;
}
