// @flow

import * as React from "react";
import { Transaction } from "util/data";
import * as style from "./style.scss";

const COLUMNS = [
  "date",
  "amount",
  "person",
  "description",
  "originalDescription",
  "transactionType",
  "category",
  "originalCategory",
  "accountName",
  "labels",
  "notes",
  "tags",
];

interface IDetailsTableIProps {
  show?: boolean;
  colSpan?: number;
  transaction: Transaction;
}

const DetailsTable = (props: IDetailsTableIProps) => {
  if (!props.show) { return null; }
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
            {COLUMNS.map((c) => {
              const data = props.transaction.render(c);
              if (!data) { return null; }
              return (
                <tr key={c} className={style.normal_row}>
                  <td>{Transaction.transactionName(c)}</td>
                  <td>{data}</td>
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
  colSpan: 1,
  show: false,
};

interface IProps {
  columns: string[];
  transaction_ids: string[];
  transactions: Map<string, Transaction>;

  filter: (entry: [string, Transaction]) => boolean;
  transform: (t: Transaction) => Transaction;
}

interface IState {
  show: { [uid: string]: boolean };
}

export default class Transactions extends React.Component<IProps, IState> {
  public static defaultProps = {
    columns: ["date", "amount", "person", "description"],
    filter: () => true,
    transform: (t: Transaction) => t,
  };

  constructor(props: IProps) {
    super(props);

    this.state = {
      show: {},
    };
  }

  public toggleDetails(tid: string) {
    const { show } = this.state;
    show[tid] = !show[tid];
    this.setState({ show });
  }

  public fetchTransactionDetails(tid: string): Transaction {
    const transaction = this.props.transactions.get(tid);
    if (transaction) { return transaction; }

    const year = tid.slice(0, 4);
    const month = tid.slice(4, 6);
    const day = tid.slice(6, 8);
    const money = tid.slice(8, 18);

    let type = tid.slice(18, 19);
    if (type === "D") {
      type = "Debit";
    } else if (type === "C") {
      type = "Credit";
    } else if (type === "T") {
      type = "Trasnfer";
    }
    return new Transaction(
      "Unknown",
      `${money.slice(0, 6)}.${money.slice(6, 10)}`,
      "unknown",
      new Date(`${month}/${day}/${year}`),
      "Unknown",
      "",
      "",
      "",
      "UNKNOWN",
      "unknown",
      ["details not exported"],
      type,
    );
  }

  public renderHeaders() {
    return this.props.columns.map((id) => (
      <th key={id}>{Transaction.transactionName(id)}</th>
    ));
  }

  public renderRowCells(t: Transaction) {
    return this.props.columns.map((id) => <td key={id}>{t.render(id)}</td>);
  }

  public render() {
    const transactions = this.props.transaction_ids
      .sort()
      .map((tid): [string, Transaction] => [
        tid,
        this.fetchTransactionDetails(tid),
      ])
      .filter(this.props.filter)
      .map(([tid, t]): [string, Transaction] => [tid, this.props.transform(t)])
      .reverse();
    return (
      <table className={style.table}>
        <thead>
          <tr>{this.renderHeaders()}</tr>
        </thead>
        <tbody>
          {transactions.map(([tid, transaction]) => [
            <tr
              key={tid}
              onClick={() => this.toggleDetails(tid)}
              className={style.normal_row}
            >
              {this.renderRowCells(transaction)}
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
