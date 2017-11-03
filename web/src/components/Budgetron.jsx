import Cashflow from 'components/Cashflow';
import Categories from 'components/Categories';
import ReactDOM from 'react-dom';
import React from 'react';
import RollingBudget from 'components/RollingBudget';
import ByTimeframe from 'components/ByTimeframe';
import PropTypes from 'prop-types';

const TimeframeReports = props => (
  <div>
    { props.data.map(({ data, report, key }) => {
      const timeframeKey = `by_${props.timeframe.toLocaleLowerCase()}`;
      if (report[timeframeKey]) {
        const ty = report.config.type;
        let Component = ReactDOM.div;
        if (ty === 'RollingBudget') {
          Component = RollingBudget;
        } else if (ty === 'Cashflow') {
          Component = Cashflow;
        } else if (ty === 'Categories') {
          Component = Categories;
        }
        return (
          <ByTimeframe
            key={key}
            title={report.name}
            Component={Component}
            timeframe={props.timeframe}
            transactions={props.transactions}
            config={report.config}
            data={data[timeframeKey]}
          />
        );
      }
      return null;
    }) }
  </div>
);

TimeframeReports.propTypes = {
  data: PropTypes.arrayOf(PropTypes.shape({})).isRequired,
  timeframe: PropTypes.oneOf(['Year', 'Quarter', 'Month']).isRequired,
};

export default class Budgetron extends React.Component {
  static defaultProps = {
    data: [],
    transactions: {},
  };

  static propTypes = {
    data: PropTypes.arrayOf(PropTypes.shape({})),
    transactions: PropTypes.shape({}),
  };

  render() {
    return (
      <div>
        <TimeframeReports timeframe="Month" {...this.props} />
        <TimeframeReports timeframe="Quarter" {...this.props} />
        <TimeframeReports timeframe="Year" {...this.props} />
      </div>
    );
  }
}
