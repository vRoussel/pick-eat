<template>
    <div>
        <div class="flex gap-x-2">
            <div class="form-control md:hidden">
                <button class="btn btn-accent btn-square" @click="toggle">
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
                    <input
                        class="input w-full join-item"
                        type="search"
                        placeholder="Trouver une recette"
                        :value="search_query"
                        @input="(e) => (search_query = e.target.value)"
                    />
                </div>
                <span v-if="filters.search_query" class="icon cursor-pointer" @click="clearSearch">
                    <Icon
                        :icon="icons.close"
                        class="absolute right-3 top-0 bottom-0 h-full"
                        @click="clearSearch"
                    />
                </span>
            </div>
        </div>
        <div v-show="expanded" class="flex flex-col gap-y-4 mt-4">
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
                <button class="btn btn-accent" @click="clearFilters">
                    <span class="icon text-xl">
                        <Icon :icon="icons.reset" />
                    </span>
                </button>
            </div>
        </div>
    </div>
</template>

<script>
import Multiselect from '@vueform/multiselect'

import { mapStores } from 'pinia'
import { useFoodStore } from '@/store/food.js'

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

export default {
    name: 'RecipeFilters',
    components: {
        Multiselect,
    },
    inject: ['icons'],
    props: {
        filters: {
            type: Object,
            required: true,
        },
    },
    emits: ['update:filters'],
    data: function () {
        return {
            timer: null,
            expanded: !this.on_mobile(),
            innerWidth_cached: window.innerWidth,
        }
    },
    computed: {
        ...mapStores(useFoodStore),
        search_query: {
            get: function () {
                return this.filters.search_query
            },
            set: function (val) {
                let new_filters = { ...this.filters, search_query: val }
                if (val == '' || val == null) this.updateFilters(new_filters, 0)
                else this.updateFilters(new_filters, 400)
            },
        },
        tags: {
            get: function () {
                return this.filters.tags
            },
            set: function (val) {
                let new_filters = { ...this.filters, tags: val }
                this.updateFilters(new_filters, 0)
            },
        },
        categories: {
            get: function () {
                return this.filters.categories
            },
            set: function (val) {
                let new_filters = { ...this.filters, categories: val }
                this.updateFilters(new_filters, 0)
            },
        },
        seasons: {
            get: function () {
                return this.filters.seasons
            },
            set: function (val) {
                let new_filters = { ...this.filters, seasons: val }
                this.updateFilters(new_filters, 0)
            },
        },
        ingredients: {
            get: function () {
                return this.filters.ingredients
            },
            set: function (val) {
                let new_filters = { ...this.filters, ingredients: val }
                this.updateFilters(new_filters, 0)
            },
        },
        account: {
            get: function () {
                return this.filters.account
            },
            set: function (val) {
                let new_filters = { ...this.filters, account: val }
                this.updateFilters(new_filters, 0)
            },
        },
        diets: {
            get: function () {
                return this.filters.diets
            },
            set: function (val) {
                let new_filters = { ...this.filters, diets: val }
                this.updateFilters(new_filters, 0)
            },
        },
    },
    mounted() {
        window.addEventListener('resize', () => {
            let old_val = this.innerWidth_cached
            let new_val = window.innerWidth
            if (old_val <= 768 && new_val > 768) this.expanded = true
            else if (old_val > 768 && new_val <= 768) this.expanded = false
            this.innerWidth_cached = new_val
        })
    },
    methods: {
        updateFilters(filters, delay) {
            if (this.timer) {
                clearTimeout(this.timer)
                this.timer = null
            }
            this.timer = setTimeout(() => {
                this.$emit('update:filters', filters)
            }, delay)
        },
        toggle() {
            this.expanded = !this.expanded
        },
        clearSearch() {
            this.search_query = null
        },
        clearFilters() {
            this.updateFilters(Filters(), 0)
        },
        on_mobile() {
            return window.innerWidth < 768
        },
    },
}
</script>
