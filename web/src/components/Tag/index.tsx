import * as React from "react";
import * as style from "./style.scss";

interface IProps {
  text: string;
}

const Tag = (props: IProps) => <span className={style.tag}>{props.text}</span>;

export default Tag;
