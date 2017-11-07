// @flow

import * as React from 'react';
import { Card, CardTitle, CardActions, CardText } from 'material-ui/Card';
import RaisedButton from 'material-ui/RaisedButton';
import Toggle from 'material-ui/Toggle';

import style from './style.scss';

type Props = {
  title: string,
  onClick?: null | (e: Event) => void,
  children: React.Node,
};

const Page = (props: Props) => (
  <Card className={style.page}>
    <CardTitle>
      { props.title } <Toggle />
    </CardTitle>
    { props.onClick
        ? <CardActions><RaisedButton onClick={props.onClick} label="Expand" /></CardActions>
        : null }
    <CardText>
      { props.children }
    </CardText>
  </Card>
);

Page.defaultProps = {
  onClick: null,
};

export default Page;
