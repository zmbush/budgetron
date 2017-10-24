// @flow

var path = require('path');
var HtmlWebpackPlugin = require('html-webpack-plugin');
var CopyWebpackPlugin = require('copy-webpack-plugin');
var webpack = require('webpack');

module.exports = {
  entry: [
    'babel-polyfill', "./src/entry.jsx"
  ],
  output: {
    path: path.resolve('./static'),
    filename: 'assets/js/[name]-[chunkhash].js',
    sourceMapFilename: '[file].map'
  },
  devtool: 'source-map',
  resolve: {
    modules: [
      path.resolve('./src/'),
      'node_modules'
    ],
    extensions: ['.js', '.jsx']
  },
  plugins: [
    new CopyWebpackPlugin([
      {
        from: './src/images/',
        to: 'images'
      }
    ]),
    new HtmlWebpackPlugin({title: 'Budgetron', inject: false, template: './src/index.ejs'}),
    new webpack
      .optimize
      .CommonsChunkPlugin({
        name: 'vendor',
        minChunks: function(module) {
          return module.context && module
            .context
            .indexOf('node_modules') !== -1;
        }
      }),
    new webpack
      .optimize
      .CommonsChunkPlugin({name: 'manifest'})
  ],
  module: {
    rules: [
      {
        test: /\.jsx?$/,
        exclude: /(node_modules|bower_components)/,
        use: {
          loader: 'babel-loader',
          options: {
            presets: [
              'env', 'react', 'flow'
            ],
            plugins: ['transform-object-rest-spread', 'transform-class-properties']
          }
        }
      }, {
        test: /\.scss$/,
        use: ['style-loader', 'css-loader', 'sass-loader']
      }
    ]
  }
};
