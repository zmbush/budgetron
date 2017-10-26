import React from 'react';
import Money from 'components/Money';

export default class Categories extends React.Component {
  render() {
    let reverse = this.props.config.only_type != 'Debit';
    let categories = Object.entries(this.props.data).sort((a, b) => a[1] - b[1]);
    if (reverse) {categories = categories.reverse();}

    return <table><tbody>
        { categories.map(([category, amount]) => {
          return <tr key={category}>
            <td><b>{ category }</b></td><td><Money amount={amount} invert={ !reverse }/></td>
          </tr>;
        }) }
        <br/><br />
    </tbody></table>;
  }
}
