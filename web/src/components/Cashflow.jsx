import React from 'react';
import Money from 'components/Money';

export default class Cashflow extends React.Component {
  render() {
    let {
      credit,
      debit
    } = this.props.data;
    return <span>
      <Money amount={credit} /> - <Money amount={debit} /> = <Money amount={credit - debit} />
    </span>;
  }
}
