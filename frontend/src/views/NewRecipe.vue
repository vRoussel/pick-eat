<script setup>
import { onActivated } from 'vue'
import { useRouter } from 'vue-router'

import RecipeForm from '@/components/RecipeForm.vue'
import { useFoodStore } from '@/store/food.js'

const foodStore = useFoodStore()
const router = useRouter()

function viewRecipe(recipe) {
    let id = recipe.id
    router.push({ name: 'recipe', params: { id } })
}
onActivated(() => {
    foodStore.fetchAll()
})
</script>

<template>
    <div>
        <recipe-form @done="viewRecipe" />
    </div>
</template>

