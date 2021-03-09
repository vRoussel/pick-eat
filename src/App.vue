<template>
  <new-recipe-form :tags="this.tags" :categories="this.categories" :units="this.units" :ingredients="this.ingredients"></new-recipe-form>
</template>

<script>
import NewRecipeForm from './components/NewRecipeForm.vue'

export default {
  name: 'App',
  components: {
    NewRecipeForm
  },
  data: function() {
    return {
        categories: [],
        tags: [],
        ingredients: [],
        units: [],
    }
  },
  created() {
    fetch('http://127.0.0.1/api/v1/tags')
        .then(response => response.json())
        .then(json => {
            this.tags = json
        });
    fetch('http://127.0.0.1/api/v1/categories')
        .then(response => response.json())
        .then(json => {
            this.categories = json
        });
    fetch('http://127.0.0.1/api/v1/units')
        .then(response => response.json())
        .then(json => {
            this.units = json
        });
    fetch('http://127.0.0.1/api/v1/ingredients')
        .then(response => response.json())
        .then(json => {
            this.ingredients = json
        });
    }

}
</script>

<!--
<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  margin-top: 60px;
}
</style>
-->
