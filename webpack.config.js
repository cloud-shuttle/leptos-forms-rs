const path = require("path");

module.exports = {
  mode: "production",
  entry: "./leptos-forms-rs/pkg/leptos_forms_rs.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "leptos-forms-rs.bundle.js",
    library: "LeptosForms",
    libraryTarget: "umd",
    globalObject: "this",
  },
  module: {
    rules: [
      {
        test: /\.wasm$/,
        type: "webassembly/async",
      },
    ],
  },
  experiments: {
    asyncWebAssembly: true,
  },
  optimization: {
    minimize: true,
    usedExports: true,
    sideEffects: false,
    splitChunks: {
      chunks: "all",
      cacheGroups: {
        wasm: {
          test: /\.wasm$/,
          name: "wasm",
          chunks: "all",
        },
      },
    },
  },
  resolve: {
    extensions: [".js", ".wasm"],
  },
  performance: {
    hints: "warning",
    maxEntrypointSize: 512000,
    maxAssetSize: 512000,
  },
};
