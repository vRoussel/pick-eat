<template>
    <nav class="navbar bg-base-100 border-b border-primary">
      <div class="navbar-start">
        <div class="dropdown" ref="dd">
          <label tabindex="0" class="btn btn-ghost sm:hidden">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h8m-8 6h16" /></svg>
          </label>
          <ul tabindex="0" class="menu menu-compact dropdown-content mt-3 p-2 shadow bg-base-100 rounded-box w-52">
            <li><router-link @mouseup="unfocus" to="/recipes">Recettes</router-link></li>
            <li><router-link @mouseup="unfocus" to="/new-recipe">Nouvelle recette</router-link></li>
          </ul>
        </div>
        <router-link class="min-w-[150px]" to="/recipes">
          <img :src="pickeat_png" width="200">
        </router-link>
        <ul class="shrink-0 grow menu menu-horizontal p-0 hidden sm:flex">
        <li class="shrink-0"><router-link to="/recipes">Recettes</router-link></li>
        <li class="shrink-0"><router-link to="/new-recipe">Nouvelle recette</router-link></li>
        </ul>
      </div>
      <div class="navbar-end space-x-3">
        <label for="modal_gl" class="indicator">
            <ion-icon class="icon text-2xl sm:text-3xl md:text-4xl cursor-pointer" :src="cart_outline_svg"></ion-icon>
            <span v-if="nbItemsInCart() > 0" class="indicator-item badge badge-primary">{{ nbItemsInCart() }}</span>
        </label>
        <theme-toggle dark_theme="dark" light_theme="pickeat_light"/>
      </div>
    </nav>
  <router-view v-slot="{ Component, route }">
  <transition name="fade" mode="out-in">
        <component
          :is="Component"
          :key="route.path"
          ref="main"
        />
  </transition>
</router-view>
<grocery-list-modal modal_id="modal_gl"/>
</template>

<script>
import store from '@/store/store.js'
import cart from '@/store/cart.js'
import ThemeToggle from '@/components/ThemeToggle.vue'
import GroceryListModal from '@/components/GroceryListModal.vue'

import pickeat_png from '@/assets/pickeat.png'
import cart_outline_svg from '@/assets/ionicons/cart-outline.svg'

export default {
  name: 'App',
  provide: {
    store,
    cart
  },
  components: {
      ThemeToggle,
      GroceryListModal
  },
  data: function() {
    return {
        navbarIsOpen: false,
        pickeat_png: pickeat_png,
        cart_outline_svg: cart_outline_svg
    }
  },
  created: function() {
    let api_calls = [store.getTags(), store.getCategories(), store.getIngredients(), store.getUnits(), store.getSeasons()]
    for (let ret of api_calls)
        ret.catch(error => console.error(error));
  },
  methods: {
        unfocus(e) {
            let targetEl = e.currentTarget;
            setTimeout(function(){
                targetEl.blur()
            }, 0)
        },
        nbItemsInCart() {
            return cart.recipeCount()
        }
  },
  watch: {
        $route: {
            immediate: true,
            handler(to) {
                document.title = to.meta.title || 'Some Default Title';
            }
        },
  }
}
</script>

<style>
    @import '@vueform/multiselect/themes/tailwind.css';
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

    .multiselect-dropdown {
        @apply !max-h-[40vh];
    }

    .multiselect {
        @apply !bg-base-100;
    }

    .multiselect * {
        @apply !bg-base-100;
    }
</style>
