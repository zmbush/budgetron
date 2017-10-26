import React from 'react';
import ByTimeframe from 'components/ByTimeframe';

export default class Report extends React.Component {
  render() {
    return (
      <div>
        <h1>{this.props.config.name}</h1>
        <ByTimeframe {...this.props} timeframe='Year' data={this.props.data.by_year} />
        <ByTimeframe {...this.props} timeframe='Quarter' data={this.props.data.by_quarter} />
        <ByTimeframe {...this.props} timeframe='Month' data={this.props.data.by_month} />
        { (!this.props.config.by_year && !this.props.config.by_quarter && !this.props.config.by_month) ?
            <this.props.Component {...this.props} /> : null }
          </div>
    );
  }
}
