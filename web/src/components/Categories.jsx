import React from 'react';
import Money from 'components/Money';
import Transactions from 'components/Transactions';

const CategoryEntry = (props) => {
  return [
    <tr key="row">
      <td><b onClick={ props.onClick }>{ props.category }</b></td>
      <td><Money amount={ props.amount } invert={ props.invert } /></td>
    </tr>,
    <tr key="transactions">
      { (props.expanded) ? <Transactions {...props}/> : null }
    </tr>
  ];
};

export default class Categories extends React.Component {
  constructor(props) {
    super(props);

    this.state = {
      expanded: {}
    };
  }

  toggleExpanded(category) {
    let expanded = this.state.expanded;
    expanded[category] = !expanded[category];
    this.setState({ expanded });
  }

  render() {
    let reverse = this.props.config.only_type != 'Debit';
    let categories = Object.entries(this.props.data).sort((a, b) => a[1].amount - b[1].amount);
    if (reverse) {categories = categories.reverse();}

    return <table><tbody>
        { categories.map(([category, amount]) => (
          <CategoryEntry
            key = {category}
            category = {category}
            amount = {amount.amount}
            transaction_ids = {amount.transactions}
            transactions = { this.props.transactions }
            invert = {!reverse}
            expanded = { !!this.state.expanded[category] }
            onClick = { () => this.toggleExpanded(category) }/>
        ))}
        <tr/>
        <tr/>
    </tbody></table>;
  }
}
