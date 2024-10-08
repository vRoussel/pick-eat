<script setup>
import { inject, ref, computed, onMounted } from 'vue'

import Multiselect from '@vueform/multiselect'

import { useFoodStore } from '@/store/food.js'
import { useAuthStore } from '@/store/auth.js'

const foodStore = useFoodStore()
const authStore = useAuthStore()

const icons = inject('icons')

const model = defineModel('filters', {
    required: true,
    type: Object,
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
    },
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

const only_favs = computed({
    get: function () {
        return model.value.only_favs
    },
    set: function (val) {
        let new_filters = { ...model.value, only_favs: val }
        updateFilters(new_filters, 0)
    },
})

const only_private = computed({
    get: function () {
        return model.value.only_private
    },
    set: function (val) {
        let new_filters = { ...model.value, only_private: val }
        updateFilters(new_filters, 0)
    },
})

const active_filters_count = computed(() => {
    let f = model.value
    return (
        f.diets.length +
        f.seasons.length +
        f.ingredients.length +
        f.tags.length +
        f.categories.length +
        (f.account ? 1 : 0) +
        (f.only_favs ? 1 : 0) +
        (f.only_private ? 1 : 0)
    )
})

onMounted(() => {
    expanded.value = !on_mobile()
    innerWidth_cached.value = window.innerWidth
    window.addEventListener('resize', () => {
        let old_val = innerWidth_cached.value
        let new_val = window.innerWidth
        if (old_val <= 768 && new_val > 768) expanded.value = true
        else if (old_val > 768 && new_val <= 768) expanded.value = false
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
export function Filters(
    q = null,
    i = [],
    t = [],
    c = [],
    s = [],
    a = null,
    d = [],
    of = false,
    op = false,
) {
    return {
        search_query: q,
        ingredients: i,
        tags: t,
        categories: c,
        seasons: s,
        account: a,
        diets: d,
        only_favs: of,
        only_private: op,
    }
}
</script>

<template>
    <div class="space-y-4 lg:space-y-6">
        <div class="flex gap-x-2">
            <div class="form-control relative grow">
                <div class="join bg-accent flex items-center">
                    <span class="icon text-xl bg-accent text-accent-content join px-3">
                        <Icon :icon="icons.search" :inline="true" />
                    </span>
                    <input
                        class="input w-full join-item"
                        type="search"
                        placeholder="Trouver une recette"
                        :value="search_query"
                        @input="(e) => (search_query = e.target.value)"
                    />
                </div>
                <span v-if="model.search_query" class="icon cursor-pointer" @click="clearSearch">
                    <Icon
                        :icon="icons.close"
                        class="absolute right-3 top-0 bottom-0 h-full"
                        @click="clearSearch"
                    />
                </span>
            </div>
        </div>
        <div class="form-control">
            <button
                class="btn w-full mx-auto btn-primary"
                :class="expanded ? '' : 'btn-outline'"
                @click="toggle"
                aria-label="Afficher/cacher les filtres"
            >
                Filtres ({{ active_filters_count }})
                <span class="icon text-xl">
                    <Icon :icon="expanded ? icons.arrow_up : icons.arrow_down" />
                </span>
            </button>
        </div>
        <div
            v-show="expanded"
            class="flex flex-col gap-y-4 border-b md:border-0 border-primary pb-8"
        >
            <div v-if="authStore.is_logged_in" class="form-control">
                <label class="label cursor-pointer justify-start gap-x-4 py-1">
                    <input
                        v-model="only_favs"
                        type="checkbox"
                        class="checkbox checkbox-sm checkbox-accent"
                    />
                    <span class="label-text">Mes favoris</span>
                </label>
                <label class="label cursor-pointer justify-start gap-x-4 py-1">
                    <input
                        v-model="only_private"
                        type="checkbox"
                        class="checkbox checkbox-sm checkbox-accent"
                    />
                    <span class="label-text">Mes recettes privées</span>
                </label>
            </div>
            <div class="form-control">
                <label class="label">
                    <span class="label-text">Tags</span>
                </label>
                <Multiselect
                    v-model="tags"
                    mode="tags"
                    :options="foodStore.tags"
                    label="name"
                    searchable
                    :strict="false"
                    track-by="name"
                    value-prop="id"
                    :close-on-select="false"
                />
            </div>
            <div class="form-control">
                <label class="label">
                    <span class="label-text">Ingrédients</span>
                </label>
                <Multiselect
                    v-model="ingredients"
                    mode="tags"
                    :options="foodStore.ingredients"
                    label="name"
                    searchable
                    :strict="false"
                    track-by="name"
                    value-prop="id"
                    :close-on-select="false"
                />
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
                        <input
                            v-model="diets"
                            type="checkbox"
                            class="checkbox checkbox-sm checkbox-accent"
                            :value="d.id"
                        />
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
                        <input
                            v-model="seasons"
                            type="checkbox"
                            class="checkbox checkbox-sm checkbox-accent"
                            :value="s.id"
                        />
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
                        <input
                            v-model="categories"
                            type="checkbox"
                            class="checkbox checkbox-sm checkbox-accent"
                            :value="c.id"
                        />
                        <span class="label-text">{{ c.name }}</span>
                    </label>
                </div>
            </fieldset>
            <div class="form-control">
                <label class="label">
                    <span class="label-text">Auteur</span>
                </label>
                <Multiselect
                    v-model="account"
                    mode="single"
                    :options="foodStore.accounts_with_recipes"
                    label="display_name"
                    searchable
                    :strict="false"
                    track-by="display_name"
                    value-prop="id"
                    :close-on-select="true"
                />
            </div>
            <div class="form-control">
                <button
                    class="btn btn-outline btn-primary"
                    @click="clearFilters"
                    aria-label="Réinitialiser les filtres"
                >
                    Réinitialiser les filtres
                    <span class="text-xl">
                        <Icon :icon="icons.reset" />
                    </span>
                </button>
            </div>
        </div>
    </div>
</template>
