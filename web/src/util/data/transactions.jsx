// @flow

import * as React from 'react';
import Money from 'components/Money';
import Chip from 'material-ui/Chip';

export class Transaction {
  accountName: string
  amount: string | number
  category: string
  date: Date
  description: string
  labels: string
  notes: string
  originalCategory: string
  originalDescription: string
  person: string
  tags: Array<string>
  transactionType: string
  transferDestinationAccount: ?string

  constructor(
    accountName: string, amount: string | number, category: string, date: Date,
    description: string, labels: string, notes: string, originalCategory: string,
    originalDescription: string, person: string, tags: Array<mixed>,
    transactionType: string, data?: {transferDestinationAccount?: mixed},
  ) {
    this.accountName = accountName;
    this.amount = amount;
    this.category = category;
    this.date = date;
    this.description = description;
    this.labels = labels;
    this.notes = notes;
    this.originalCategory = originalCategory;
    this.originalDescription = originalDescription;
    this.person = person;
    this.tags = [];
    tags.forEach((t) => {
      if (typeof t === 'string') {
        this.tags.push(t);
      }
    });
    this.transactionType = transactionType;
    if (data && data.transferDestinationAccount && typeof data.transferDestinationAccount === 'string') {
      this.transferDestinationAccount = data.transferDestinationAccount;
    }
  }

  static parse(data: mixed): ?Transaction {
    if (data
      && typeof data === 'object'
      && typeof data.account_name === 'string'
      && typeof data.amount === 'string'
      && typeof data.category === 'string'
      && typeof data.date === 'string'
      && typeof data.description === 'string'
      && typeof data.labels === 'string'
      && typeof data.notes === 'string'
      && typeof data.original_category === 'string'
      && typeof data.original_description === 'string'
      && typeof data.person === 'string'
      && Array.isArray(data.tags)
      && typeof data.transaction_type === 'string') {
      return new Transaction(
        data.account_name,
        data.amount,
        data.category,
        new Date(data.date),
        data.description,
        data.labels,
        data.notes,
        data.original_category,
        data.original_description,
        data.person,
        data.tags,
        data.transaction_type,
        data,
      );
    }

    return null;
  }

  render(name: string): null | string | React.Node {
    switch (name) {
      case 'accountName':
        if (this.transferDestinationAccount) {
          return `${this.accountName} -> ${this.transferDestinationAccount}`;
        }
        return this.accountName;
      case 'amount': return <Money amount={this.amount} invert={this.transactionType === 'Debit'} />;
      case 'category': return this.category;
      case 'date': return this.date.toLocaleDateString();
      case 'description': return this.description;
      case 'labels': return this.labels;
      case 'notes': return this.notes;
      case 'originalCategory': return this.originalCategory;
      case 'originalDescription': return this.originalDescription;
      case 'person': return this.person;
      case 'tags': return this.tags.map(tag => <Chip key={tag}>{ tag }</Chip>);
      case 'transactionType':
      default:
        return null;
    }
  }

  static name(name: string): string {
    const result = name.replace(/([A-Z])/g, ' $1');
    return result.charAt(0).toUpperCase() + result.slice(1);
  }
}

export function parseTransactions(transactions: { [string]: mixed }): Map<string, Transaction> {
  const parsedTransactions = new Map();

  Object.entries(transactions).forEach(([uid, transaction]) => {
    if (typeof uid === 'string') {
      const t = Transaction.parse(transaction);
      if (t) parsedTransactions.set(uid, t);
    }
  });

  return parsedTransactions;
}
