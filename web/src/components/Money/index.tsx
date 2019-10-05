import * as React from "react";
import * as styles from "./style.scss";

interface IProps {
  invert?: boolean;
  amount: string | number;
}

const Money = (props: IProps) => {
  let amount;
  if (typeof props.amount === "number") {
    amount = props.amount;
  } else {
    amount = parseFloat(props.amount);
  }
  if (props.invert) {
    amount = -amount;
  }
  const className = amount > 0 ? "positive" : "negative";
  const dollars = amount.toLocaleString("en-US", {
    currency: "USD",
    style: "currency",
  });
  return <span className={styles[className]}>{dollars}</span>;
};

Money.defaultProps = {
  invert: false,
};

export default Money;
