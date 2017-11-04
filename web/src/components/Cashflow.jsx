import React from 'react';
import BudgetronTypes from 'budgetron-types';
import Money from 'components/Money';

const Cashflow = (props) => {
  const { credit, debit } = props.data;
  return (
    <span>
      <Money amount={credit} /> - <Money amount={debit} /> = <Money amount={credit - debit} />
    </span>
  );
};

Cashflow.propTypes = {
  data: BudgetronTypes.CashflowData.isRequired,
};

export default Cashflow;
