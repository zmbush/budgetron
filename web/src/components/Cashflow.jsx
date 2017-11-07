// @flow

import React from 'react';
import { CashflowData } from 'util/data';
import Money from 'components/Money';

type Props = {
  data: CashflowData,
};

const Cashflow = (props: Props) => {
  const { credit, debit } = props.data;
  const delta = parseInt(credit, 10) - parseInt(debit, 10);
  return (
    <span>
      <Money amount={credit} /> - <Money amount={debit} /> = <Money amount={delta} />
    </span>
  );
};

export default Cashflow;
