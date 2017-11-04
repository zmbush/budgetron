// @flow

import React from 'react';
import ReactDOM from 'react-dom';
import Budgetron from 'components/Budgetron';
import MuiThemeProvider from 'material-ui/styles/MuiThemeProvider';
import 'normalize.css';

const App = props => (
  <MuiThemeProvider>
    <Budgetron {...props} />
  </MuiThemeProvider>
);

let data = [];
let transactions = {};
const render = () => {
  const root = document.getElementById('root');
  if (root) {
    ReactDOM.render(
      <App data={data} transactions={transactions.transactions} />,
      root,
    );
  }
};

fetch('/__/data.json').then(reports => reports.json().then((json) => {
  data = json;
  render();
}));

fetch('/__/transactions.json').then(reports => reports.json().then((json) => {
  transactions = json;
  render();
}));
render();
