import PropTypes from 'prop-types';
import React from 'react';
import styles from './style.scss';

const Money = (props) => {
  let amount = parseFloat(props.amount);
  if (props.invert) { amount = -amount; }
  const className = (amount > 0) ? 'positive' : 'negative';
  const dollars = amount.toLocaleString('en-US', {
    style: 'currency',
    currency: 'USD',
  });
  return <span className={styles[className]}>{ dollars }</span>;
};

Money.propTypes = {
  invert: PropTypes.bool,
  amount: PropTypes.oneOfType([PropTypes.string, PropTypes.number]).isRequired,
};

Money.defaultProps = {
  invert: false,
};

export default Money;
