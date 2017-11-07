// @flow

import React from 'react';
import type { ComponentType } from 'react';
import { ReportInfo, type ReportData, type Transaction } from 'util/data';
import Page from 'components/Page';

const monthNames = [
  'January', 'February', 'March', 'April', 'May', 'June', 'July', 'August',
  'September', 'October', 'November', 'December',
];

type Props = {
  timeframe: 'Year' | 'Quarter' | 'Month',
  data: Map<Date, ReportData>,
  title: string,
  transactions: { [uid: string]: Transaction },
  report: ReportInfo,
  className?: string,
  count?: number,
  Component: string | ComponentType<{
    data: ReportData,
    transactions: { [uid: string]: Transaction },
    report: ReportInfo,
  }>,
};

type State = {
  expanded: bool,
};


export default class ByTimeframe extends React.Component<Props, State> {
  static defaultProps = {
    className: null,
    count: 1,
  };

  constructor(props: Props) {
    super(props);

    this.state = {
      expanded: false,
    };
  }

  printDate(date: Date) {
    if (this.props.timeframe === 'Year') {
      return date.getFullYear();
    } else if (this.props.timeframe === 'Quarter') {
      return `${date.getFullYear()} Q${(date.getMonth() / 3) + 1}`;
    } else if (this.props.timeframe === 'Month') {
      return `${monthNames[date.getMonth()]} ${date.getFullYear()}`;
    }
    return date.toLocaleDateString();
  }

  toggleExpanded = () => {
    this.setState({ expanded: !this.state.expanded });
  }

  render() {
    if (this.props.data) {
      let timeframes = [...this.props.data.entries()]
        .sort((a, b) => a[0] - b[0])
        .reverse();
      if (!this.state.expanded) {
        timeframes = timeframes.slice(0, this.props.count);
      }

      const title = `${this.props.title} By ${this.props.timeframe}`;
      const { Component } = this.props;
      return (
        <Page className={this.props.className} title={title} onClick={this.toggleExpanded}>
          { timeframes.map(([date, content]) => (
            <div key={date}>
              <b>{ this.printDate(date) }</b> <Component
                data={content}
                transactions={this.props.transactions}
                report={this.props.report}
              />
            </div>
          )) }
        </Page>
      );
    }
    return null;
  }
}
