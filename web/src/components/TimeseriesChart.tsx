import * as React from "react";
import {
  LineChart,
  Line,
  XAxis,
  YAxis,
  ResponsiveContainer,
  ReferenceLine,
  CartesianGrid,
  Tooltip,
  Legend
} from "recharts";
import { Timeseries } from "util/data";
import * as moment from "moment";
import Money from "components/Money";
import * as d3 from "d3-scale-chromatic";

type Props<T> = {
  timeseries: Timeseries<T>;
  className?: string;
  lineNames: (string | null)[];

  formatDate?: (timestamp: number) => string;
  gridDasharray?: string;
  height?: string | number;
  lineType?:
    | "monotone"
    | "linear"
    | "basis"
    | "basisClosed"
    | "basisOpen"
    | "linearClosed"
    | "natural"
    | "monotoneX"
    | "monotoneY"
    | "step"
    | "stepBefore"
    | "stepAfter";
  width?: string | number;
};

export default function TimeseriesChart<T>(props: Props<T>) {
  const {
    className,
    formatDate,
    gridDasharray,
    height,
    lineNames,
    lineType,
    timeseries,
    width
  } = props;
  const { data } = timeseries;
  let category = d3.schemeCategory10;
  if (data.length === 0) {
    return <div>No Data</div>;
  }

  return (
    <ResponsiveContainer className={className} width={width} height={height}>
      <LineChart data={data}>
        <XAxis
          dataKey="date"
          name="Time"
          domain={[data[0].date, data[data.length - 1].date]}
          tickFormatter={formatDate}
          type="number"
        />
        <YAxis />
        <CartesianGrid strokeDasharray={gridDasharray} />
        <Tooltip
          formatter={(a: string) => <Money amount={a} />}
          labelFormatter={formatDate}
        />
        <Legend />
        {lineNames.map((name, i) => {
          if (name) {
            return (
              <Line
                type={lineType}
                key={name}
                dataKey={name}
                stroke={category[i % category.length]}
                dot={data.length > 200 ? false : undefined}
              />
            );
          } else {
            return null;
          }
        })}
        <ReferenceLine y={0} stroke="red" />
      </LineChart>
    </ResponsiveContainer>
  );
}

TimeseriesChart.defaultProps = {
  formatDate: (ts: string) => moment(ts).format("MMM Do YYYY"),
  gridDasharray: "3 3",
  height: 200,
  lineType: "monotone",
  width: "95%"
};
