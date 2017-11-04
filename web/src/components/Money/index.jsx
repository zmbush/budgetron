// @flow

import React from 'react';
import styles from './style.scss';

type Props = {
  invert?: bool,
  amount: string | number,
};

const Money = (props: Props) => {
  let amount = parseFloat(props.amount);
  if (props.invert) { amount = -amount; }
  const className = (amount > 0) ? 'positive' : 'negative';
  const dollars = amount.toLocaleString('en-US', {
    style: 'currency',
    currency: 'USD',
  });
  return <span className={styles[className]}>{ dollars }</span>;
};

Money.defaultProps = {
  invert: false,
};

export default Money;
