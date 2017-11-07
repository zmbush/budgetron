// @flow

import React from 'react';
import Money from 'components/Money';
import { RollingBudgetData, RollingBudgetConfig, ReportInfo, Transaction } from 'util/data';
import Transactions from 'components/Transactions';

type Props = {
  data: RollingBudgetData,
  report: ReportInfo & {
    config: RollingBudgetConfig,
  },
  transactions: Map<string, Transaction>,
};

type State = {
  show: { [person: string]: bool },
};

export default class RollingBudget extends React.Component<Props, State> {
  constructor(props: Props) {
    super(props);

    this.state = {
      show: {},
    };
  }

  toggleTable(person: string) {
    const { show } = this.state;
    show[person] = !show[person];
    this.setState({ show });
  }

  proportions() {
    const parts = [...this.props.report.config.amounts.entries()]
      .map(([name, amount]) => [name, parseFloat(amount)]);
    const total = parts.map(([, amount]) => amount).reduce((s, v) => s + v, 0);
    return parts.reduce((acc, [name, v]) => {
      acc[name] = v / total;
      return acc;
    }, {});
  }

  renderBudgets() {
    return [...this.props.data.budgets.entries()]
      .map(([person: string, budget: string]) => (
        <div key={person}>
          <button onClick={() => this.toggleTable(person)}>
            {person}
          </button>: <Money amount={budget} />
          { (this.state.show[person]) ? <Transactions
            transaction_ids={this.props.data.transactions}
            transactions={this.props.transactions}
            filter={([, t]) => t.person === person || t.person === this.props.report.config.split}
            transform={(t: Transaction) => {
              if (t.person === this.props.report.config.split) {
                const proportion = this.proportions()[person];
                const amount = parseFloat(t.amount) * proportion;
                const obj = Object.assign({}, t, { amount });
                Object.setPrototypeOf(obj, Transaction.prototype);
                // convince flow I did indeed coerce it. facebook/flow#1138
                if (obj instanceof Transaction) {
                  return obj;
                }
              }
              return t;
            }}
          /> : null }
        </div>
      ));
  }

  render() {
    return (
      <div>
        { this.renderBudgets() }
      </div>
    );
  }
}
