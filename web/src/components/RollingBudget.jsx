import React from 'react';
import Money from 'components/Money';
import PropTypes from 'prop-types';

export default class RollingBudget extends React.Component {
  propTypes = {
    data: PropTypes.object, // eslint-disable-line react/forbid-prop-types
  }

  renderBudgets() {
    return Object.entries(this.props.data.budgets).map(([person, budget]) => (
      <div key={person}>
        <b>{person}</b>: <Money amount={budget} />
      </div>
    ));
  }

  renderTransactions() {
    return <table>
      <tbody>{
        this.props.data.transactions
          .map((tid) => this.props.transactions[tid])
          .sort((a,b) => a.amount - b.amount)
          .reverse()
          .map((transaction) => (
          <tr>
            <td>{ transaction.person }</td>
            <td>{ transaction.description }</td>
            <td>{ transaction.amount }</td>
            <td>{ transaction.transaction_type }</td>
            <td>{ transaction.original_description }</td>
          </tr>
        ))
      }</tbody>
    </table>;
  }

  render() {
    return (
      <div>
        { this.renderBudgets() }
        { this.renderTransactions() }
      </div>
    );
  }
}
