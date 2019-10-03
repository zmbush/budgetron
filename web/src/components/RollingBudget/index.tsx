import * as React from "react";
import Money from "components/Money";
import {
  RollingBudgetData,
  RollingBudgetConfig,
  ReportInfo,
  Transaction
} from "util/data";
import Transactions from "components/Transactions";
import TimeseriesChart from "components/TimeseriesChart";

import * as style from "./style.scss";

type Props = {
  data: RollingBudgetData;
  report: ReportInfo & {
    config: RollingBudgetConfig;
  };
  transactions: Map<string, Transaction>;
};

type State = {
  show: string;
};

export default class RollingBudget extends React.Component<Props, State> {
  constructor(props: Props) {
    super(props);

    this.state = {
      show: ""
    };
  }

  toggleTable(person: string) {
    if (this.state.show === person) {
      this.setState({ show: "" });
    } else {
      this.setState({ show: person });
    }
  }

  proportions(): { [name: string]: number } {
    const parts: [string, number][] = [
      ...this.props.report.config.amounts.entries()
    ].map(([name, amount]) => [name, parseFloat(amount)]);

    const total = parts.map(([, amount]) => amount).reduce((s, v) => s + v, 0);
    return parts.reduce(
      (acc, [name, v]) => {
        acc[name] = v / total;
        return acc;
      },
      {} as { [name: string]: number }
    );
  }

  renderBudgets() {
    return [...this.props.data.budgets.entries()].map(
      ([person, budget]: [string, string]) => (
        <div key={person}>
          <button onClick={() => this.toggleTable(person)}>{person}</button>:{" "}
          <Money amount={budget} />
          {this.state.show === person ? (
            <Transactions
              transaction_ids={this.props.data.transactions}
              transactions={this.props.transactions}
              filter={([, t]) =>
                t.person === person ||
                t.person === this.props.report.config.split
              }
              transform={(t: Transaction) => {
                if (t.person === this.props.report.config.split) {
                  const proportion = this.proportions()[person];
                  let amount: number;
                  if (typeof t.amount === "string") {
                    amount = parseFloat(t.amount);
                  } else {
                    amount = t.amount;
                  }
                  amount *= proportion;
                  const obj = Object.assign({}, t, { amount });
                  Object.setPrototypeOf(obj, Transaction.prototype);
                  // convince flow I did indeed coerce it. facebook/flow#1138
                  if (obj instanceof Transaction) {
                    return obj;
                  }
                }
                return t;
              }}
            />
          ) : null}
        </div>
      )
    );
  }

  renderTimeseries() {
    const { timeseries } = this.props.data;
    if (!timeseries) {
      return null;
    }

    let lineNames: (string | null)[] = [
      ...this.props.report.config.amounts.keys()
    ];
    if (this.state.show !== "") {
      lineNames = [...this.props.report.config.amounts.keys()].map(k => {
        if (k === this.state.show) {
          return k;
        }
        return null;
      });
    }
    return (
      <TimeseriesChart
        className={style.graph}
        timeseries={timeseries}
        lineNames={lineNames}
      />
    );
  }

  render() {
    return (
      <div className={style.main}>
        <div className={style.data}>{this.renderBudgets()}</div>
        {this.renderTimeseries()}
      </div>
    );
  }
}
