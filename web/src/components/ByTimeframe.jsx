import React from 'react';

const monthNames = [
  'January', 'February', 'March', 'April', 'May', 'June', 'July', 'August',
  'September', 'October', 'November', 'December',
];

export default class ByTimeframe extends React.Component {
  constructor(props) {
    super(props);

    this.state = {
      expanded: false
    };
  }

  printDate(date) {
    if (this.props.timeframe == 'Year') {
      return date.getFullYear();
    } else if (this.props.timeframe == 'Quarter') {
      return `${date.getFullYear()} Q${(date.getMonth() / 3) + 1}`;
    } else if (this.props.timeframe == 'Month') {
      return `${monthNames[date.getMonth()]} ${date.getFullYear()}`;
    } else {
      return date.toLocaleDateString();
    }

  }

  toggleExpanded = () => {
    this.setState({
      expanded: !this.state.expanded
    })
  }

  render() {
    if (this.props.data) {
      let timeframes = Object.entries(this.props.data).map(([date_str, content]) => {
        return [new Date(date_str), content]
      }).sort((a, b) => a[0] - b[0]).reverse();
      if (!this.state.expanded) {
        timeframes = timeframes.slice(0, 4);
}

      return <div>
        <h2>By {this.props.timeframe}</h2>
        <h3><a onClick={ this.toggleExpanded }>{ this.state.expanded ? 'Collapse' : 'Expand' }</a></h3>
        { timeframes.map(([date, content]) => {
          return <div key={date}>
            <b>{ this.printDate(date) }</b> <this.props.Component data={ content } config={ this.props.config }/>
          </div>;
        }) }
      </div>
    }
    return null;
  }
}
