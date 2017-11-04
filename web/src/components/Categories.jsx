// @flow

import React from 'react';
import Money from 'components/Money';
import type { ReportInfo, CategoriesData, Transaction } from 'util/budgetron-types';
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

type Props = {
  report: ReportInfo,
  data: CategoriesData,
  transactions: { [uid: string]: Transaction },
};

type State = {
  expanded: { [category: string]: bool },
};

export default class Categories extends React.Component<Props, State> {
  constructor(props: Props) {
    super(props);

    this.state = {
      expanded: {},
    };
  }

  toggleExpanded(category: string) {
    const { expanded } = this.state;
    expanded[category] = !expanded[category];
    this.setState({ expanded });
  }

  render() {
    const reverse = this.props.report.only_type !== 'Debit';
    let categories: Array<[string, Transaction]> = Object.entries(this.props.data);

    categories.sort((a, b) => a[1].amount - b[1].amount);
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
