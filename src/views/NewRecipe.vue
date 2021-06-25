<template>
    <div class="container is-max-desktop my-4 px-4">
        <recipe-form></recipe-form>
    </div>
</template>

<script>
import RecipeForm from '@/components/RecipeForm.vue'

export default {
    name: 'new-recipe',
    components: {
      RecipeForm,
    },
}
</script>
