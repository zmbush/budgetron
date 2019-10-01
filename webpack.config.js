// @flow

const path = require("path");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const CopyWebpackPlugin = require("copy-webpack-plugin");
const webpack = require("webpack");

module.exports = {
  mode: "development",
  entry: ["babel-polyfill", "./web/src/entry.jsx"],
  output: {
    path: path.resolve("./web/static"),
    filename: "assets/js/[name]-[chunkhash].js",
    sourceMapFilename: "[file].map"
  },
  devtool: "source-map",
  resolve: {
    modules: [path.resolve("./web/src/"), "node_modules"],
    extensions: [".js", ".jsx", ".ts", ".tsx"]
  },
  plugins: [
    new CopyWebpackPlugin([
      {
        from: "./web/src/images/",
        to: "images"
      }
    ]),
    new HtmlWebpackPlugin({
      title: "Budgetron",
      inject: false,
      template: "./web/src/index.ejs"
    })
  ],
  optimization: {
    splitChunks: {
      cacheGroups: {
        vendor: {
          test: /[\\/]node_modules[\\/]/,
          priority: -10
        }
      }
    }
  },
  module: {
    rules: [
      {
        test: /\.jsx?$/,
        exclude: /(node_modules|bower_components)/,
        use: [
          {
            loader: "babel-loader",
            options: {
              presets: ["@babel/env", "@babel/preset-react", "@babel/flow"],
              plugins: [
                "@babel/plugin-proposal-class-properties",
                "flow-react-proptypes"
              ]
            }
          }
        ]
      },
      {
        test: /\.ts(x?)$/,
        exclude: /node_modules/,
        use: [
          {
            loader: "ts-loader"
          }
        ]
      },
      {
        test: /\.s?css$/,
        use: [
          {
            loader: "style-loader"
          },
          {
            loader: "css-modules-typescript-loader"
          },
          {
            loader: "css-loader",
            query: {
              modules: true,
              localIdentName: "[path][name]_[local]--[hash:base64:5]"
            }
          },
          {
            loader: "sass-loader"
          }
        ]
      }
    ]
  }
};
