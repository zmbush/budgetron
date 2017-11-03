import React from 'react';
import PropTypes from 'prop-types';
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
  data: PropTypes.shape({
    credit: PropTypes.string,
    debit: PropTypes.string,
  }).isRequired,
};

export default Cashflow;
