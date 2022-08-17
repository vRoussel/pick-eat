<template>
    <div>
        <recipe-form @done='afterInsert'></recipe-form>
    </div>
</template>

<script>
import RecipeForm from '@/components/RecipeForm.vue'

export default {
    name: 'new-recipe',
    components: {
      RecipeForm,
    },
    methods: {
        afterInsert(recipe) {
            let id = recipe.id
            this.$router.push({ name: 'recipe', params: { id } })
        }
    }
}
</script>
