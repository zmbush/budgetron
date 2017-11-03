import React from 'react';
import Money from 'components/Money';
import Tag from 'components/Tag';
import PropTypes from 'prop-types';
import style from './style.scss';

const COLUMNS = {
  date: 'Date',
  amount: {
    name: 'Amount',
    render: t => <Money amount={t.amount || 0} invert={t.transaction_type === 'Debit'} />,
  },
  person: 'Person',
  description: 'Description',
  original_description: 'Original Description',
  transaction_type: 'Transaction Type',
  category: 'Category',
  original_category: 'Original Category',
  account_name: {
    name: 'Account Name',
    render: (t) => {
      if (t.transaction_type === 'Transfer') {
        return `${t.account_name} -> ${t.transfer_destination_account}`;
      }
      return t.account_name;
    },
  },
  labels: 'Labels',
  notes: 'Notes',
  tags: {
    name: 'Tags',
    render: t => t.tags.map(tag => <Tag key={tag} text={tag} />),
  },
};

const getColumn = (id) => {
  let col = COLUMNS[id];
  if (typeof col === 'string') {
    col = { name: col, render: t => t[id] };
  }
  return col;
};

const DetailsTable = (props) => {
  if (!props.show) return null;
  return (
    <tr>
      <td colSpan={props.colSpan}>
        <table className={style.inner_table}>
          <thead>
            <tr>
              <th>Field</th>
              <th>Value</th>
            </tr>
          </thead>
          <tbody>
            {Object.keys(COLUMNS).map((c) => {
              const col = getColumn(c);
              const data = col.render(props.transaction);
              if (!data) return null;
              return (
                <tr key={c} className={style.normal_row}>
                  <td>{ col.name }</td>
                  <td>{ data }</td>
                </tr>
              );
            })}
          </tbody>
        </table>
      </td>
    </tr>
  );
};

DetailsTable.propTypes = {
  show: PropTypes.bool,
  colSpan: PropTypes.number,
  transaction: PropTypes.shape({
    date: PropTypes.string.isRequired,
    description: PropTypes.string.isRequired,
    amount: PropTypes.string.isRequired,
    transaction_type: PropTypes.string.isRequired,
    person: PropTypes.string.isRequired,
    original_description: PropTypes.string.isRequired,
    account_name: PropTypes.string.isRequired,
    labels: PropTypes.string.isRequired,
    notes: PropTypes.string.isReqiured,
    transfer_destination_account: PropTypes.string,
    tags: PropTypes.arrayOf(PropTypes.string).isRequired,
  }).isRequired,
};

DetailsTable.defaultProps = {
  show: false,
  colSpan: 1,
};

export default class Transactions extends React.Component {
  static propTypes = {
    columns: PropTypes.arrayOf(PropTypes.string),
    transaction_ids: PropTypes.arrayOf(PropTypes.string).isRequired,
    transactions: PropTypes.shape({}).isRequired,
  };

  static defaultProps = {
    columns: ['date', 'amount', 'person', 'description'],
  };

  constructor(props) {
    super(props);

    this.state = {
      show: {},
    };
  }

  toggleDetails(tid) {
    const { show } = this.state;
    show[tid] = !show[tid];
    this.setState({ show });
  }

  renderHeaders() {
    return this.props.columns.map(id => <th key={id}>{ getColumn(id).name }</th>);
  }

  renderRowCells(t) {
    return this.props.columns.map(id => <td key={id}>{ getColumn(id).render(t) }</td>);
  }

  render() {
    const transactions = this.props.transaction_ids
      .sort()
      .map(tid => [tid, this.props.transactions[tid] || {}])
      .reverse();
    return (
      <table className={style.table}>
        <thead>
          <tr>{ this.renderHeaders() }</tr>
        </thead>
        <tbody>
          {transactions.map(([tid, transaction]) => [
            <tr
              key={tid}
              onClick={() => this.toggleDetails(tid)}
              className={style.normal_row}
            >
              { this.renderRowCells(transaction) }
            </tr>,
            <DetailsTable
              colSpan={this.props.columns.length}
              key={`${tid} details`}
              show={this.state.show[tid]}
              transaction={transaction}
            />,
          ])}
        </tbody>
      </table>
    );
  }
}
