import { createApp } from 'vue'
import App from './App.vue'
import FloatingVue from 'floating-vue'

import router from './router/routes';
import 'floating-vue/dist/style.css'
import './assets/tailwind.css'
import { Icon } from '@iconify/vue/dist/offline';


FloatingVue.options.themes.tooltip.triggers = ['hover', 'click']

createApp(App)
    .use(router)
    .use(FloatingVue)
    .component("Icon", Icon)
    .mount('#app');
