<template>
        <div class="container is-max-desktop px-4 my-4">
           <recipe-view v-if="mode === 'view'" :recipe='recipe' @edit="editRecipe"></recipe-view>
           <recipe-form v-else :existing_recipe='recipe' @done='afterEdit'></recipe-form>
        </div>
</template>

<script>
import RecipeForm from '@/components/RecipeForm.vue'
import RecipeView from '@/components/RecipeView.vue'
export default {
    name: 'recipe',
    components: {
      RecipeForm,
      RecipeView,
    },
    inject: ["store"],
    props: {
        id: {
            type: Number,
        }
    },
    data: function() {
        return {
            recipe: null,
            mode: 'view',
        }
    },
    methods: {
        toggleFavorite(recipe) {
            this.store.toggleFavorite(recipe)
        },
        loadRecipe() {
            this.store.getOneRecipe(this.id).then(result => {
                this.recipe = result
                document.title = this.recipe.name + ' - PickEat'
            });
        },
        editRecipe() {
            this.mode = 'update'
        },
        afterEdit() {
            this.loadRecipe()
            this.mode = 'view'
        }
    },
    created() {
        this.loadRecipe()
    },
}
</script>
