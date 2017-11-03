import React from 'react';
import PropTypes from 'prop-types';

const Page = props => (
  <div className={props.className}>
    <button onClick={props.onClick}>Click</button>
    { props.title ? <h2>{ props.title }</h2> : null }
    { props.children }
  </div>
);

Page.propTypes = {
  title: PropTypes.string,
  className: PropTypes.string,
  onClick: PropTypes.func,
  children: PropTypes.element.isRequired,
};

Page.defaultProps = {
  title: null,
  className: null,
  onClick: null,
};

export default Page;
