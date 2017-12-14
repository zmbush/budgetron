// @flow

import React from 'react';
import Money from 'components/Money';
import { ReportInfo, CategoriesData, Transaction } from 'util/data';
import Transactions from 'components/Transactions';
import TimeseriesChart from 'components/TimeseriesChart';

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
  transactions: Map<string, Transaction>,
  showGraph: bool,
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
    const reverse = this.props.report.onlyType !== 'Debit';
    let categories = [...this.props.data.categories.entries()]
      .sort((a, b) => parseFloat(a[1].amount) - parseFloat(b[1].amount));

    if (reverse) { categories = categories.reverse(); }

    return (
      <div>
        <b>Total:</b> <Money amount={this.props.data.total()} invert={!reverse} />
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
        { (this.props.data.timeseries && this.props.showGraph) ? <TimeseriesChart
          timeseries={this.props.data.timeseries}
          lineNames={[...this.props.data.categories.keys()]}
        /> : null }
      </div>
    );
  }
}
