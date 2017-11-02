import React from 'react';
import Money from 'components/Money';
import Tag from 'components/Tag';
import style from './style.scss';

const COLUMNS = {
  date: "Date",
  amount: {
    name: "Amount",
    render: (t) => <Money amount={ t.amount || 0 } invert={ t.transaction_type == "Debit" } />
  },
  person: "Person",
  description: "Description",
  original_description: "Original Description",
  transaction_type: "Transaction Type",
  category: "Category",
  original_category: "Original Category",
  account_name: {
    name: "Account Name",
    render: (t) => {
      if (t.transaction_type == "Transfer") {
        return `${t.account_name} -> ${t.transfer_destination_account}`;
      } else {
        return t.account_name;
      }
    }
  },
  labels: "Labels",
  notes: "Notes",
  tags: {
    name: "Tags",
    render: (t) => t.tags.map((tag) => <Tag key={ tag } text={ tag }/>)
  }
};

const getColumn = (id) => {
  let col = COLUMNS[id];
  if (typeof col == 'string') {
    col = { name: col, render: (t) => t[id] };
  }
  return col;
}

const DetailsTable = (props) => {
  if (!props.show) return null;
  return (
    <tr><td colSpan={ props.colSpan }>
      <table className={ style.inner_table }>
        <thead>
          <tr>
            <th>Field</th>
            <th>Value</th>
          </tr>
        </thead>
        <tbody>{Object.keys(COLUMNS).map((c) => {
          let col = getColumn(c);
          let data = col.render(props.transaction);
          if (!data) return null;
          return <tr key={ c } className={ style.normal_row }>
            <td>{ col.name }</td>
            <td>{ data }</td>
          </tr>;
        })}</tbody>
      </table>
    </td></tr>
  );
};

export default class Transactions extends React.Component {
  static defaultProps = {
    columns: ['date', 'amount', 'person', 'description'],
  };

  constructor(props) {
    super(props);

    this.state = {
      show: {},
    };
  }


  renderHeader(id) {
    return <th key={ id }>{ getColumn(id).name }</th>;
  }

  renderRowCell(id, t) {
    return <td key={ id }>{ getColumn(id).render(t) }</td>;
  }

  toggleDetails(tid) {
    let show = this.state.show;
    show[tid] = !show[tid];
    this.setState({ show });
  }

  render() {
    let transactions = this.props.transaction_ids
      .sort()
      .map((tid) => [tid, this.props.transactions[tid] || {}])
      .reverse()
    return (
      <table className={ style.table }>
        <thead>
          <tr>{ this.props.columns.map((id) => this.renderHeader(id)) }</tr>
        </thead>
        <tbody>{transactions.map(([tid, transaction]) => [
          <tr
            key={ tid }
            onClick={ () => this.toggleDetails(tid) }
            className={ style.normal_row }>
              { this.props.columns.map((id) => this.renderRowCell(id, transaction)) }
          </tr>,
          <DetailsTable
            colSpan={ this.props.columns.length }
            key={ tid + "details" }
            show={ this.state.show[tid] }
            transaction={ transaction }/>
        ])}</tbody>
      </table>
    );
  }
}
