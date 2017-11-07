// @flow

import React from 'react';
import ReactDOM from 'react-dom';
import Budgetron from 'components/Budgetron';
import MuiThemeProvider from 'material-ui/styles/MuiThemeProvider';
import { parseReports, parseTransactions, Report } from 'util/data';
import 'normalize.css';

const App = props => (
  <MuiThemeProvider>
    <Budgetron {...props} />
  </MuiThemeProvider>
);

let data: Report[] = [];
let transactions = new Map();
const render = () => {
  const root = document.getElementById('root');
  if (root) {
    ReactDOM.render(
      <App data={data} transactions={transactions} />,
      root,
    );
  }
};

fetch('/__/data.json').then(reports => reports.json().then((json) => {
  data = parseReports(json);
  render();
}));

fetch('/__/transactions.json').then(reports => reports.json().then((json) => {
  transactions = parseTransactions(json.transactions);
  render();
}));
render();
