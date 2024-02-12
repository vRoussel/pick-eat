<script setup>
import { inject, ref, computed, onMounted } from 'vue'

import Multiselect from '@vueform/multiselect'

import { useFoodStore } from '@/store/food.js'

const foodStore = useFoodStore()

const icons = inject('icons')

const model = defineModel('filters', {
    required: true,
    type: Object
})

const timer = ref(null)
const expanded = ref(false)
const innerWidth_cached = ref(0)

const search_query = computed({
    get: function () {
        return model.value.search_query
    },
    set: function (val) {
        let new_filters = { ...model.value, search_query: val }
        if (val == '' || val == null) updateFilters(new_filters, 0)
        else updateFilters(new_filters, 400)
    }
})

const categories = computed({
    get: function () {
        return model.value.categories
    },
    set: function (val) {
        let new_filters = { ...model.value, categories: val }
        updateFilters(new_filters, 0)
    },
})

const tags = computed({
    get: function () {
        return model.value.tags
    },
    set: function (val) {
        let new_filters = { ...model.value, tags: val }
        updateFilters(new_filters, 0)
    },
})

const ingredients = computed({
    get: function () {
        return model.value.ingredients
    },
    set: function (val) {
        let new_filters = { ...model.value, ingredients: val }
        updateFilters(new_filters, 0)
    },
})

const seasons = computed({
    get: function () {
        return model.value.seasons
    },
    set: function (val) {
        let new_filters = { ...model.value, seasons: val }
        updateFilters(new_filters, 0)
    },
})

const account = computed({
    get: function () {
        return model.value.account
    },
    set: function (val) {
        let new_filters = { ...model.value, account: val }
        updateFilters(new_filters, 0)
    },
})

const diets = computed({
    get: function () {
        return model.value.diets
    },
    set: function (val) {
        let new_filters = { ...model.value, diets: val }
        updateFilters(new_filters, 0)
    },
})

onMounted(() => {
    expanded.value = !on_mobile();
    innerWidth_cached.value = window.innerWidth
    window.addEventListener('resize', () => {
        let old_val = innerWidth_cached.value
        let new_val = window.innerWidth
        if (old_val <= 768 && new_val > 768)
            expanded.value = true
        else if (old_val > 768 && new_val <= 768)
            expanded.value = false
        innerWidth_cached.value = new_val
    })
})

function updateFilters(filters, delay) {
    if (timer.value) {
        clearTimeout(timer.value)
        timer.value = null
    }
    timer.value = setTimeout(() => {
        model.value = filters
    }, delay)
}

function toggle() {
    expanded.value = !expanded.value
}

function clearSearch() {
    search_query.value = null
}

function clearFilters() {
    updateFilters(Filters(), 0)
}
function on_mobile() {
    return window.innerWidth < 768
}
</script>

//Maybe this should be in a dedicated .js file, I'm not sure
<script>
export function Filters(q = null, i = [], t = [], c = [], s = [], a = null, d = []) {
    return {
        search_query: q,
        ingredients: i,
        tags: t,
        categories: c,
        seasons: s,
        account: a,
        diets: d,
    }
}
</script>

<template>
    <div>
        <div class="flex gap-x-2">
            <div class="form-control md:hidden">
                <button class="btn btn-accent btn-square" @click="toggle" aria-label="Afficher/cacher les filtres">
                    <span class="icon text-xl">
                        <Icon :icon="icons.options" />
                    </span>
                </button>
            </div>
            <div class="form-control relative grow">
                <div class="join bg-accent flex items-center">
                    <span class="icon text-xl bg-accent text-accent-content join px-3">
                        <Icon :icon="icons.search" :inline="true" />
                    </span>
                    <input class="input w-full join-item" type="search" placeholder="Trouver une recette"
                        :value="search_query" @input="(e) => (search_query = e.target.value)" />
                </div>
                <span v-if="model.search_query" class="icon cursor-pointer" @click="clearSearch">
                    <Icon :icon="icons.close" class="absolute right-3 top-0 bottom-0 h-full" @click="clearSearch" />
                </span>
            </div>
        </div>
        <div v-show="expanded" class="flex flex-col gap-y-4 mt-4">
            <div class="form-control">
                <label class="label">
                    <span class="label-text">Ingrédients</span>
                </label>
                <Multiselect v-model="ingredients" mode="tags" :options="foodStore.ingredients" label="name" searchable
                    :strict="false" track-by="name" value-prop="id" :close-on-select="false" />
            </div>
            <div class="form-control">
                <label class="label">
                    <span class="label-text">Tags</span>
                </label>
                <Multiselect v-model="tags" mode="tags" :options="foodStore.tags" label="name" searchable :strict="false"
                    track-by="name" value-prop="id" :close-on-select="false" />
            </div>
            <!--
        <fieldset>
            <legend class="label">
                <span class="label-text">Tags</span>
            </legend>
            <div class="form-control" v-for="t in this.store.state.tags" :key="t.id">
                <label class="label cursor-pointer justify-start gap-x-4 py-1">
                    <input type="checkbox" class="checkbox checkbox-sm" v-model="this.tags" :value="t.id">
                    <span class="label-text">{{ t.name }}</span>
                </label>
            </div>
        </fieldset>
        -->
            <fieldset>
                <legend class="label">
                    <span class="label-text">Régimes alimentaires</span>
                </legend>
                <div v-for="d in foodStore.diets" :key="d.id" class="form-control">
                    <label class="label cursor-pointer justify-start gap-x-4 py-1">
                        <input v-model="diets" type="checkbox" class="checkbox checkbox-sm checkbox-accent" :value="d.id" />
                        <span class="label-text">{{ d.name }}</span>
                    </label>
                </div>
            </fieldset>
            <fieldset>
                <legend class="label">
                    <span class="label-text">Saisons</span>
                </legend>
                <div v-for="s in foodStore.seasons" :key="s.id" class="form-control">
                    <label class="label cursor-pointer justify-start gap-x-4 py-1">
                        <input v-model="seasons" type="checkbox" class="checkbox checkbox-sm checkbox-accent"
                            :value="s.id" />
                        <span class="label-text">{{ s.name }}</span>
                    </label>
                </div>
            </fieldset>
            <fieldset>
                <legend class="label">
                    <span class="label-text">Catégories</span>
                </legend>
                <div v-for="c in foodStore.categories" :key="c.id" class="form-control">
                    <label class="label cursor-pointer justify-start gap-x-4 py-1">
                        <input v-model="categories" type="checkbox" class="checkbox checkbox-sm checkbox-accent"
                            :value="c.id" />
                        <span class="label-text">{{ c.name }}</span>
                    </label>
                </div>
            </fieldset>
            <div class="form-control">
                <label class="label">
                    <span class="label-text">Auteur</span>
                </label>
                <Multiselect v-model="account" mode="single" :options="foodStore.accounts_with_recipes" label="display_name"
                    searchable :strict="false" track-by="display_name" value-prop="id" :close-on-select="true" />
            </div>
            <div class="form-control">
                <button class="btn btn-accent" @click="clearFilters" aria-label="Reinitialiser les filtres">
                    <span class="icon text-xl">
                        <Icon :icon="icons.reset" />
                    </span>
                </button>
            </div>
        </div>
    </div>
</template>

