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
export default {
    name: 'Recipe',
    components: {
      RecipeForm,
      RecipeView,
    },
    inject: ["store"],
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
    mounted() {
        this.loadRecipe()
    },
    methods: {
        toggleFavorite(recipe) {
            this.store.toggleFavorite(recipe)
        },
        loadRecipe() {
            this.store.getOneRecipe(this.id).then(result => {
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
