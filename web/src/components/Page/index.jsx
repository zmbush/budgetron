import React from 'react';

export default class Page extends React.Component {
  render() {
    return <div className={ this.props.className } onClick={ this.props.onClick }>
      { this.props.title ? <h2>{ this.props.title }</h2> : null }
      { this.props.children }
    </div>
  }
}
