// @flow

import React from 'react';
import type { CashflowData } from 'util/budgetron-types';
import Money from 'components/Money';

type Props = {
  data: CashflowData,
};

const Cashflow = (props: Props) => {
  const { credit, debit } = props.data;
  return (
    <span>
      <Money amount={credit} /> - <Money amount={debit} /> = <Money amount={credit - debit} />
    </span>
  );
};

export default Cashflow;
