import Money from "components/Money";
import Page from "components/Page";
import * as moment from "moment";
import * as React from "react";
import { ReportData, ReportInfo, Transaction } from "util/data";

const monthNames = [
  "January",
  "February",
  "March",
  "April",
  "May",
  "June",
  "July",
  "August",
  "September",
  "October",
  "November",
  "December",
];

interface IProps {
  timeframe: "Year" | "Quarter" | "Month";
  data: Map<Date, ReportData>;
  title: string;
  transactions: Map<string, Transaction>;
  report: ReportInfo;
  className?: string;
  count?: number;
  Component:
  | string
  | React.ComponentType<{
    data: ReportData;
    transactions: Map<string, Transaction>;
    report: ReportInfo;
    showGraph: boolean;
  }>;
}

interface IState {
  expanded: boolean;
  showGraph: Map<Date, boolean>;
}

export default class ByTimeframe extends React.Component<IProps, IState> {
  public static defaultProps = {
    className: null,
    count: 1,
  };

  constructor(props: IProps) {
    super(props);

    this.state = {
      expanded: false,
      showGraph: new Map(),
    };
  }

  public getMostRecent(): Date {
    return [...this.props.data.keys()]
      .sort((a, b) => a.getTime() - b.getTime())
      .reverse()[0];
  }

  public getAverageStartDate(): Date {
    const mostRecent = this.getMostRecent();
    switch (this.props.timeframe) {
      case "Year":
        return moment(mostRecent)
          .subtract(5, "years")
          .toDate();
      case "Quarter":
        return moment(mostRecent)
          .subtract(1, "year")
          .toDate();
      case "Month":
        return moment(mostRecent)
          .subtract(1, "year")
          .toDate();
      default:
        return mostRecent;
    }
  }

  public getDateAgo(
    count: moment.DurationInputArg1,
    unit: moment.DurationInputArg2,
  ): Date {
    return moment(this.getMostRecent())
      .subtract(count, unit)
      .toDate();
  }

  public printDate(date: Date): string | number {
    switch (this.props.timeframe) {
      case "Year":
        return date.getFullYear();
      case "Quarter":
        return `${date.getFullYear()} Q${date.getMonth() / 3 + 1}`;
      case "Month":
        return `${monthNames[date.getMonth()]} ${date.getFullYear()}`;
      default:
        return date.toLocaleDateString();
    }
  }

  public stats(
    count: moment.DurationInputArg1,
    unit: moment.DurationInputArg2,
  ): { mean: number; median: number } {
    const start = this.getDateAgo(count, unit);
    const dataPoints = [...this.props.data.entries()]
      .filter(([date]) => date >= start)
      .map(([, content]) => {
        if ("total" in content && typeof content.total === "function") {
          return [1, content.total()];
        }
        return [1, 0];
      })
      .sort((a, b) => a[1] - b[1]);
    const medianIndex = Math.min(
      Math.round(dataPoints.length / 2),
      dataPoints.length - 1,
    );
    const median = dataPoints[medianIndex][1];
    const [meanCount, meanSum] = dataPoints.reduce(
      ([c1, a1], [c2, a2]) => [c1 + c2, a1 + a2],
      [0, 0.0],
    );
    const mean = meanSum / meanCount;

    return { median, mean };
  }

  public toggleExpanded = () => {
    this.setState({ expanded: !this.state.expanded });
  }

  public toggleGraph = (date: Date) => {
    const { showGraph } = this.state;
    showGraph.set(date, !showGraph.get(date));
    this.setState({ showGraph });
  }

  public render() {
    if (this.props.data) {
      let timeframes = [...this.props.data.entries()]
        .sort((a, b) => a[0].getTime() - b[0].getTime())
        .reverse();
      if (!this.state.expanded) {
        timeframes = timeframes.slice(0, this.props.count);
      }

      const title = `${this.props.title} By ${this.props.timeframe}`;
      const { Component, className, report, transactions } = this.props;
      const { showGraph, expanded } = this.state;
      return (
        <Page
          className={className}
          expanded={expanded}
          title={title}
          onClick={this.toggleExpanded}
        >
          <b>6 Month Average:</b>{" "}
          <Money amount={this.stats(6, "months").mean} />
          <br />
          <b>6 Month Median:</b>{" "}
          <Money amount={this.stats(6, "months").median} />
          <br />
          <b>1 Year Average:</b> <Money amount={this.stats(1, "year").mean} />
          <br />
          <b>1 Year Median:</b> <Money amount={this.stats(1, "year").median} />
          <br />
          {timeframes.map(([date, content]) => (
            <div key={date.toISOString()}>
              <h1>{this.printDate(date)}</h1>
              <Component
                data={content}
                transactions={transactions}
                report={report}
                showGraph={!!showGraph.get(date)}
              />
            </div>
          ))}
        </Page>
      );
    }
    return null;
  }
}
