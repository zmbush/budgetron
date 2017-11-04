// @flow

export type Transaction = {
  date: string,
  description: string,
  amount: string | number,
  transaction_type: string,
  person: string,
  original_description: string,
  account_name: string,
  labels: string,
  notes: string,
  transfer_destination_account?: string,
  tags: Array<string>,
};

export type RollingBudgetConfig = {
  type: 'RollingBudget',
  split: string,
  start_date: string,
  amounts: { [key: string]: string },
};

export type ReportConfig = RollingBudgetConfig | {
  type: 'Cashflow' | 'Categories',
};

export type ReportInfo = {
  name: string,
  config: ReportConfig,
  skip_tags?: Array<string>,
  only_type?: string,
  by_week?: bool,
  by_month?: bool,
  by_quarter?: bool,
  by_year?: bool,
}

export type RollingBudgetData = {
  budgets: { [person: string]: string },
  transactions: Array<string>,
}

export type CashflowData = {
  credit: string,
  debit: string,
};

export type CategoriesData = {
  amount: string,
  transactions: Array<string>,
};

export type ReportDataBase = RollingBudgetData | CashflowData | CategoriesData;

export type ReportData = ReportDataBase | {
  by_week?: { [date: string]: ReportDataBase },
  by_month?: { [date: string]: ReportDataBase },
  by_quarter?: { [date: string]: ReportDataBase },
  by_year?: { [date: string]: ReportDataBase },
};

export type Report = {
  key: string,
  report: ReportInfo,
  data: ReportData,
};
