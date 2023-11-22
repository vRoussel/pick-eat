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
import { defineAsyncComponent } from 'vue'

const RecipeForm = defineAsyncComponent(() =>
  import('@/components/RecipeForm.vue')
)
const RecipeView = defineAsyncComponent(() =>
  import('@/components/RecipeView.vue')
)

import { mapStores } from 'pinia'
import { useFoodStore } from '@/store/food.js'

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
        ...mapStores(useFoodStore),
    },
    mounted() {
        this.loadRecipe()
    },
    methods: {
        loadRecipe() {
            this.foodStore.getRecipeById(this.id).then(result => {
                this.recipe = result
                document.title = this.recipe.name + ' - Pickeat'
            });
        },
        editRecipe() {
            this.$router.push({ query: { ...this.$route.query, edit: null} });
        },
        afterEdit() {
            this.loadRecipe()
            this.$router.go(-1)
        }
    },
}
</script>
