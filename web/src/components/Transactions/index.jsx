// @flow

import React from 'react';
import Money from 'components/Money';
import Tag from 'components/Tag';
import type { Transaction } from 'util/budgetron-types';
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

type DetailsTableProps = {
  show?: bool,
  colSpan?: number,
  transaction: Transaction,
};

const DetailsTable = (props: DetailsTableProps) => {
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

DetailsTable.defaultProps = {
  show: false,
  colSpan: 1,
};

type Props = {
  columns: Array<string>,
  transaction_ids: Array<string>,
  transactions: { [uid: string]: Transaction },

  filter: (t: Transaction) => bool,
  transform: (t: Transaction) => Transaction,
};

type State = {
  show: { [uid: string]: bool },
};

export default class Transactions extends React.Component<Props, State> {
  static defaultProps = {
    columns: ['date', 'amount', 'person', 'description'],
    filter: () => true,
    transform: t => t,
  };

  constructor(props: Props) {
    super(props);

    this.state = {
      show: {},
    };
  }

  toggleDetails(tid: string) {
    const { show } = this.state;
    show[tid] = !show[tid];
    this.setState({ show });
  }

  fetchTransactionDetails(tid: string) {
    if (tid in this.props.transactions) {
      return this.props.transactions[tid];
    }

    const year = tid.slice(0, 4);
    const month = tid.slice(4, 6);
    const day = tid.slice(6, 8);
    const money = tid.slice(8, 18);

    let type = tid.slice(18, 19);
    if (type === 'D') {
      type = 'Debit';
    } else if (type === 'C') {
      type = 'Credit';
    } else if (type === 'T') {
      type = 'Trasnfer';
    }
    return {
      date: `${month}/${day}/${year}`,
      amount: `${money.slice(0, 6)}.${money.slice(6, 10)}`,
      transaction_type: type,
      person: 'unknown',
      description: 'Unknown',
      original_description: 'UNKNOWN',
      account_name: 'Unknown',
      labels: '',
      notes: '',
      tags: ['details not exported'],
    };
  }

  renderHeaders() {
    return this.props.columns.map(id => <th key={id}>{ getColumn(id).name }</th>);
  }

  renderRowCells(t: Transaction) {
    return this.props.columns.map(id => <td key={id}>{ getColumn(id).render(t) }</td>);
  }

  render() {
    const transactions = this.props.transaction_ids
      .sort()
      .map(tid => [tid, this.fetchTransactionDetails(tid)])
      .filter(this.props.filter)
      .map(([tid, t]) => [tid, this.props.transform(t)])
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
