import ByTimeframe from "components/ByTimeframe";
import Cashflow from "components/Cashflow";
import Categories from "components/Categories";
import IncomeExpenseRatio from "components/IncomeExpenseRatio";
import RollingBudget from "components/RollingBudget";
import Chip from "material-ui/Chip";
import * as React from "react";
import { Report, TimedReportData, Transaction } from "util/data";

import Page from "components/Page";

import * as style from "./style.scss";

const componentConfig = (
  type: "RollingBudget" | "Cashflow" | "Categories" | "IncomeExpenseRatio",
) => {
  const config: {
    Component: string | React.ComponentType<any>;
    count: number;
  } = {
    Component: "div",
    count: 1,
  };
  if (type === "RollingBudget") {
    config.Component = RollingBudget;
  } else if (type === "Cashflow") {
    config.Component = Cashflow;
    config.count = 4;
  } else if (type === "Categories") {
    config.Component = Categories;
  } else if (type === "IncomeExpenseRatio") {
    config.Component = IncomeExpenseRatio;
    config.count = 100;
  }

  return config;
};

interface ITimeframeReportsProps {
  data: Report[];
  timeframe: "Year" | "Quarter" | "Month";
  transactions: Map<string, Transaction>;
  display: boolean;
}

const TimeframeReports = (props: ITimeframeReportsProps) => {
  if (props.display) {
    return (
      <>
        {props.data.map(({ data, report, key }) => {
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
        })}
      </>
    );
  }
  return null;
};

interface ISimpleReportsProps {
  data: Report[];
}

const SimpleReports = (props: ISimpleReportsProps) => (
  <>
    {props.data.map(({ data, report, key }) => {
      if (data instanceof TimedReportData) {
        return null;
      }
      const cfg = componentConfig(report.config.type);
      return (
        <Page key={key} className={style.report} title={report.name}>
          <cfg.Component
            {...props}
            count={cfg.count}
            data={data}
            report={report}
          />
        </Page>
      );
    })}
  </>
);

interface IBudgetronProps {
  data: Report[];
  transactions: Map<string, Transaction>;
}

interface IBudgetronState {
  month: boolean;
  quarter: boolean;
  year: boolean;
}

class Budgetron extends React.Component<IBudgetronProps, IBudgetronState> {
  constructor(props: IBudgetronProps) {
    super(props);

    this.state = {
      month: false,
      quarter: false,
      year: false,
    };
  }

  public getChip(period: "month" | "quarter" | "year", description: string) {
    let bg;
    if (this.state[period]) {
      bg = "#00FF00";
    }
    return (
      <Chip backgroundColor={bg} onClick={() => this.toggle(period)}>
        {description}
      </Chip>
    );
  }

  public toggle(period: "month" | "quarter" | "year") {
    const newState: Pick<IBudgetronState, typeof period> = {
      month: this.state.month,
      quarter: this.state.quarter,
      year: this.state.year,
    };
    newState[period] = !newState[period];
    this.setState(newState);
  }

  public render() {
    return (
      <div className={style.mainContent}>
        <div className={style.chipBag}>
          {this.getChip("month", "By Month")}
          {this.getChip("quarter", "By Quarter")}
          {this.getChip("year", "By Year")}
        </div>
        <SimpleReports {...this.props} />
        <TimeframeReports
          display={this.state.month}
          timeframe="Month"
          {...this.props}
        />
        <TimeframeReports
          display={this.state.quarter}
          timeframe="Quarter"
          {...this.props}
        />
        <TimeframeReports
          display={this.state.year}
          timeframe="Year"
          {...this.props}
        />
      </div>
    );
  }
}

export default Budgetron;
