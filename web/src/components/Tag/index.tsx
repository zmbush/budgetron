import * as React from "react";
import * as style from "./style.scss";

type Props = {
  text: string;
};

const Tag = (props: Props) => <span className={style.tag}>{props.text}</span>;

export default Tag;
