import { Card, CardText, CardTitle } from "material-ui/Card";
import Toggle from "material-ui/Toggle";
import * as React from "react";

import * as style from "./style.scss";

interface IProps {
  title: string;
  onClick?: (e: React.MouseEvent) => void;
  expanded?: boolean;
  children: React.ReactNode;
  className?: string;
}

interface IState {
  expanded: boolean;
}

class Page extends React.Component<IProps, IState> {
  public static defaultProps = {
    expanded: null,
    onClick: null,
  };

  constructor(props: IProps) {
    super(props);

    this.state = {
      expanded: false,
    };
  }

  public render() {
    return (
      <Card
        className={style.page}
        expanded={this.state.expanded}
        expandable
        onExpandChange={(expanded) => this.setState({ expanded })}
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
