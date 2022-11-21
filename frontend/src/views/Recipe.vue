<template>
  <div class="container px-4 my-4">
    <recipe-view
      v-if="!edit"
      :recipe="recipe"
      @edit="editRecipe"
    />
    <recipe-form
      v-else
      :existing_recipe="recipe"
      @done="afterEdit"
    />
  </div>
</template>

<script>
import RecipeForm from '@/components/RecipeForm.vue'
import RecipeView from '@/components/RecipeView.vue'

import { mapStores } from 'pinia'
import { useApiStore } from '@/store/api.js'

export default {
    name: 'Recipe',
    components: {
      RecipeForm,
      RecipeView,
    },
    props: {
        id: {
            type: Number,
        },
        edit: {
            type: Boolean
        }
    },
    data: function() {
        return {
            recipe: null
        }
    },
    computed: {
        ...mapStores(useApiStore),
    },
    mounted() {
        this.loadRecipe()
    },
    methods: {
        loadRecipe() {
            this.apiStore.getRecipeById(this.id).then(result => {
                this.recipe = result
                document.title = this.recipe.name + ' - Pickeat'
            });
        },
        editRecipe() {
            this.$router.push({ query: { ...this.$route.query, edit: null} });
        },
        afterEdit() {
            this.loadRecipe()
            this.$router.push({ query: { ...this.$route.query, edit: undefined} });
        }
    },
}
</script>
