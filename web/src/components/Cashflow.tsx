import Money from "components/Money";
import TimeseriesChart from "components/TimeseriesChart";
import * as React from "react";
import { CashflowData, ReportInfo } from "util/data";

interface IProps {
  data: CashflowData;
  report: ReportInfo;
}

const Cashflow = (props: IProps) => {
  const { credit, debit } = props.data;
  const delta = parseInt(credit, 10) - parseInt(debit, 10);
  if (props.report.uiConfig.expensesOnly) {
    return (
      <span>
        <Money amount={debit} />
      </span>
    );
  } else if (props.report.uiConfig.showDiff) {
    return (
      <span>
        <Money amount={credit} /> - <Money amount={debit} /> ={" "}
        <Money amount={delta} />
        {props.data.timeseries ? (
          <TimeseriesChart
            timeseries={props.data.timeseries}
            lineNames={["credit", "debit", "net"]}
          />
        ) : null}
      </span>
    );
  }
  return (
    <span>
      Income: <Money amount={credit} /> Expense: <Money amount={debit} />
      {props.data.timeseries ? (
        <TimeseriesChart
          timeseries={props.data.timeseries}
          lineNames={["credit", "debit"]}
        />
      ) : null}
    </span>
  );
};

export default Cashflow;
