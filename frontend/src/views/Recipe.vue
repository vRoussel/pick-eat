<script setup>
import { onMounted, defineProps, defineAsyncComponent, ref } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { useHead } from '@unhead/vue'
import { defineRecipe, useSchemaOrg } from '@unhead/schema-org'

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

function q_ingredient_to_text(q_ingr) {
    let ret = q_ingr.name
    if (q_ingr.quantity) {
        ret += ' x ' + q_ingr.quantity
        if (q_ingr.unit) {
            ret += ' ' + q_ingr.unit.short_name
        }
    }
    return ret
}

function diet_to_schema_org_format(diet) {
    return diet.label[0].toUpperCase() + diet.label.substring(1) + 'Diet'
}

function loadRecipe() {
    foodStore.getRecipeById(props.id).then((result) => {
        recipe.value = result
        useSchemaOrg([
            defineRecipe({
                name: recipe.value.name,
                image: recipe.value.image || null,
                recipeInstructions: recipe.value.instructions,
                recipeIngredient: recipe.value.q_ingredients.map(q_ingredient_to_text),
                datePublished: recipe.value.publication_date,
                cookTime: `${recipe.value.cook_time_min} minutes`,
                prepTime: `${recipe.value.prep_time_min} minutes`,
                recipeYield: recipe.value.n_shares.toString(),
                suitableForDiet: recipe.value.diets.map(diet_to_schema_org_format)
            })
        ])
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

