module.exports = {
    chainWebpack: config => {
    config.module
      .rule('vue')
      .use('vue-loader')
      .tap(options => {
        options.compilerOptions = {
          ...options.compilerOptions,
          isCustomElement: tag => tag.startsWith('ion-')
        }
        return options
      })
  },
    devServer: {
    host: '0.0.0.0',
    hot: true,
  }
};
