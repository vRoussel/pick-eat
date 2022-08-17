<template>
    <div class="container px-4 my-4">
       <recipe-view v-if="!edit" :recipe='recipe' @edit="editRecipe"></recipe-view>
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
    methods: {
        toggleFavorite(recipe) {
            this.store.toggleFavorite(recipe)
        },
        loadRecipe() {
            this.store.getOneRecipe(this.id).then(result => {
                this.recipe = result
                this.$route.meta.title = this.recipe.name + ' - PickEat'
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
    mounted() {
        this.loadRecipe()
    },
}
</script>
