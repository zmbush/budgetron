// @flow

import Cashflow from 'components/Cashflow';
import Categories from 'components/Categories';
import React from 'react';
import RollingBudget from 'components/RollingBudget';
import ByTimeframe from 'components/ByTimeframe';
import type { TransactionType, ReportType } from 'util/budgetron-types';

import Page from 'components/Page';

import style from './style.scss';

const componentConfig = (type) => {
  const config = {
    Component: 'div',
    count: 1,
  };
  if (type === 'RollingBudget') {
    config.Component = RollingBudget;
  } else if (type === 'Cashflow') {
    config.Component = Cashflow;
    config.count = 4;
  } else if (type === 'Categories') {
    config.Component = Categories;
  }

  return config;
};

type TimeframeReportsProps = {
  data: Array<ReportType>,
  timeframe: 'Year' | 'Quarter' | 'Month',
  transactions: { [key: string]: TransactionType },
};

const TimeframeReports = (props: TimeframeReportsProps) => {
  const timeframeKey = `by_${props.timeframe.toLocaleLowerCase()}`;
  const reports = props.data.filter(({ report }) => report[timeframeKey]);
  return reports.map(({ data, report, key }) => (
    <ByTimeframe
      key={key}
      title={report.name}
      timeframe={props.timeframe}
      transactions={props.transactions}
      report={report}
      data={data[timeframeKey]}
      className={style.report}
      {...componentConfig(report.config.type)}
    />
  ));
};

type SimpleReportsProps = {
  data: Array<ReportType>,
};

const SimpleReports = (props: SimpleReportsProps) => props.data.map(({ data, report, key }) => {
  const hasTimeframes = Object.keys(data).some(k => k.startsWith('by_'));
  const cfg = componentConfig(report.config.type);
  if (hasTimeframes) return null;
  return (
    <Page
      key={key}
      className={style.report}
      title={report.name}
    >
      <cfg.Component {...props} count={cfg.count} data={data} report={report} />
    </Page>
  );
});

type BudgetronProps = {
  data: Array<ReportType>,
  transactions: { [key: string]: TransactionType },
};

const Budgetron = (props: BudgetronProps) => (
  <div className={style.mainContent}>
    <SimpleReports {...props} />
    <TimeframeReports timeframe="Month" {...props} />
    <TimeframeReports timeframe="Quarter" {...props} />
    <TimeframeReports timeframe="Year" {...props} />
  </div>
);

export default Budgetron;
