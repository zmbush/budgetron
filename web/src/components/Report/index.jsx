import React from 'react';
import ByTimeframe from 'components/ByTimeframe';
import Page from 'components/Page';
import style from './style.scss';

export default class Report extends React.Component {
  render() {
    const has_timeframes = Object.keys(this.props.config).some((k) => k.startsWith("by_"));
    return [
      <ByTimeframe {...this.props}
        key='year'
        title={ this.props.config.name }
        className={ style.page }
        timeframe='Year'
        data={this.props.data.by_year} />,
      <ByTimeframe {...this.props}
        key="quarter"
        title={ this.props.config.name }
        className={ style.page }
        timeframe='Quarter'
        data={this.props.data.by_quarter} />,
      <ByTimeframe {...this.props}
        key="month"
        title={ this.props.config.name }
        className={ style.page }
        timeframe='Month'
        data={this.props.data.by_month} />,
      has_timeframes ? null : (
        <Page
          key="basic"
          className={ style.page }
          title={ this.props.config.name }>
          <this.props.Component {...this.props} />
        </Page>
      )
    ];
  }
}
