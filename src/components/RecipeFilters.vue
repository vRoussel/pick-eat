<template>
<div>
    <div class="field is-grouped is-grouped-centered">
            <div class="control has-icons-left has-icons-right is-expanded">
                    <input @input="e => filters.search_query = e.target.value" class="input is-rounded" type="text" placeholder="Trouver une recette" :value="filters.search_query">
                    <span class="icon is-right" v-if="filters.search_query">
                       <i class="fas fa-times is-clickable" @click="clearSearch"></i>
                    </span>
                    <span class="icon is-left">
                       <i class="fas fa-search"></i>
                    </span>
            </div>
            <div class="control">
                <button class="button" @click="toggle">
                <span class="icon">
                  <i class="fas fa-sliders-h"></i>
                </span>
                </button>
            </div>
    </div>
    <div v-show="this.expanded">
        <div class="field">
            <label class="label">Ingrédients</label>
                <Multiselect mode="tags" :options="this.store.state.ingredients" label="name" searchable :strict="false" trackBy="name" valueProp="id" ref="multiselect" :closeOnSelect="false" v-model="this.filters.ingredients"/>
        </div>
        <!--
        <div class="field">
            <label class="label">Tags</label>
        <Multiselect mode="tags" :options="this.store.state.tags" label="name" searchable :strict="false" trackBy="name" object valueProp="id" ref="multiselect" :closeOnSelect="false"/>
        </div>
        -->
        <fieldset class="block">
            <legend class="label">Tags</legend>
            <div class="control" v-for="t in this.store.state.tags" :key="t.id">
                <label class="checkbox">
                    <input type="checkbox" v-model="this.filters.tags" :value="t.id">
                    {{ t.name }}
                </label>
            </div>
        </fieldset>
        <fieldset class="block">
            <legend class="label">Saisons</legend>
            <div class="control" v-for="s in this.store.state.seasons" :key="s.id">
                <label class="checkbox">
                    <input type="checkbox" v-model="this.filters.seasons" :value="s.id">
                    {{ s.name }}
                </label>
            </div>
        </fieldset>
        <fieldset class="block">
            <legend class="label">Catégories</legend>
            <div class="control" v-for="c in this.store.state.categories" :key="c.id">
                <label class="checkbox">
                    <input type="checkbox" v-model="this.filters.categories" :id ="c.id" :value="c.id">
                    {{ c.name }}
                </label>
            </div>
        </fieldset>
        <div class="field">
            <div class="control">
                <button class="button is-fullwidth" @click="clearFilters">
                <span class="icon">
                  <i class="fas fa-undo"></i>
                </span>
                </button>
            </div>
        </div>
    </div>
</div>
</template>

<script>
import Multiselect from '@vueform/multiselect'

export class Filters {
    constructor (q=null, i=[], t=[], c=[], s=[]) {
        this.search_query = q,
        this.ingredients = i,
        this.tags = t,
        this.categories = c,
        this.seasons = s
    }
}

export default {
    name: 'recipe-filters',
    inject: ["store"],
    components: {
        Multiselect,
    },
    data: function() {
        return {
            timer: null,
            expanded: this.on_mobile() ? false : true,
            filters: structuredClone(this.initial_filters)
        }
    },
    props: {
        initial_filters : {
            type: Object,
            default: new Filters()
        }
    },
    methods: {
        scheduleSearch(filters, delay) {
            if (this.timer) {
                clearTimeout(this.timer)
                this.timer = null
            }
            this.timer = setTimeout(() => {
                this.$emit('search', filters)
            }, delay)
        },
        toggle() {
            this.expanded = !this.expanded
        },
        clearSearch() {
            this.filters.search_query = null
        },
        clearFilters() {
            this.filters.ingredients = []
            this.filters.tags = []
            this.filters.categories = []
            this.filters.seasons = []
        },
        on_mobile() {
            return screen.width < 768;
        }
    },
    watch: {
        'filters.search_query': function() {
            this.scheduleSearch(this.filters, 400)
        },
        'filters.tags': function() {
            this.scheduleSearch(this.filters, 0)
        },
        'filters.categories': function() {
            this.scheduleSearch(this.filters, 0)
        },
        'filters.seasons': function() {
            this.scheduleSearch(this.filters, 0)
        },
        'filters.ingredients': function() {
            this.scheduleSearch(this.filters, 0)
        },
        initial_filters: {
            handler(val) {
                this.filters = structuredClone(val)
            },
            deep: true
        }
    },
    emits: ['search', 'toggle']
}
</script>

<style lang="scss" scoped>
    #filter-icon {
        margin-bottom: 0.75rem;
    }
</style>
