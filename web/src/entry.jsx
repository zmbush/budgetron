import React from 'react';
import ReactDOM from 'react-dom';

class Money extends React.Component {
  render() {
    const color = (this.props.amount > 0) ? "black" : "red";
    const dollars = this.props.amount.toLocaleString("en-US", {style: "currency", currency: "USD"});
    return <span style={{color}}>{ dollars }</span>;
  }
}

class RollingBudget extends React.Component {
  renderBudgets() {
    return Object.entries(this.props.data.budgets).map(([person, budget]) => {
      return (<div>
        <b>{person}</b>: <Money amount={budget} />
      </div>);
    });
  }

  render() {
    return <div>
      <h1>{this.props.config.name}</h1>
      { this.renderBudgets() }
    </div>;
  }
}
class Cashflow extends React.Component {
  render() {
    return <div>{this.props.config.name}</div>;
  }
}
class Categories extends React.Component {
  render() {
    return <div>{this.props.config.name}</div>;
  }
}

class Budgetron extends React.Component {
  render_reports() {
    return Object.keys(this.props.data).map((report_id) => {
      let report = this.props.data[report_id];
      let ty = report.config.config.type;
      let Component = ReactDOM.div;
      if (ty == "RollingBudget") {
        Component = RollingBudget;
      } else if (ty == "Cashflow") {
        Component = Cashflow;
      } else if (ty == "Categories") {
        Component = Categories;
      }
      return <Component key={report_id} {...report} />;
    })
  }

  render() {
    return <div>{ this.render_reports() }</div>;
  }
}

fetch("/__/data.json").then(reports => reports.json().then(json => {
  let data = json;
  ReactDOM.render(<Budgetron data={data} />, document.getElementById('root'));
}))
ReactDOM.render(<Budgetron data={{}} />, document.getElementById('root'));
