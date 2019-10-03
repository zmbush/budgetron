import * as React from "react";
import * as ReactDOM from "react-dom";
import Budgetron from "components/Budgetron";
import MuiThemeProvider from "material-ui/styles/MuiThemeProvider";
import {
  parseReports,
  parseTransactions,
  Report,
  Transaction
} from "util/data";
import "normalize.css";

const App = (props: {
  data: Array<Report>;
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

fetch("/__/data.json").then(reports =>
  reports.json().then(json => {
    data = parseReports(json);
    render();
  })
);

fetch("/__/transactions.json").then(reports =>
  reports.json().then(json => {
    transactions = parseTransactions(json.transactions);
    render();
  })
);
render();
