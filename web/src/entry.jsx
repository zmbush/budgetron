import React from 'react';
import ReactDOM from 'react-dom';
import PropTypes from 'prop-types';

const Money = (props) => {
  let amount = parseFloat(props.amount);
  if (props.invert) { amount = -amount; }
  const color = (amount > 0) ? 'black' : 'red';
  const dollars = amount.toLocaleString('en-US', {
    style: 'currency',
    currency: 'USD',
  });
  return <span style={{ color }}>{ dollars }</span>;
};

Money.propTypes = {
  invert: PropTypes.bool,
  amount: PropTypes.string.isRequired,
};

Money.defaultProps = {
  invert: false,
};

class RollingBudget extends React.Component {
  propTypes = {
    data: PropTypes.object, // eslint-disable-line react/forbid-prop-types
  }

  renderBudgets() {
    return Object.entries(this.props.data.budgets).map(([person, budget]) => (
      <div key={person}>
        <b>{person}</b>: <Money amount={budget} />
      </div>
    ));
  }

  renderTransactions() {
    return <table>
      <tbody>{
        this.props.data.transactions
          .map((tid) => this.props.transactions[tid])
          .sort((a,b) => a.amount - b.amount)
          .reverse()
          .map((transaction) => (
          <tr>
            <td>{ transaction.person }</td>
            <td>{ transaction.description }</td>
            <td>{ transaction.amount }</td>
            <td>{ transaction.transaction_type }</td>
            <td>{ transaction.original_description }</td>
          </tr>
        ))
      }</tbody>
    </table>;
  }

  render() {
    return (
      <div>
        { this.renderBudgets() }
        { this.renderTransactions() }
      </div>
    );
  }
}

const monthNames = [
  'January', 'February', 'March', 'April', 'May', 'June', 'July', 'August',
  'September', 'October', 'November', 'December',
];

class ByTimeframe extends React.Component {
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

class Cashflow extends React.Component {
  render() {
    let {
      credit,
      debit
    } = this.props.data;
    return <span>
      <Money amount={credit} /> - <Money amount={debit} /> = <Money amount={credit - debit} />
    </span>;
  }
}

class Report extends React.Component {
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
class Categories extends React.Component {
  render() {
    let reverse = this.props.config.only_type != 'Debit';
    let categories = Object.entries(this.props.data).sort((a, b) => a[1] - b[1]);
    if (reverse) {categories = categories.reverse();}

    return <table><tbody>
        { categories.map(([category, amount]) => {
          return <tr key={category}>
            <td><b>{ category }</b></td><td><Money amount={amount} invert={ !reverse }/></td>
          </tr>;
        }) }
        <br/><br />
    </tbody></table>;
  }
}

class Budgetron extends React.Component {
  render_reports() {
    return Object.keys(this.props.data).map((report_id) => {
      let report = this.props.data[report_id];
      let ty = report.config.config.type;
      let Component = ReactDOM.div;
      if (ty == 'RollingBudget') {Component = RollingBudget;} else if (ty == 'Cashflow') {Component = Cashflow;} else if (ty == 'Categories') {Component = Categories;}

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

let data = {};
let transactions = {};
let render = () => {
  ReactDOM.render(<Budgetron data={data} transactions={transactions}/>, document.getElementById('root'));
}

fetch('/__/data.json').then(reports => reports.json().then(json => {
  data = json;
  render();
}));
fetch('/__/transactions.json').then(reports => reports.json().then(json => {
  transactions = json;
  render();
}));
render();
