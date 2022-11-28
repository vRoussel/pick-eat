<template>
  <nav class="navbar bg-base-100 border-b border-primary">
    <div class="navbar-start">
      <div
        ref="dd"
        class="dropdown"
      >
        <label
          tabindex="0"
          class="btn btn-ghost sm:hidden"
        >
          <svg
            xmlns="http://www.w3.org/2000/svg"
            class="h-5 w-5"
            fill="none"
            viewBox="0 0 24 24"
            stroke="currentColor"
          ><path
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-width="2"
            d="M4 6h16M4 12h8m-8 6h16"
          /></svg>
        </label>
        <ul
          tabindex="0"
          class="menu menu-compact dropdown-content mt-3 p-2 shadow bg-base-100 rounded-box w-52"
        >
          <li>
            <router-link
              to="/recipes"
              @mouseup="unfocus"
            >
              Recettes
            </router-link>
          </li>
          <li>
            <router-link
              to="/new-recipe"
              @mouseup="unfocus"
            >
              Nouvelle recette
            </router-link>
          </li>
        </ul>
      </div>
      <router-link
        class="min-w-[150px]"
        to="/recipes"
      >
        <img
          :src="pickeat_png"
          width="200"
        >
      </router-link>
      <ul class="shrink-0 grow menu menu-horizontal p-2 rounded-box hidden sm:flex">
        <li class="shrink-0">
          <router-link to="/recipes">
            Recettes
          </router-link>
        </li>
        <li class="shrink-0">
          <router-link to="/new-recipe">
            Nouvelle recette
          </router-link>
        </li>
      </ul>
    </div>
    <div class="navbar-end space-x-3">
      <button
        class="indicator"
        @click="this.$refs.grocery_list_modal.open()"
        type="button"
      >
        <Icon
          class="icon text-2xl sm:text-3xl md:text-4xl cursor-pointer"
          :icon="icons.cart_outline"
        />
        <span
          v-if="cartStore.recipeCount > 0"
          class="indicator-item badge badge-primary"
        >{{ cartStore.recipeCount }}</span>
      </button>
      <theme-toggle
        dark_theme="dark"
        light_theme="pickeat_light"
      />
    </div>
  </nav>
  <router-view v-slot="{ Component, route }">
    <transition
      name="fade"
      mode="out-in"
    >
      <KeepAlive>
        <component
          :is="Component"
          :key="route.path"
          ref="main"
        />
      </KeepAlive>
    </transition>
  </router-view>
  <grocery-list-modal ref="grocery_list_modal" />
</template>

<script>
import icons from '@/utils/icons.js'
import ThemeToggle from '@/components/ThemeToggle.vue'
import GroceryListModal from '@/components/GroceryListModal.vue'
import { mapStores } from 'pinia'
import { useCartStore } from '@/store/cart.js'
import { useApiStore } from '@/store/api.js'

import pickeat_png from '@/assets/pickeat.png'

export default {
  name: 'App',
  components: {
      ThemeToggle,
      GroceryListModal
  },
  provide: {
    icons
  },
  data: function() {
    return {
        navbarIsOpen: false,
        pickeat_png: pickeat_png,
        icons: icons
    }
  },
  computed: {
    ...mapStores(useCartStore, useApiStore)
  },
  watch: {
        $route: {
            immediate: true,
            handler(to) {
                document.title = to.meta.title || 'Some Default Title';
            }
        },
  },
  created: async function() {
    await Promise.all([
        this.apiStore.fetchTags(),
        this.apiStore.fetchCategories(),
        this.apiStore.fetchIngredients(),
        this.apiStore.fetchUnits(),
        this.apiStore.fetchSeasons()
    ])
  },
  methods: {
        unfocus(e) {
            let targetEl = e.currentTarget;
            setTimeout(function(){
                targetEl.blur()
            }, 0)
        }
  }
}
</script>

<style>
    @font-face {
      font-family: "Rounded_Elegance";
      src: local("Rounded_Elegance"),   url(./fonts/Rounded_Elegance.ttf) format("truetype");
    }

    .fade-enter-active,
    .fade-leave-active {
      transition: opacity 0.2s ease;
    }

    .fade-enter-from,
    .fade-leave-to {
      opacity: 0;
    }
</style>
