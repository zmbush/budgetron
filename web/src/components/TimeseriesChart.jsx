import React from 'react';
import {
  LineChart,
  Line,
  XAxis,
  YAxis,
  ResponsiveContainer,
  ReferenceLine,
  CartesianGrid,
  Tooltip,
  Legend,
} from 'recharts';
import { Timeseries } from 'util/data';
import moment from 'moment';
import Money from 'components/Money';
import * as d3 from 'd3-scale';

type Props<T> = {
  timeseries: Timeseries<T>,
  className: ?string,
  lineNames: string[],

  formatDate?: (timestamp: number) => string,
  gridDasharray?: string,
  height?: string | number,
  lineType?: string,
  width?: string | number,
}

export default function TimeseriesChart<T>(props: Props<T>) {
  const {
    className,
    formatDate,
    gridDasharray,
    height,
    lineNames,
    lineType,
    timeseries,
    width,
  } = props;
  const { data } = timeseries;
  let category = d3.schemeCategory10;
  if (lineNames.length > 10) {
    category = d3.schemeCategory20;
  }
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
          formatter={a => <Money amount={a} />}
          labelFormatter={formatDate}
        />
        <Legend />
        { lineNames.map((name, i) => (
          <Line
            type={lineType}
            key={name}
            dataKey={name}
            stroke={category[i % category.length]}
            dot={data.length > 200 ? false : undefined}
          />
        )) }
        <ReferenceLine y={0} stroke="red" />
      </LineChart>
    </ResponsiveContainer>
  );
}

TimeseriesChart.defaultProps = {
  formatDate: ts => moment(ts).format('MMM Do YYYY'),
  gridDasharray: '3 3',
  height: 200,
  lineType: 'monotone',
  width: '95%',
};
