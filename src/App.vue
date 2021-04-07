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
        <router-link to="/" class="navbar-item is-tab">Home</router-link>
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
  components: {
  },
  data: function() {
    return {
        navbarIsOpen: false,
    }
  },
  created() {
    let api_host = '192.168.1.59';
    fetch(`http://${api_host}/api/v1/tags`)
        .then(response => response.json())
        .then(json => {
            store.tags = json;
        });
    fetch(`http://${api_host}/api/v1/categories`)
        .then(response => response.json())
        .then(json => {
            store.categories = json
        });
    fetch(`http://${api_host}/api/v1/units`)
        .then(response => response.json())
        .then(json => {
            store.units = json
        });
    fetch(`http://${api_host}/api/v1/ingredients`)
        .then(response => response.json())
        .then(json => {
            store.ingredients = json
        });
    }

}
</script>
