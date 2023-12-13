<script setup>
import { onMounted, defineProps, defineAsyncComponent, ref } from 'vue'
import { useRouter, useRoute } from 'vue-router'

const RecipeForm = defineAsyncComponent(() => import('@/components/RecipeForm.vue'))
const RecipeView = defineAsyncComponent(() => import('@/components/RecipeView.vue'))
import { useFoodStore } from '@/store/food.js'

const foodStore = useFoodStore()
const router = useRouter()
const route = useRoute()

const props = defineProps({
    id: Number,
    edit: Boolean
});

const recipe = ref(null)

onMounted(() => {
    loadRecipe()
})

function loadRecipe() {
    foodStore.getRecipeById(props.id).then((result) => {
        recipe.value = result
        document.title = recipe.value.name + ' - Pickeat'
    })
}

function editRecipe() {
    router.push({ query: { ...route.query, edit: null } })
}

function afterEdit() {
    loadRecipe()
    router.go(-1)
}
</script>

<template>
    <div class="container px-4 my-4">
        <recipe-view v-if="!edit" :recipe="recipe" @edit="editRecipe" />
        <recipe-form v-else :existing_recipe="recipe" @done="afterEdit" />
    </div>
</template>

