import React from 'react';
import PropTypes from 'prop-types';
import Page from 'components/Page';

const monthNames = [
  'January', 'February', 'March', 'April', 'May', 'June', 'July', 'August',
  'September', 'October', 'November', 'December',
];

export default class ByTimeframe extends React.Component {
  static propTypes = {
    timeframe: PropTypes.oneOf(['Year', 'Quarter', 'Month']).isRequired,
    data: PropTypes.shape({}).isRequired,
    title: PropTypes.string.isRequired,
    transactions: PropTypes.shape({}).isRequired,
    report: PropTypes.shape({
      name: PropTypes.string,
    }).isRequired,
    className: PropTypes.string,
    count: PropTypes.number,
  };

  static defaultProps = {
    className: null,
    count: 1,
  };

  constructor(props) {
    super(props);

    this.state = {
      expanded: false,
    };
  }

  printDate(date) {
    if (this.props.timeframe === 'Year') {
      return date.getFullYear();
    } else if (this.props.timeframe === 'Quarter') {
      return `${date.getFullYear()} Q${(date.getMonth() / 3) + 1}`;
    } else if (this.props.timeframe === 'Month') {
      return `${monthNames[date.getMonth()]} ${date.getFullYear()}`;
    }
    return date.toLocaleDateString();
  }

  toggleExpanded = () => {
    this.setState({ expanded: !this.state.expanded });
  }

  render() {
    if (this.props.data) {
      let timeframes = Object.entries(this.props.data)
        .map(([dateStr, content]) => [new Date(dateStr), content])
        .sort((a, b) => a[0] - b[0]).reverse();
      if (!this.state.expanded) {
        timeframes = timeframes.slice(0, this.props.count);
      }

      const title = `${this.props.title} By ${this.props.timeframe}`;
      return (
        <Page className={this.props.className} title={title} onClick={this.toggleExpanded}>
          { timeframes.map(([date, content]) => (
            <div key={date}>
              <b>{ this.printDate(date) }</b> <this.props.Component
                data={content}
                transactions={this.props.transactions}
                report={this.props.report}
              />
            </div>
          )) }
        </Page>
      );
    }
    return null;
  }
}
