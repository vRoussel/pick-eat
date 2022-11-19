<template>
  <div>
    <recipe-form @done="afterInsert" />
  </div>
</template>

<script>
import RecipeForm from '@/components/RecipeForm.vue'

export default {
    name: 'NewRecipe',
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
