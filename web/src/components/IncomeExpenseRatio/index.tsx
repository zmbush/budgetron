import Money from "components/Money";
import * as React from "react";
import {
  IncomeExpenseRatioData,
  IncomeExpenseRatioDatum,
  ReportInfo,
} from "util/data";

import * as styles from "./styles.scss";

interface IProps {
  data: IncomeExpenseRatioData;
  report: ReportInfo;
}

const parseFloatIfNeeded = (amount: string | number): number => {
  if (typeof amount === "string") {
    return parseFloat(amount);
  }
  return amount;
};

const MoneyPerc = ({
  amount,
  total,
}: {
  amount: string | number;
  total: string | number;
}) => {
  const amountFloat = parseFloatIfNeeded(amount);
  const totalFloat = parseFloatIfNeeded(total);
  const perc = ((amountFloat / totalFloat) * 100).toFixed(2);
  return (
    <span>
      <Money amount={amount} /> ({perc}%)
    </span>
  );
};

function titleCase(str: string) {
  return str
    .toLowerCase()
    .split(" ")
    .map((word) => word.replace(word[0], word[0].toUpperCase()))
    .join(" ");
}

const Category = ({
  amount,
  name,
  total,
}: {
  amount: string | number;
  name: string;
  total: string | number;
}) => {
  const prettyName = titleCase(name.split("-").join(" "));
  return (
    <div className={styles.category}>
      <h3>{prettyName}</h3>
      <MoneyPerc amount={amount} total={total} />
    </div>
  );
};

const Ratios = (props: {
  totalCredit: number;
  datum: IncomeExpenseRatioDatum;
}) => {
  const { totalCredit, datum } = props;
  const data = [];
  for (const [name, amount] of datum.byTag.entries()) {
    data.push(<Category name={name} amount={amount} total={totalCredit} />);
  }
  data.push(<Category name="other" amount={datum.other} total={totalCredit} />);
  return <div>{data}</div>;
};

const IncomeExpenseRatio = (props: IProps) => {
  const { credit, debit } = props.data;
  const totalCredit = credit.total();
  const totalDebit = debit.total();
  return (
    <div>
      <div className={styles.reportSection}>
        <h2>Income:</h2>
        <Ratios totalCredit={totalCredit} datum={credit} />
        <br />
      </div>

      <div className={styles.reportSection}>
        <h2>Expenses:</h2>
        <Ratios totalCredit={totalCredit} datum={debit} />
        <br />
      </div>

      <div className={styles.reportSection}>
        <h2>Savings:</h2>
        <Ratios
          totalCredit={totalCredit}
          datum={
            new IncomeExpenseRatioDatum({ other: totalCredit - totalDebit })
          }
        />
        <br />
      </div>
    </div>
  );
};

export default IncomeExpenseRatio;
