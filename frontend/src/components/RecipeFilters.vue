<template>
  <div>
    <div class="flex gap-x-2">
      <div class="form-control md:hidden">
        <button
          class="btn btn-accent btn-square"
          @click="toggle"
        >
          <span class="icon text-xl">
            <Icon :icon="icons.options" />
          </span>
        </button>
      </div>
      <div class="form-control relative grow">
        <div class="input-group">
          <span class="icon text-xl">
            <Icon
              :icon="icons.search"
              :inline="true"
            />
          </span>
          <input
            class="input w-full"
            type="search"
            placeholder="Trouver une recette"
            :value="search_query"
            @input="e => search_query = e.target.value"
          >
        </div>
        <span
          v-if="filters.search_query"
          class="icon cursor-pointer"
          @click="clearSearch"
        >
          <Icon
            :icon="icons.close"
            class="absolute right-3 top-0 bottom-0 h-full"
            @click="clearSearch"
          />
        </span>
      </div>
    </div>
    <div
      v-show="expanded"
      class="flex flex-col gap-y-4 mt-4"
    >
      <div class="form-control">
        <label class="label">
          <span class="label-text text-lg">Ingrédients</span>
        </label>
        <Multiselect
          ref="multiselect"
          v-model="ingredients"
          mode="tags"
          :options="store.state.ingredients"
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
          <span class="label-text text-lg">Tags</span>
        </label>
        <Multiselect
          ref="multiselect"
          v-model="tags"
          mode="tags"
          :options="store.state.tags"
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
        <div
          v-for="s in store.state.seasons"
          :key="s.id"
          class="form-control"
        >
          <label class="label cursor-pointer justify-start gap-x-4 py-1">
            <input
              v-model="seasons"
              type="checkbox"
              class="checkbox checkbox-sm"
              :value="s.id"
            >
            <span class="label-text">{{ s.name }}</span>
          </label>
        </div>
      </fieldset>
      <fieldset>
        <legend class="label">
          <span class="label-text text-lg">Catégories</span>
        </legend>
        <div
          v-for="c in store.state.categories"
          :key="c.id"
          class="form-control"
        >
          <label class="label cursor-pointer justify-start gap-x-4 py-1">
            <input
              v-model="categories"
              type="checkbox"
              class="checkbox checkbox-sm"
              :value="c.id"
            >
            <span class="label-text">{{ c.name }}</span>
          </label>
        </div>
      </fieldset>
      <div class="form-control">
        <button
          class="btn btn-accent"
          @click="clearFilters"
        >
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
    name: 'RecipeFilters',
    components: {
        Multiselect,
    },
    inject: ["store", "icons"],
    props: {
        filters : {
            type: Object,
        }
    },
    emits: ['toggle', 'update:filters'],
    data: function() {
        return {
            timer: null,
            expanded: !this.on_mobile(),
            innerWidth_cached: window.innerWidth,
        }
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
                return this.filters.tags;
            },
            set: function(val) {
                let new_filters = {...this.filters, 'tags': val};
                this.updateFilters(new_filters, 0);
            }
        },
        categories: {
            get: function() {
                return this.filters.categories;
            },
            set: function(val) {
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
    }
}
</script>
