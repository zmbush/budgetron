// @flow

import * as React from 'react';
import { Card, CardTitle, CardText } from 'material-ui/Card';
import Toggle from 'material-ui/Toggle';

import style from './style.scss';

type Props = {
  title: string,
  onClick?: ?(e: Event) => void,
  expanded?: ?bool,
  children: React.Node,
};

const Page = (props: Props) => (
  <Card className={style.page}>
    <CardTitle title={props.title} />
    { props.onClick ? (
      <CardText>
        <Toggle
          onToggle={props.onClick}
          toggled={props.expanded}
          labelPosition="right"
          label="Expand"
        />
      </CardText>
    ) : null }
    <CardText>
      { props.children }
    </CardText>
  </Card>
);

Page.defaultProps = {
  onClick: null,
  expanded: null,
};

export default Page;
