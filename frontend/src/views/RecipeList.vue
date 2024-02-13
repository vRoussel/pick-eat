<script setup>
import { computed, ref, onMounted, watch, onActivated, onDeactivated, inject } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { useHead } from '@unhead/vue'
import isEqual from 'lodash.isequal'

import Pagination from '@/components/Pagination.vue'
import RecipeFilters from '@/components/RecipeFilters.vue'
import RecipeListItem from '@/components/RecipeListItem.vue'
import { useFoodStore } from '@/store/food.js'

import { Filters } from '@/components/RecipeFilters.vue'
import no_recipe_video from '@/assets/homer_hungry.mp4'

const foodStore = useFoodStore()
const router = useRouter()
const route = useRoute()
const icons = inject('icons')

const recipes = ref([])
const per_page = ref(12)
const total_count = ref(null)

const page = computed({
    get() {
        return parseInt(route.query.page) || 1
    },
    set(value) {
        router.push({ query: { ...route.query, page: value, ns: undefined } })
    },
})
const from = computed(() => {
    return (page.value - 1) * per_page.value + 1
})
const to = computed(() => {
    return page.value * per_page.value
})
const max_page = computed(() => {
    return Math.ceil(total_count.value / per_page.value) || 1
})

const canonical_url = computed(() => {
    let url = new URL(window.location)
    let params = new URLSearchParams()
    params.set('page', page.value)
    url.search = params
    return url.toString()
})

const filters = computed({
    get() {
        let q = route.query
        return Filters(
            q.search,
            q.i ? q.i.split(',') : [],
            q.t ? q.t.split(',') : [],
            q.c ? q.c.split(',') : [],
            q.s ? q.s.split(',') : [],
            q.a,
            q.d ? q.d.split(',') : [],
        )
    },
    set(f) {
        let q = { ...route.query }

        q.search = f.search_query || undefined
        q.i = f.ingredients.join(',') || undefined
        q.t = f.tags.join(',') || undefined
        q.c = f.categories.join(',') || undefined
        q.s = f.seasons.join(',') || undefined
        q.d = f.diets.join(',') || undefined
        q.a = f.account || undefined

        // Avoid scrolling to top if we are changing filters
        q.ns = null
        router.push({ name: 'recipe-list', query: q })
    },
})

let sort_method = computed({
    get: function () {
        return route.query.sort || "random"
    },
    set: function (val) {
        router.push({ name: 'recipe-list', query: { ...route.query, 'sort': val } })
    }
})

const sort_methods = ref([
    { text: "Ordre aléatoire (choix par défaut)", value: "random" },
    { text: "Ordre alphabétique", value: "name" },
    { text: "Les plus récentes", value: "pub_date_desc" },
    { text: "Les plus anciennes", value: "pub_date_asc" },
    { text: "Les plus rapides", value: "total_time" },
    { text: "Le moins d'ingrédients", value: "ingr_count" },
])

let last_query = null;
let saved_query = null;
function loadRecipes() {
    foodStore.getRecipes(from.value, to.value, filters.value, sort_method.value).then((result) => {
        let [_recipes, _total_count] = result
        recipes.value = _recipes
        total_count.value = _total_count
    })
    last_query = route.query
}

onMounted(() => {
    loadRecipes()
})

onDeactivated(() => {
    saved_query = last_query
})

onActivated(async () => {
    if (saved_query != null) {
        // Only restore saved_query if the current_one  has no query params
        if (
            Object.keys(route.query).length == 0 &&
            !isEqual(saved_query, route.query)
        ) {
            await router.replace({ query: saved_query })
        }
        loadRecipes()
        saved_query = null
    }

    useHead({
        title: "Recettes et liste de courses",
        meta: [
            {
                name: 'description',
                content: "Trouvez de nouvelles recettes et générez votre liste de courses, c'est tout. Sur pick-eat, vous ne trouverez ni pubs ni blabla."
            }
        ],
        link: () => canonical_url.value === window.location.toString() ? null : {
            rel: 'canonical',
            href: canonical_url.value
        }
    })

})

watch(() => route.query, () => {
    // If saved_query exists, it means it hasn't been restore yet
    // It will be soon so there's no point to call loadRecipes before that
    if (route.name == 'recipe-list' && saved_query == null) {
        loadRecipes()
    }
})

function on_mobile() {
    return window.innerWidth < 768
}
</script>

<template>
    <div class="flex my-4 mx-4 lg:mx-8 gap-x-8 gap-y-4 flex-col md:flex-row">
        <recipe-filters v-model:filters="filters" class="md:min-w-[16rem] md:max-w-[16rem]" />
        <div v-if="total_count > 0" class="w-full">
            <div class="join flex items-center w-full">
                <span class="icon text-xl text-base-content join px-3">
                    <Icon :icon="icons.sort" :inline="true" />
                </span>
                <select v-model="sort_method" class="select grow">
                    <option v-for="sm in sort_methods" :value="sm.value">
                        {{ sm.text }}
                    </option>
                </select>
            </div>
            <!--
            <p class="text-xl my-2">{{total_count}}  {{total_count > 1 ? "résultats" : "résultat"}}</p>
        -->
            <div
                class="p-4 gap-x-4 gap-y-6 lg:p-6 lg:gap-x-6 lg:gap-y-9 shadow-md shadow-accent rounded-md grid auto-rows-fr grid-cols-1 sm:grid-cols-2 xl:grid-cols-3 2xl:grid-cols-4">
                <div v-for="(recipe, idx) in recipes" :key="'r' + recipe.id">
                    <recipe-list-item :recipe="recipe" :lazy="idx >= 2 && on_mobile()" />
                </div>
            </div>
            <div class="max-w-fit mx-auto my-8">
                <pagination v-model:current_page="page" :max_page="max_page" />
            </div>
        </div>
        <div v-else-if="total_count == 0">
            <p class="text-xl mb-2">Aucune recette trouvée :(</p>
            <video :src="no_recipe_video" autoplay loop muted playsinline />
        </div>
    </div>
</template>
