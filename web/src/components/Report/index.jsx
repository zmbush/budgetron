import React from 'react';
import PropTypes from 'prop-types';
import ByTimeframe from 'components/ByTimeframe';
import Page from 'components/Page';
import style from './style.scss';

export default class Report extends React.Component {
  static propTypes = {
    data: PropTypes.shape({
      by_year: PropTypes.shape({}),
      by_quarter: PropTypes.shape({}),
      by_month: PropTypes.shape({}),
    }).isRequired,
    config: PropTypes.shape({
      name: PropTypes.string,
    }).isRequired,
  };

  render() {
    const hasTimeframes = Object.keys(this.props.data).some(k => k.startsWith('by_'));
    return [
      <ByTimeframe
        {...this.props}
        key="year"
        title={this.props.config.name}
        className={style.page}
        timeframe="Year"
        data={this.props.data.by_year}
      />,
      <ByTimeframe
        {...this.props}
        key="quarter"
        title={this.props.config.name}
        className={style.page}
        timeframe="Quarter"
        data={this.props.data.by_quarter}
      />,
      <ByTimeframe
        {...this.props}
        key="month"
        title={this.props.config.name}
        className={style.page}
        timeframe="Month"
        data={this.props.data.by_month}
      />,
      hasTimeframes ? null : (
        <Page
          key="basic"
          className={style.page}
          title={this.props.config.name}
        >
          <this.props.Component {...this.props} />
        </Page>
      ),
    ];
  }
}
