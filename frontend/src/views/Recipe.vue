<script setup>
import { onMounted, defineProps, defineAsyncComponent, ref } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { useHead } from '@unhead/vue'

const RecipeForm = defineAsyncComponent(() => import('@/components/RecipeForm.vue'))
const RecipeView = defineAsyncComponent(() => import('@/components/RecipeView.vue'))
import { useFoodStore } from '@/store/food.js'
import { useAuthStore } from '@/store/auth.js'

const foodStore = useFoodStore()
const authStore = useAuthStore()
const router = useRouter()
const route = useRoute()

const props = defineProps({
    id: Number,
    edit: Boolean
});

const recipe = ref(null)

useHead({
    title: () => recipe.value ? recipe.value.name : null
})

onMounted(() => {
    loadRecipe()
    if (!authStore.is_logged_in && props.edit) {
        authStore.return_url = route.fullPath
        router.push('/login')
    }
})

function loadRecipe() {
    foodStore.getRecipeById(props.id).then((result) => {
        recipe.value = result
    })
}

function editRecipe() {
    if (authStore.is_logged_in) {
        router.push({ query: { ...route.query, edit: null } })
    } else {
        authStore.return_url = route.fullPath
        router.push('/login')
    }
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

