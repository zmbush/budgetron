// @flow

import Cashflow from 'components/Cashflow';
import Categories from 'components/Categories';
import React from 'react';
import RollingBudget from 'components/RollingBudget';
import ByTimeframe from 'components/ByTimeframe';
import type { ComponentType } from 'react';
import { Report, TimedReportData, type Transaction } from 'util/data';

import Page from 'components/Page';

import style from './style.scss';

const componentConfig = (type) => {
  const config: { Component: string | ComponentType<*>, count: number } = {
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
  data: Array<Report>,
  timeframe: 'Year' | 'Quarter' | 'Month',
  transactions: { [key: string]: Transaction },
};

const TimeframeReports = (props: TimeframeReportsProps) => (
  props.data.map(({ data, report, key }) => {
    if (data instanceof TimedReportData) {
      const dataByTimeframe = data.by(props.timeframe);
      if (dataByTimeframe) {
        return (
          <ByTimeframe
            key={key}
            title={report.name}
            timeframe={props.timeframe}
            transactions={props.transactions}
            report={report}
            data={dataByTimeframe}
            className={style.report}
            {...componentConfig(report.config.type)}
          />
        );
      }
    }
    return null;
  })
);

type SimpleReportsProps = {
  data: Array<Report>,
};

const SimpleReports = (props: SimpleReportsProps) => props.data.map(({ data, report, key }) => {
  if (data instanceof TimedReportData) return null;
  const cfg = componentConfig(report.config.type);
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
  data: Array<Report>,
  transactions: { [key: string]: Transaction },
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
