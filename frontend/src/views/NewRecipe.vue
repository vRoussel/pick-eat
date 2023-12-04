<template>
    <div>
        <recipe-form @done="afterInsert" />
    </div>
</template>

<script>
import RecipeForm from '@/components/RecipeForm.vue'
import { mapStores } from 'pinia'
import { useFoodStore } from '@/store/food.js'

export default {
    name: 'NewRecipe',
    components: {
        RecipeForm,
    },
    methods: {
        afterInsert(recipe) {
            let id = recipe.id
            this.$router.push({ name: 'recipe', params: { id } })
        },
    },
    computed: {
        ...mapStores(useFoodStore),
    },
    activated() {
        this.foodStore.fetchAll()
    },
}
</script>
