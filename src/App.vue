<template>
  <nav class="navbar" role="navigation" aria-label="main navigation">
  <div class="navbar-brand">
    <a class="navbar-item" href="https://bulma.io">
      <img src="https://bulma.io/images/bulma-logo.png" width="112" height="28">
    </a>

    <a role="button" class="navbar-burger" aria-label="menu" aria-expanded="false" data-target="mynavbar" @click="navbarIsOpen = !navbarIsOpen" v-bind:class="{'is-active': navbarIsOpen}">
      <span aria-hidden="true"></span>
      <span aria-hidden="true"></span>
      <span aria-hidden="true"></span>
    </a>
  </div>
  <div id="mynavbar" class="navbar-menu" v-bind:class="{'is-active': navbarIsOpen}">
    <div class="navbar-start">
        <router-link to="/recipes?page=1" class="navbar-item is-tab">Recettes</router-link>
        <router-link to="/new-recipe" class="navbar-item is-tab">Nouvelle recette</router-link>
    </div>
  </div>
  </nav>
  <router-view />
</template>

<script>
import store from '@/store/store.js'

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
  }
}
</script>

<style>
@font-face {
  font-family: "Rounded_Elegance";
  src: local("Rounded_Elegance"),   url(./fonts/Rounded_Elegance.ttf) format("truetype");}
</style>
