const HtmlWebpackPlugin = require("html-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const path = require("path");

module.exports = {
    entry: './static/main.js',
    output: {
        publicPath: "/"
    },
    mode: "development",
    devServer: {
        contentBase: './static',
        historyApiFallback: true,
    },
    module: {
        rules: [
            {
                test: /\.wasm$/,
                type: 'webassembly/async'
            },
            {
                test: /\.css$/i,
                use: ['style-loader', 'css-loader'],
            },
        ]
    },
    plugins: [
        new HtmlWebpackPlugin({
            template: "./static/index.html"
        }),
        new WasmPackPlugin({
            crateDirectory: path.resolve(__dirname, '../client'),
            outDir: path.resolve(__dirname, '../pkg')
        }),
    ],
    experiments: {
        asyncWebAssembly: true,
    },
}
