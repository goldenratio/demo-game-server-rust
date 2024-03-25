const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const { CleanWebpackPlugin } = require('clean-webpack-plugin');
const CopyPlugin = require('copy-webpack-plugin');

module.exports = {
  target: 'web',
  mode: 'development',
  devtool: 'source-map',
  entry: {
    main: './src/index.ts'
  },
  devServer: {
    static: './bin',
    client: {
      overlay: false
    }
  },
  module: {
    rules: [
      {
        test: /\.tsx?$/,
        use: 'ts-loader',
        exclude: /node_modules/
      },
      {
        test: /\.html$/,
        use: 'html-loader',
        exclude: /node_modules/
      },
      {
        test: /\.(png|jpg|gif)$/,
        exclude: /node_modules/,
        loader: 'file-loader'
      },
      {
        test: /\.css$/,
        use: [
          {loader: "style-loader"},
          {loader: "css-loader"}
        ]
      }
    ]
  },
  resolve: {
    extensions: ['.tsx', '.ts', '.js']
  },
  plugins: [
    new CleanWebpackPlugin(),
    new HtmlWebpackPlugin({
      title: 'super-ecs sandbox'
    }),
    new CopyPlugin({
      patterns: [
        { from: './src/assets', to: './assets' }
      ]
    })
  ],
  output: {
    filename: '[name].bundle.js',
    path: path.resolve(__dirname, 'bin')
  }
};
