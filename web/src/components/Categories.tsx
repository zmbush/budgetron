import Money from "components/Money";
import TimeseriesChart from "components/TimeseriesChart";
import Transactions from "components/Transactions";
import * as React from "react";
import { CategoriesData, ReportInfo, Transaction } from "util/data";

const CategoryEntry = (props: {
  onClick: () => void;
  category: string;
  amount: string | number;
  invert: boolean;
  expanded: boolean;
  transaction_ids: string[];
  transactions: Map<string, Transaction>;
}) => (
    <>
      <tr key="row">
        <td>
          <button onClick={props.onClick}>{props.category}</button>
        </td>
        <td>
          <Money amount={props.amount} invert={props.invert} />
        </td>
      </tr>
      <tr key="transactions">
        {props.expanded ? <Transactions {...props} /> : null}
      </tr>
    </>
  );

interface IProps {
  report: ReportInfo;
  data: CategoriesData;
  transactions: Map<string, Transaction>;
  showGraph: boolean;
}

interface IState {
  expanded: { [category: string]: boolean };
}

export default class Categories extends React.Component<IProps, IState> {
  constructor(props: IProps) {
    super(props);

    this.state = {
      expanded: {},
    };
  }

  public toggleExpanded(category: string) {
    const { expanded } = this.state;
    expanded[category] = !expanded[category];
    this.setState({ expanded });
  }

  public render() {
    const reverse = this.props.report.onlyType !== "Debit";
    let categories = [...this.props.data.categories.entries()].sort(
      (a, b) => parseFloat(a[1].amount) - parseFloat(b[1].amount),
    );

    if (reverse) {
      categories = categories.reverse();
    }

    return (
      <div>
        <b>Total:</b>{" "}
        <Money amount={this.props.data.total()} invert={!reverse} />
        <table>
          <tbody>
            {categories.map(([category, amount]) => (
              <CategoryEntry
                key={category}
                category={category}
                amount={amount.amount}
                transaction_ids={amount.transactions}
                transactions={this.props.transactions}
                invert={!reverse}
                expanded={!!this.state.expanded[category]}
                onClick={() => this.toggleExpanded(category)}
              />
            ))}
            <tr />
            <tr />
          </tbody>
        </table>
        {this.props.data.timeseries && this.props.showGraph ? (
          <TimeseriesChart
            timeseries={this.props.data.timeseries}
            lineNames={[...this.props.data.categories.keys()]}
          />
        ) : null}
      </div>
    );
  }
}
