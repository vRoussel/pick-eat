import { createApp } from 'vue'
import App from './App.vue'
import FloatingVue from 'floating-vue'

import router from './router/routes';
import 'floating-vue/dist/style.css'
import './assets/tailwind.css'
import '@vueform/multiselect/themes/default.css';
import { Icon } from '@iconify/vue/dist/offline';
import { createPinia } from 'pinia'

import { useAuthStore } from '@/store/auth.js'

FloatingVue.options.themes.tooltip.triggers = ['hover', 'click']
const pinia = createPinia()
const authStore = useAuthStore(pinia);

authStore.load_account().finally(() => {
    createApp(App)
        .use(router)
        .use(FloatingVue)
        .use(pinia)
        .component("Icon", Icon)
        .mount('#app');
})
