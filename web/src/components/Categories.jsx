import React from 'react';
import Money from 'components/Money';
import PropTypes from 'prop-types';
import Transactions from 'components/Transactions';

const CategoryEntry = props => [
  <tr key="row">
    <td><button onClick={props.onClick}>{ props.category }</button></td>
    <td><Money amount={props.amount} invert={props.invert} /></td>
  </tr>,
  <tr key="transactions">
    { (props.expanded) ? <Transactions {...props} /> : null }
  </tr>,
];

export default class Categories extends React.Component {
  static propTypes = {
    config: PropTypes.shape({
      only_type: PropTypes.string,
    }).isRequired,
    data: PropTypes.shape({}).isRequired,
    transactions: PropTypes.shape({}).isRequired,
  };

  constructor(props) {
    super(props);

    this.state = {
      expanded: {},
    };
  }

  toggleExpanded(category) {
    const { expanded } = this.state;
    expanded[category] = !expanded[category];
    this.setState({ expanded });
  }

  render() {
    const reverse = this.props.config.only_type !== 'Debit';
    let categories = Object.entries(this.props.data).sort((a, b) => a[1].amount - b[1].amount);
    if (reverse) { categories = categories.reverse(); }

    return (
      <table>
        <tbody>
          { categories.map(([category, amount]) => (
            <CategoryEntry
              key={category}
              category={category}
              amount={amount.amount}
              transaction_ids={amount.transactions}
              transactions={this.props.transactions}
              invert={!reverse}
              expanded={!!this.state.expanded[category]}
              onClick={() => this.toggleExpanded(category)}
            />
          ))}
          <tr />
          <tr />
        </tbody>
      </table>
    );
  }
}
