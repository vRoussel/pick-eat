<template>
<div>
    <div class="flex gap-x-2">
            <div class="form-control md:hidden">
                <button class="btn btn-accent btn-square" @click="toggle">
                <span class="icon">
                  <ion-icon name="options" size="small"></ion-icon>
                </span>
                </button>
            </div>
            <div class="form-control relative grow">
                <div class="input-group">
                    <span class="icon">
                       <ion-icon name="search" size="small"></ion-icon>
                    </span>
                    <input @input="e => search_query = e.target.value" class="input w-full" type="search" placeholder="Trouver une recette" :value="search_query">
                </div>
                <span v-if="filters.search_query" class="icon cursor-pointer" @click="clearSearch">
                   <ion-icon name="close" class="absolute right-3 top-0 bottom-0 h-full" @click="clearSearch"></ion-icon>
               </span>
            </div>
    </div>
    <div v-show="this.expanded" class="flex flex-col gap-y-4 mt-4">
        <div class="form-control">
            <label class="label">
                <span class="label-text text-lg">Ingrédients</span>
            </label>
            <Multiselect mode="tags" :options="this.store.state.ingredients" label="name" searchable :strict="false" trackBy="name" valueProp="id" ref="multiselect" :closeOnSelect="false" v-model="this.ingredients"/>
        </div>
        <div class="form-control">
            <label class="label">
                <span class="label-text text-lg">Tags</span>
            </label>
            <Multiselect mode="tags" :options="this.store.state.tags" label="name" searchable :strict="false" trackBy="name" valueProp="id" ref="multiselect" :closeOnSelect="false" v-model="this.tags"/>
        </div>
        <!--
        <fieldset>
            <legend class="label">
                <span class="label-text text-lg">Tags</span>
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
                <span class="label-text text-lg">Saisons</span>
            </legend>
            <div class="form-control" v-for="s in this.store.state.seasons" :key="s.id">
                <label class="label cursor-pointer justify-start gap-x-4 py-1">
                    <input type="checkbox" class="checkbox checkbox-sm" v-model="this.seasons" :value="s.id">
                    <span class="label-text">{{ s.name }}</span>
                </label>
            </div>
        </fieldset>
        <fieldset>
            <legend class="label">
                <span class="label-text text-lg">Catégories</span>
            </legend>
            <div class="form-control" v-for="c in this.store.state.categories" :key="c.id">
                <label class="label cursor-pointer justify-start gap-x-4 py-1">
                    <input type="checkbox" class="checkbox checkbox-sm" v-model="this.categories" :value="c.id">
                    <span class="label-text">{{ c.name }}</span>
                </label>
            </div>
        </fieldset>
        <div class="form-control">
            <button class="btn btn-accent" @click="clearFilters">
            <span class="icon">
              <ion-icon name="trash" size="small"></ion-icon>
            </span>
            </button>
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
            expanded: !this.on_mobile(),
            innerWidth_cached: window.innerWidth
        }
    },
    props: {
        filters : {
            type: Object,
        }
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
            this.updateFilters(new Filters(), 0)
        },
        on_mobile() {
            return window.innerWidth <= 768;
        }
    },
    mounted() {
        window.addEventListener('resize', () => {
            let old_val = this.innerWidth_cached
            let new_val = window.innerWidth
            if (old_val <= 768 && new_val > 768)
                this.expanded = true
            else if (old_val > 768 && new_val <= 768)
                this.expanded = false
                this.innerWidth_cached = new_val
        })
    },
    computed: {
        search_query: {
            get: function() {
                return this.filters.search_query;
            },
            set: function(val) {
                let new_filters = {...this.filters, 'search_query': val};
                if (val == "" || val == null)
                    this.updateFilters(new_filters, 0);
                else
                    this.updateFilters(new_filters, 400);
            }
        },
        tags: {
            get: function() {
                console.log('get_tags')
                return this.filters.tags;
            },
            set: function(val) {
                console.log('set_tags')
                let new_filters = {...this.filters, 'tags': val};
                this.updateFilters(new_filters, 0);
            }
        },
        categories: {
            get: function() {
                console.log('get_categs')
                return this.filters.categories;
            },
            set: function(val) {
                console.log('set_categs')
                let new_filters = {...this.filters, 'categories': val};
                this.updateFilters(new_filters, 0);
            }
        },
        seasons: {
            get: function() {
                return this.filters.seasons;
            },
            set: function(val) {
                let new_filters = {...this.filters, 'seasons': val};
                this.updateFilters(new_filters, 0);
            }
        },
        ingredients: {
            get: function() {
                return this.filters.ingredients;
            },
            set: function(val) {
                let new_filters = {...this.filters, 'ingredients': val};
                this.updateFilters(new_filters, 0);
            }
        }
    },
    emits: ['toggle', 'update:filters']
}
</script>
