<template>
        <div class="container is-max-desktop px-4 my-4">
           <recipe-view :recipe='recipe'></recipe-view>
        </div>
</template>

<script>
import RecipeView from '@/components/RecipeView.vue'
export default {
    name: 'recipe',
    components: {
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
        }
    },
    created() {
        this.loadRecipe()
    },
}
</script>
