import React from 'react';
import PropTypes from 'prop-types';
import style from './style.scss';

const Page = props => (
  <div className={props.className}>
    { props.title ? <h2 className={style.title}>{ props.title }</h2> : null }
    <button className={style.button} onClick={props.onClick}>*</button>
    { props.children }
  </div>
);

Page.propTypes = {
  title: PropTypes.string,
  className: PropTypes.string,
  onClick: PropTypes.func,
  children: PropTypes.node.isRequired,
};

Page.defaultProps = {
  title: null,
  className: null,
  onClick: null,
};

export default Page;
