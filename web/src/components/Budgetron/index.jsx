import Cashflow from 'components/Cashflow';
import Categories from 'components/Categories';
import ReactDOM from 'react-dom';
import React from 'react';
import RollingBudget from 'components/RollingBudget';
import ByTimeframe from 'components/ByTimeframe';
import PropTypes from 'prop-types';
import AirbnbPropTypes from 'airbnb-prop-types';
import BudgetronTypes from 'budgetron-types';
import Page from 'components/Page';
import style from './style.scss';

const componentConfig = (type) => {
  const config = {
    Component: ReactDOM.div,
    count: 1,
  };
  if (type === 'RollingBudget') {
    config.Component = RollingBudget;
  } else if (type === 'Cashflow') {
    config.Component = Cashflow;
    config.count = 4;
  } else if (type === 'Categories') {
    config.Component = Categories;
  }

  return config;
};

const TimeframeReports = (props) => {
  const timeframeKey = `by_${props.timeframe.toLocaleLowerCase()}`;
  const reports = props.data.filter(({ report }) => report[timeframeKey]);
  return (
    <div className={style.reports}>
      { reports.map(({ data, report, key }, i) => [
        (i > 0) ? <div key="spacer" className={style.spacer} /> : null,
        <ByTimeframe
          key={key}
          title={report.name}
          timeframe={props.timeframe}
          transactions={props.transactions}
          report={report}
          data={data[timeframeKey]}
          className={style.report}
          {...componentConfig(report.config.type)}
        />,
      ]) }
    </div>
  );
};

TimeframeReports.propTypes = {
  data: PropTypes.arrayOf(BudgetronTypes.Report).isRequired,
  timeframe: PropTypes.oneOf(['Year', 'Quarter', 'Month']).isRequired,
  transactions: AirbnbPropTypes.valuesOf(BudgetronTypes.Transaction).isRequired,
};

const SimpleReports = props => (
  <div className={style.reports}>
    { props.data.map(({ data, report, key }) => {
      const hasTimeframes = Object.keys(data).some(k => k.startsWith('by_'));
      const cfg = componentConfig(report.config.type);
      if (hasTimeframes) return null;
      return (
        <Page
          key={key}
          className={style.report}
          title={report.name}
        >
          <cfg.Component {...props} count={cfg.count} data={data} report={report} />
        </Page>
      );
    }) }
  </div>
);

SimpleReports.propTypes = {
  data: PropTypes.arrayOf(BudgetronTypes.Report).isRequired,
};

export default class Budgetron extends React.Component {
  static defaultProps = {
    data: [],
    transactions: {},
  };

  static propTypes = {
    data: PropTypes.arrayOf(BudgetronTypes.Report).isRequired,
    transactions: AirbnbPropTypes.valuesOf(BudgetronTypes.Transaction).isRequired,
  };

  render() {
    return (
      <div>
        <SimpleReports {...this.props} />
        <TimeframeReports timeframe="Month" {...this.props} />
        <TimeframeReports timeframe="Quarter" {...this.props} />
        <TimeframeReports timeframe="Year" {...this.props} />
      </div>
    );
  }
}
