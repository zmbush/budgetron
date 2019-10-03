import * as React from "react";
import { Card, CardTitle, CardText } from "material-ui/Card";
import Toggle from "material-ui/Toggle";

import * as style from "./style.scss";

type Props = {
  title: string;
  onClick?: (e: React.MouseEvent) => void;
  expanded?: boolean;
  children: React.ReactNode;
  className?: string;
};

type State = {
  expanded: boolean;
};

class Page extends React.Component<Props, State> {
  static defaultProps = {
    onClick: null,
    expanded: null
  };

  constructor(props: Props) {
    super(props);

    this.state = {
      expanded: false
    };
  }

  render() {
    return (
      <Card
        className={style.page}
        expanded={this.state.expanded}
        expandable
        onExpandChange={expanded => this.setState({ expanded })}
      >
        <CardTitle title={this.props.title} actAsExpander />
        {this.props.onClick ? (
          <CardText expandable>
            <Toggle
              onToggle={this.props.onClick}
              toggled={this.props.expanded}
              labelPosition="right"
              label="Expand"
            />
          </CardText>
        ) : null}
        <CardText expandable>{this.props.children}</CardText>
      </Card>
    );
  }
}

export default Page;
