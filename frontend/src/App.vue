<script setup>
import { inject, ref, onMounted } from 'vue'
import { useHead } from '@unhead/vue'
import { useSchemaOrg, defineWebSite, defineOrganization } from '@unhead/schema-org'
import { useRoute } from 'vue-router'

import ThemeToggle from '@/components/ThemeToggle.vue'
import GroceryListModal from '@/components/GroceryListModal.vue'
import Toast from '@/components/Toast.vue'

import { useCartStore } from '@/store/cart.js'
import { useFoodStore } from '@/store/food.js'
import { useNotifStore } from '@/store/notif.js'
import { useAuthStore } from '@/store/auth.js'

import pickeat_logo_light from '@/assets/pickeat_light.png'
import pickeat_logo_dark from '@/assets/pickeat_dark.png'

const WEBSITE_PROTO = window.location.protocol
const WEBSITE_HOST = window.location.hostname
const WEBSITE_URL = `${WEBSITE_PROTO}//${WEBSITE_HOST}`


const cartStore = useCartStore()
const foodStore = useFoodStore()
const notifStore = useNotifStore()
const authStore = useAuthStore()

useHead({
    titleTemplate: (title) => !title ? 'Pickeat' : `${title} - Pickeat`,
})

useHead({
    templateParams: {
        schemaOrg: {
            host: WEBSITE_URL,
            path: useRoute().path,
            inLanguage: 'fr-FR'
        }
    }
})

useSchemaOrg([
    defineOrganization({
        name: 'Pickeat',
        logo: () => pickeat_logo.value,
    }),
    defineWebSite({
        name: 'Pickeat',
    })
])

const icons = inject('icons')

const pickeat_logo = ref(null)
const dropdown_main_opened = ref(false)
const grocery_list_modal_el = ref(null);

onMounted(() => {
    foodStore.fetchAll().then(() => {
        cartStore.restore()
    })
    window.setInterval(
        () => {
            foodStore.fetchAll()
        },
        5 * 60 * 1000,
    )
})

function set_logo(theme) {
    if (theme == 'light')
        pickeat_logo.value = pickeat_logo_light
    else
        pickeat_logo.value = pickeat_logo_dark
}

function close_dropdown_if_opened() {
    if (dropdown_main_opened.value) {
        setTimeout(function () {
            document.activeElement.blur()
        }, 0)
    }
    dropdown_main_opened.value = !dropdown_main_opened.value
}
</script>

<template>
    <nav class="navbar bg-base-100 border-b border-primary">
        <div class="navbar-start">
            <div class="dropdown sm:hidden" @click="close_dropdown_if_opened">
                <label tabindex="0" class="btn btn-ghost">
                    <Icon class="icon text-2xl" :icon="icons.menu" />
                </label>
                <ul tabindex="0"
                    class="z-10 menu menu-compact dropdown-content mt-3 p-2 shadow bg-base-100 rounded-box w-52">
                    <li>
                        <router-link to="/recipes"> Recettes </router-link>
                    </li>
                    <li>
                        <router-link to="/new-recipe">
                            Nouvelle recette
                        </router-link>
                    </li>
                </ul>
            </div>
            <router-link class="xs:min-w-[150px] sm:min-w-[200px]" to="/recipes">
                <img :src="pickeat_logo" class="aspect-[200/84]" width="150" sm:width="200" alt="Logo pickeat" />
            </router-link>
            <ul class="shrink-0 grow menu menu-horizontal p-2 rounded-box hidden sm:flex sm:space-x-2">
                <li class="shrink-0">
                    <router-link to="/recipes"> Recettes </router-link>
                </li>
                <li class="shrink-0">
                    <router-link to="/new-recipe"> Nouvelle recette </router-link>
                </li>
            </ul>
        </div>
        <div class="navbar-end space-x-3">
            <button class="indicator" type="button" @click="grocery_list_modal_el.open()"
                aria-label="Ouvrir la liste de course">
                <Icon class="icon text-2xl sm:text-3xl md:text-4xl cursor-pointer" :icon="icons.cart_outline" />
                <span v-if="cartStore.recipeCount > 0" class="indicator-item badge badge-primary">{{
                    cartStore.recipeCount
                }}</span>
            </button>
            <theme-toggle dark_theme="pickeat_dark" light_theme="pickeat_light" @theme_changed="set_logo" />
            <button class="indicator" type="button" @click="$router.push({ name: 'account' })" aria-label="Mon compte">
                <Icon class="icon text-2xl sm:text-3xl md:text-4xl cursor-pointer" :icon="icons.account" />
                <span v-if="authStore.is_logged_in" class="indicator-item badge badge-primary badge-xs "></span>
            </button>
        </div>
    </nav>
    <router-view v-slot="{ Component, route }">
        <transition name="fade" mode="out-in">
            <KeepAlive include="NewRecipe,RecipeList">
                <component :is="Component" :key="route.path" ref="main" />
            </KeepAlive>
        </transition>
    </router-view>
    <grocery-list-modal ref="grocery_list_modal_el" />
    <toast :error_queue="notifStore.error_msgs" :info_queue="notifStore.info_msgs" />
</template>
