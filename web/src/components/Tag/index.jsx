import React from 'react';
import style from './style.scss';

export default class Tag extends React.Component {
  render() {
    return <span className={ style.tag }>{ this.props.text }</span>;
  }
}
