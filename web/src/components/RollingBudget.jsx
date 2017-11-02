import React from 'react';
import Money from 'components/Money';
import PropTypes from 'prop-types';
import Transactions from 'components/Transactions';

export default class RollingBudget extends React.Component {
  static propTypes = {
    data: PropTypes.object, // eslint-disable-line react/forbid-prop-types
  };

  renderBudgets() {
    return Object.entries(this.props.data.budgets).map(([person, budget]) => (
      <div key={person}>
        <b>{person}</b>: <Money amount={budget} />
      </div>
    ));
  }

  render() {
    return (
      <div>
        { this.renderBudgets() }
        <Transactions
          transaction_ids={ this.props.data.transactions }
          transactions={this.props.transactions} />
      </div>
    );
  }
}
