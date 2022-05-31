<template>
  <nav class="navbar" role="navigation" aria-label="main navigation">
  <div class="navbar-brand">
    <router-link to="/recipes">
      <img :src="require('@/assets/pickeat.png')" width="300">
    </router-link>


    <a role="button" class="navbar-burger" aria-label="menu" aria-expanded="false" data-target="mynavbar" @click="navbarIsOpen = !navbarIsOpen" v-bind:class="{'is-active': navbarIsOpen}">
      <span aria-hidden="true"></span>
      <span aria-hidden="true"></span>
      <span aria-hidden="true"></span>
    </a>
  </div>
  <div id="mynavbar" class="navbar-menu" v-bind:class="{'is-active': navbarIsOpen}">
    <div class="navbar-start">
        <router-link to="/recipes" class="navbar-item is-tab">Recettes</router-link>
        <router-link to="/new-recipe" class="navbar-item is-tab">Nouvelle recette</router-link>
    </div>
  </div>
  </nav>
  <router-view v-slot="{ Component, route }">
    <keep-alive>
          <component
            :is="Component"
            :key="route.meta.usePathKey ? route.path : undefined"
          />
    </keep-alive>
  </router-view>

</template>

<script>
import store from '@/store/store.js'
import {close_last_opened_modal} from '@/utils/utils.js'

export default {
  name: 'App',
  provide: {
    store,
  },
  components: {
  },
  data: function() {
    return {
        navbarIsOpen: false,
    }
  },
  created: function() {
    let api_calls = [store.getTags(), store.getCategories(), store.getIngredients(), store.getUnits(), store.getSeasons()]
    for (let ret of api_calls)
        ret.catch(error => console.error(error));

    document.addEventListener.call(window, "keydown", e => {
        if (e.key == 'Escape') {
            close_last_opened_modal()
        }
    });
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
@font-face {
  font-family: "Rounded_Elegance";
  src: local("Rounded_Elegance"),   url(./fonts/Rounded_Elegance.ttf) format("truetype");}
</style>
