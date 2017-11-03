import React from 'react';
import PropTypes from 'prop-types';
import style from './style.scss';

const Tag = props => <span className={style.tag}>{ props.text }</span>;

Tag.propTypes = {
  text: PropTypes.string.isRequired,
};

export default Tag;
