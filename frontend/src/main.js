import { createApp } from 'vue'
import App from './App.vue'
import FloatingVue from 'floating-vue'

import router from './router/routes';
import 'floating-vue/dist/style.css'

FloatingVue.options.themes.tooltip.triggers = ['hover', 'click']

createApp(App)
    .use(router)
    .use(FloatingVue)
    .mount('#app');
