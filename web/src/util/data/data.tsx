export class Timeseries<TSData> {
  data: Array<{ date: number } & TSData>;

  constructor(data: any[], dataConstructor: ({}) => TSData | null) {
    this.data = [];
    data.forEach(datum => {
      if (!datum || typeof datum !== "object") return;
      if (!datum.value || typeof datum.value !== "object") return;
      const innerData = dataConstructor(datum.value);
      if (!innerData) return;
      if (typeof datum.date !== "string") return;
      this.data.push(
        Object.assign({ date: new Date(datum.date).getTime() }, innerData)
      );
    });
  }
}

export class RollingBudgetConfig {
  type = "RollingBudget";
  split: string;
  startDate: Date;
  amounts: Map<string, string>;

  constructor({
    split,
    start_date: startDate,
    amounts
  }: {
    split: any;
    start_date: any;
    amounts: any;
  }) {
    if (typeof split === "string") this.split = split;
    if (typeof startDate === "string") this.startDate = new Date(startDate);
    this.amounts = new Map();
    if (amounts && typeof amounts === "object") {
      Object.entries(amounts).forEach(([k, v]) => {
        if (typeof k === "string" && typeof v === "string") {
          this.amounts.set(k, v);
        }
      });
    }
  }
}

export class RollingBudgetData {
  budgets: Map<string, string>;
  transactions: Array<string>;
  timeseries?: Timeseries<{}>;

  constructor(data: { [part: string]: any }) {
    this.budgets = new Map();
    if (typeof data.budgets === "object") {
      Object.entries(data.budgets).forEach(([k, v]) => {
        if (typeof k === "string" && typeof v === "string") {
          this.budgets.set(k, v);
        }
      });
    }
    this.transactions = [];
    if (Array.isArray(data.transactions)) {
      data.transactions.forEach(t => {
        if (typeof t === "string") {
          this.transactions.push(t);
        }
      });
    }
    if (data.timeseries && Array.isArray(data.timeseries)) {
      this.timeseries = new Timeseries(
        data.timeseries,
        RollingBudgetData.parseTsDatum
      );
    }
  }

  static parseTsDatum(datum: {}): {} {
    const retval: { [date: string]: number } = {};
    Object.entries(datum).forEach(([k, v]) => {
      if (typeof k === "string" && typeof v === "string") {
        retval[k] = parseFloat(v);
      }
    });
    return retval;
  }
}

export class CashflowTsDatum {
  credit: number;
  debit: number;
  net: number;

  constructor(credit: number, debit: number, net: number) {
    this.credit = credit;
    this.debit = debit;
    this.net = net;
  }

  static parse(datum: { [part: string]: any }): CashflowTsDatum | null {
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
}

export class CashflowData {
  credit: string;
  debit: string;
  net: string;
  timeseries?: Timeseries<CashflowTsDatum>;

  constructor(data: { [part: string]: any }) {
    if (typeof data.credit === "string") this.credit = data.credit;
    if (typeof data.debit === "string") this.debit = data.debit;
    if (typeof data.net === "string") this.net = data.net;
    if (data.timeseries && Array.isArray(data.timeseries)) {
      this.timeseries = new Timeseries(data.timeseries, CashflowTsDatum.parse);
    }
  }
}

export class IncomeExpenseRatioDatum {
  byTag: Map<String, String>;
  other: String;

  constructor(data: { [part: string]: any }) {
    if (typeof data.other === "string") this.other = data.other;
    if (typeof data.other === "number") this.other = data.other.toString();
    this.byTag = new Map();
    if (typeof data.by_tag === "object") {
      Object.entries(data.by_tag).forEach(([k, v]) => {
        if (typeof k === "string" && typeof v === "string") {
          this.byTag.set(k, v);
        }
      });
    }
  }
}

export class IncomeExpenseRatioData {
  credit: IncomeExpenseRatioDatum;
  debit: IncomeExpenseRatioDatum;

  constructor(data: { [part: string]: any }) {
    if (typeof data.credit === "object")
      this.credit = new IncomeExpenseRatioDatum(data.credit);
    if (typeof data.debit === "object")
      this.debit = new IncomeExpenseRatioDatum(data.debit);
  }
}

export class CategoriesCategory {
  amount: string;
  transactions: Array<string>;

