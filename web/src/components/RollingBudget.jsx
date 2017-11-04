// @flow

import React from 'react';
import Money from 'components/Money';
import type { RollingBudgetData, ReportInfo, Transaction } from 'util/budgetron-types';
import Transactions from 'components/Transactions';

type Props = {
  data: RollingBudgetData,
  report: ReportInfo,
  transactions: { [uid: string]: Transaction },
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
    const parts = Object.entries(this.props.report.config.amounts)
      .map(([name, amount]) => [name, parseFloat(amount)]);
    const total = parts.map(([, amount]) => amount).reduce((s, v) => s + v, 0);
    return parts.reduce((acc, [name, v]) => {
      acc[name] = v / total;
      return acc;
    }, {});
  }

  renderBudgets() {
    return Object.entries(this.props.data.budgets).map(([person, budget]) => (
      <div key={person}>
        <button onClick={() => this.toggleTable(person)}>
          {person}
        </button>: <Money amount={budget} />
        { (this.state.show[person]) ? <Transactions
          transaction_ids={this.props.data.transactions}
          transactions={this.props.transactions}
          filter={([, t]) => t.person === person || t.person === this.props.report.config.split}
          transform={(t) => {
            if (t.person === this.props.report.config.split) {
              const proportion = this.proportions()[person];
              return Object.assign({}, t, { amount: t.amount * proportion });
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
