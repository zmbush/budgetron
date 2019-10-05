import Budgetron from "components/Budgetron";
import MuiThemeProvider from "material-ui/styles/MuiThemeProvider";
import "normalize.css";
import * as React from "react";
import * as ReactDOM from "react-dom";
import {
  parseReports,
  parseTransactions,
  Report,
  Transaction,
} from "util/data";

const App = (props: {
  data: Report[];
  transactions: Map<string, Transaction>;
}) => (
    <MuiThemeProvider>
      <Budgetron {...props} />
    </MuiThemeProvider>
  );

let data: Report[] = [];
let transactions = new Map();
const render = () => {
  const root = document.getElementById("root");
  if (root) {
    ReactDOM.render(<App data={data} transactions={transactions} />, root);
  }
};

fetch("/__/data.json").then((reports) =>
  reports.json().then((json) => {
    data = parseReports(json);
    render();
  }),
);

fetch("/__/transactions.json").then((reports) =>
  reports.json().then((json) => {
    transactions = parseTransactions(json);
    render();
  }),
);
render();