  constructor(data: { [part: string]: any }) {
    if (data.amount && typeof data.amount === "string")
      this.amount = data.amount;
    this.transactions = [];
    if (data.transactions && Array.isArray(data.transactions)) {
      data.transactions.forEach(t => {
        if (typeof t === "string") {
          this.transactions.push(t);
        }
      });
    }
  }
}

export class CategoriesData {
  categories: Map<string, CategoriesCategory>;
  timeseries?: Timeseries<{}>;

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
        CategoriesData.parseTsDatum
      );
    }
  }

  static parseTsDatum(datum: {}): {} {
    const retval: { [date: string]: number } = {};
    Object.entries(datum).forEach(([k, v]) => {
      if (typeof k === "string" && typeof v === "string") {
        retval[k] = parseFloat(v);
      }
    });
    return retval;
  }

  total(): number {
    return [...this.categories.values()].reduce(
      (total, category) => total + parseFloat(category.amount),
      0.0
    );
  }
}

export type ReportData =
  | RollingBudgetData
  | CashflowData
  | CategoriesData
  | IncomeExpenseRatioData;

export class TimedReportData {
  byWeek?: Map<Date, ReportData>;
  byMonth?: Map<Date, ReportData>;
  byQuarter?: Map<Date, ReportData>;
  byYear?: Map<Date, ReportData>;

  static mapFromData(
    info: ReportInfo,
    data: { [date: string]: any }
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

  by(
    timeframe: "Week" | "Month" | "Quarter" | "Year"
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

export class UIConfig {
  showDiff: boolean;
  expensesOnly: boolean;

  constructor({
    show_diff: showDiff,
    expenses_only: expensesOnly
  }: {
    show_diff: any;
    expenses_only: any;
  }) {
    if (typeof showDiff === "boolean") {
      this.showDiff = showDiff;
    }
    if (typeof expensesOnly === "boolean") {
      this.expensesOnly = expensesOnly;
    }
  }
}

export class ReportInfo {
  name: string;
  config:
    | RollingBudgetConfig
    | {
        type: "Cashflow" | "Categories";
      };
  uiConfig: UIConfig;
  skipTags?: Array<string>;
  onlyType: string;
  byWeek?: boolean;
  byMonth?: boolean;
  byQuarter?: boolean;
  byYear?: boolean;

  constructor(report: { [part: string]: any }) {
    if (typeof report.name === "string") this.name = report.name;
    if (Array.isArray(report.skip_tags)) {
      this.skipTags = [];
      report.skip_tags.forEach(t => {
        if (typeof t === "string" && this.skipTags) {
          this.skipTags.push(t);
        }
      });
    }
    if (typeof report.only_type === "string") this.onlyType = report.only_type;
    if (typeof report.by_week === "boolean") this.byWeek = report.by_week;
    if (typeof report.by_month === "boolean") this.byMonth = report.by_month;
    if (typeof report.by_quarter === "boolean")
      this.byQuarter = report.by_quarter;
    if (typeof report.by_year === "boolean") this.byYear = report.by_year;

    if (report.config && typeof report.config === "object") {
      if (report.config.type === "RollingBudget") {
        this.config = new RollingBudgetConfig(report.config);
      } else {
        this.config = {
          type: report.config.type
        };
      }
    }
    if (report.ui_config && typeof report.ui_config === "object") {
      this.uiConfig = new UIConfig(report.ui_config);
    }
  }

  parseReportData(data: { [part: string]: any }): ReportData | null {
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

  parseData(data: {
    [part: string]: any;
  }): TimedReportData | ReportData | null {
    if (data.by_week || data.by_month || data.by_quarter || data.by_year) {
      return new TimedReportData(this, data);
    }
    return this.parseReportData(data);
  }
}

export class Report {
  key: string;
  report: ReportInfo;
  data: TimedReportData | ReportData;

  static parseReport(key: string, report: {}, data: { [part: string]: any }) {
    let me = new Report();
    me.key = key;
    me.report = new ReportInfo(report);

    let parsedData = me.report.parseData(data);
    if (parsedData === null) {
      return null;
    }
    me.data = parsedData;

    return me;
  }
}

export function parseReports(reports: Array<any>): Array<Report> {
  const parsedReports: Report[] = [];

  reports.forEach(report => {
    if (
      typeof report === "object" &&
      report != null &&
      typeof report.key === "string" &&
      report.report &&
      typeof report.report === "object" &&
      report.data &&
      typeof report.data === "object"
    ) {
      let parsedReport = Report.parseReport(
        report.key,
        report.report,
        report.data
      );
      if (parsedReport !== null) {
        parsedReports.push(parsedReport);
      }
    }
  });

  return parsedReports;
}
