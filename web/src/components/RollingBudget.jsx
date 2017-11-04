import React from 'react';
import Money from 'components/Money';
import PropTypes from 'prop-types';
import Transactions from 'components/Transactions';

export default class RollingBudget extends React.Component {
  static propTypes = {
    data: PropTypes.shape({
      budgets: PropTypes.shape({}),
      transactions: PropTypes.arrayOf(PropTypes.string),
    }).isRequired,
    report: PropTypes.shape({
      config: PropTypes.shape({
        split: PropTypes.string.isRequired,
        amounts: PropTypes.shape({}),
      }).isRequired,
    }).isRequired,
    transactions: PropTypes.shape({}).isRequired,
  };

  constructor(props) {
    super(props);

    this.state = {
      show: {},
    };
  }

  toggleTable(person) {
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
