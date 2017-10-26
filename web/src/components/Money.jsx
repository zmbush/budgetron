import PropTypes from 'prop-types';
import React from 'react';

const Money = (props) => {
  let amount = parseFloat(props.amount);
  if (props.invert) { amount = -amount; }
  const color = (amount > 0) ? 'black' : 'red';
  const dollars = amount.toLocaleString('en-US', {
    style: 'currency',
    currency: 'USD',
  });
  return <span style={{ color }}>{ dollars }</span>;
};

Money.propTypes = {
  invert: PropTypes.bool,
  amount: PropTypes.string.isRequired,
};

Money.defaultProps = {
  invert: false,
};

export default Money;
