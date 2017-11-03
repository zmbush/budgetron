import React from 'react';
import ReactDOM from 'react-dom';
import Budgetron from 'components/Budgetron';
import 'normalize.css';

let data = [];
let transactions = {};
const render = () => {
  ReactDOM.render(
    <Budgetron data={data} transactions={transactions} />,
    document.getElementById('root'),
  );
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
