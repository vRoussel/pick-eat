import './assets/main.css'

import { createApp } from 'vue'
import { createPinia } from 'pinia'
import { createHead } from '@unhead/vue'
import FloatingVue from 'floating-vue'
import 'floating-vue/dist/style.css'
import { Icon } from '@iconify/vue/dist/offline'
import icons from '@/utils/icons.js'
import '@vueform/multiselect/themes/default.css'

import App from './App.vue'
import router from './router/routes'
import { useAuthStore } from '@/store/auth.js'

FloatingVue.options.themes.tooltip.triggers = ['hover', 'click']
const pinia = createPinia()
const authStore = useAuthStore(pinia)
const head = createHead()

authStore.load_account().finally(() => {
    createApp(App)
        .use(router)
        .use(FloatingVue)
        .use(pinia)
        .use(head)
        .component('Icon', Icon)
        .provide("icons", icons)
        .mount('#app')
})
