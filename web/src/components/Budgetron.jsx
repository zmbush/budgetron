import Cashflow from 'components/Cashflow';
import Categories from 'components/Categories';
import ReactDOM from 'react-dom';
import React from 'react';
import Report from 'components/Report';
import RollingBudget from 'components/RollingBudget';

export default class Budgetron extends React.Component {
  render_reports() {
    return Object.keys(this.props.data).map((report_id) => {
      let report = this.props.data[report_id];
      let ty = report.config.config.type;
      let Component = ReactDOM.div;
      if (ty == 'RollingBudget') {
        Component = RollingBudget;
      } else if (ty == 'Cashflow') {
        Component = Cashflow;
      } else if (ty == 'Categories') {
        Component = Categories;
      }

      return <Report
        Component={ Component }
        key={report_id}
        transactions={ this.props.transactions }
        {...report} />;
    })
  }

  render() {
    return <div>{ this.render_reports() }</div>;
  }
}
