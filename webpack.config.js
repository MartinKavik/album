const path = require("path");
const CleanWebpackPlugin = require("clean-webpack-plugin");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const MiniCssExtractPlugin = require("mini-css-extract-plugin");
const fs = require('fs');
const Dotenv = require('dotenv-webpack');

loadOutput = (environment) => {
    const filename = environment.production ? "scripts/[name].[hash].min.js" : "scripts/[name].js";
    const chunkFilename = environment.production ? "[name].[hash].min.js" : "[name].js";
    return {
        path: path.resolve(__dirname, "dist"),
        filename,
        chunkFilename,
        publicPath: "/"
    };
};

loadPlugins = (environment) => {

    let wasmPackArgs = "--no-typescript";
    if (environment.production) wasmPackArgs += " --release";

    return [
        new HtmlWebpackPlugin({
			template: "./front/index.html",
			favicon: "./front/assets/image/camera.png",
            chunks: ["app"],
            hash: true
        }),
        new WasmPackPlugin({
            crateDirectory: path.resolve(__dirname, `./`),
            extraArgs: wasmPackArgs
        }),
        new MiniCssExtractPlugin({
            // Options similar to the same options in webpackOptions.output
            // both options are optional
            filename: "[name].css",
            chunkFilename: "[id].css"
        }),
		new CleanWebpackPlugin(),
		new Dotenv()
    ];
};

getImages = () => {
   const dir = "./front/assets/image/";
   return fs.readdirSync(dir).map(x => x = dir + x);
}

module.exports = environment => {

    const mode = environment.production ? "production" : "development";
    const entry = { 
        app: "./front/app.js", 
        css: [
            "./front/assets/style/style.css",
            "./node_modules/spectre.css/dist/spectre.min.css",
            "./node_modules/spectre.css/dist/spectre-icons.css"
        ],
        img: getImages()
    };
    const output = loadOutput(environment);
    const plugins = loadPlugins(environment);

    const module = {
        rules: [{
            test: /\.css$/,
            use: [
                MiniCssExtractPlugin.loader,
                "css-loader"
            ]
        },
        {
            test: /\.(png|jpe?g|gif)$/,
            use: [{
                loader: 'file-loader',
                options: {
                    name: '[path][name].[ext]',
                },
            }]
        }
    ]}

    //Base Config
    const config = { mode, entry, output, plugins, module };

    //Additional Production Config
    if (environment.production) {
        config.devtool = "source-map";
    }

    //Additional Development Config
    else {
        config.devServer = {
            contentBase: path.join(__dirname, "dist"),
            open: true,
            overlay: { errors: true, warnings: false },
            port: 5000,
            watchContentBase: true
        };
    }

    return config;
};
