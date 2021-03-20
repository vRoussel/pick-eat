<template>
  <new-recipe-form :tags="this.tags" :categories="this.categories" :units="this.units" :ingredients="this.ingredients"></new-recipe-form>
</template>

<script>
import NewRecipeForm from './components/NewRecipeForm.vue'
import store from '@/store/store.js'

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
            store.tags = json;
        });
    fetch('http://127.0.0.1/api/v1/categories')
        .then(response => response.json())
        .then(json => {
            this.categories = json
            store.categories = json
        });
    fetch('http://127.0.0.1/api/v1/units')
        .then(response => response.json())
        .then(json => {
            this.units = json
            store.units = json
        });
    fetch('http://127.0.0.1/api/v1/ingredients')
        .then(response => response.json())
        .then(json => {
            this.ingredients = json
            store.ingredients = json
        });
    }

}
</script>
