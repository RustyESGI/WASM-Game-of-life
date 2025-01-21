const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');

module.exports = {
  entry: "./bootstrap.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "bootstrap.js",
  },
  mode: "development",
  plugins: [
    new HtmlWebpackPlugin({
      template: 'index.html'
    })
  ],
  experiments: {
    asyncWebAssembly: true
  }
};