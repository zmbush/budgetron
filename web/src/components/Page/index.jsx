// @flow

import * as React from 'react';
import { Card, CardTitle, CardActions, CardText } from 'material-ui/Card';
import { FlatButton } from 'material-ui/FlatButton';

type Props = {
  title?: string,
  onClick?: (e: Event) => void,
  children: React.Node,
};

const Page = (props: Props) => (
  <Card>
    { props.title
        ? <CardTitle title={props.title} />
        : null }
    { props.onClick
        ? <CardActions><FlatButton onClick={props.onClick} label="Expand" /></CardActions>
        : null }
    <CardText>
      { props.children }
    </CardText>
  </Card>
);

Page.defaultProps = {
  title: null,
  onClick: null,
};

export default Page;
