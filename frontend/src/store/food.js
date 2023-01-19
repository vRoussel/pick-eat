import { defineStore } from 'pinia'
import {insert_sorted} from '@/utils/utils.js'
import axios from 'axios';

const API_PROTO = window.location.protocol
const API_HOST = window.location.hostname
const API_ROOT = `${API_PROTO}//${API_HOST}/api/v1`

async function sendNewThing(post, endpoint) {
    let headers = {
        'Accept': 'application/json',
        'Content-Type': 'application/json;charset=UTF-8'
    }

    let resp = await axios.post(`${API_ROOT}${endpoint}`, post, { 'headers': headers })
    let location = resp.headers['location']

    let resp2 = await axios.get(`${API_ROOT}${endpoint}${location}`)
    return resp2.data
}

export const useFoodStore = defineStore('food', {
    state: () => {
        return {
            tags: [],
            categories: [],
            seasons: [],
            ingredients: [],
            units: [],
            accounts_with_recipes: []
        }
    },
    getters: {
        tagsById(state) {
            return new Map(state.tags.map(t => [t.id, t]))
        },
        categoriesById(state) {
            return new Map(state.categories.map(c => [c.id, c]))
        },
        seasonsById(state) {
            return new Map(state.seasons.map(s => [s.id, s]))
        },
        ingredientsById(state) {
            return new Map(state.ingredients.map(i => [i.id, i]))
        },
        unitsById(state) {
            return new Map(state.units.map(u => [u.id, u]))
        }
    },
    actions: {
        async fetchTags() {
            return axios.get(`${API_ROOT}/tags`).then(resp => {
                this.tags = resp.data
            })
        },

        async fetchCategories() {
            return axios.get(`${API_ROOT}/categories`).then(resp => {
                this.categories = resp.data
            })
        },

        async fetchSeasons() {
            return axios.get(`${API_ROOT}/seasons`).then(resp => {
                this.seasons = resp.data
            })
        },

        async fetchIngredients() {
            return axios.get(`${API_ROOT}/ingredients`).then(resp => {
                this.ingredients = resp.data
            })
        },

        async fetchUnits() {
            return axios.get(`${API_ROOT}/units`).then(resp => {
                this.units = resp.data
            })
        },

        async fetchAccountsWithRecipes() {
            return axios.get(`${API_ROOT}/accounts?withrecipes=true`).then(resp => {
                this.accounts_with_recipes = resp.data
            })
        },

        async getRecipes(from, to, filters) {
            let f = filters;
            let url = `${API_ROOT}/recipes`
            let params = {
                'range': `${from}-${to}`
            }
            if (f.search_query)
                params.search = f.search_query
            if (f.ingredients.length > 0)
                params.ingredients = f.ingredients.join(',')
            if (f.tags.length > 0)
                params.tags = f.tags.join(',')
            if (f.categories.length > 0)
                params.categories = f.categories.join(',')
            if (f.seasons.length > 0)
                params.seasons = f.seasons.join(',')
            if (f.account)
                params.account = f.account
            let resp = await axios.get(url, { 'params': params })
            let total_count = parseInt(resp.headers['content-range'].split('/')[1])
            return [resp.data, total_count]
        },



        getTagById(id) {
            return this.tagsById.get(id)
        },

        getIngredientById(id) {
            return this.ingredientsById.get(id)
        },

        getCategoryById(id) {
            return this.categoriesById.get(id)
        },

        getUnitById(id) {
            return this.unitsById.get(id)
        },

        getSeasonById(id) {
            return this.seasonsById.get(id)
        },

        async getRecipeById(id) {
            let resp = await axios.get(`${API_ROOT}/recipes/${id}`)
            return resp.data
        },

        async sendNewTag(tag) {
            let new_tag = await sendNewThing(tag, '/tags')
            insert_sorted(this.tags, new_tag, (a,b) => a.name.localeCompare(b.name))
            return new_tag
        },

        async sendNewCategory(category) {
            let new_categ = await sendNewThing(category, '/categories')
            insert_sorted(this.categories, new_categ, (a,b) => a.name.localeCompare(b.name))
            return new_categ
        },

        async sendNewSeason(season) {
            let new_season = await sendNewThing(season, '/seasons')
            insert_sorted(this.seasons, new_season, (a,b) => a.name.localeCompare(b.name))
            return new_season
        },

        async sendNewIngredient(ingredient) {
            let new_ingredient = await sendNewThing(ingredient, '/ingredients')
            insert_sorted(this.ingredients, new_ingredient, (a,b) => a.name.localeCompare(b.name))
            return new_ingredient
        },

        async sendNewUnit(unit) {
            let new_unit = await sendNewThing(unit, '/units')
            insert_sorted(this.units, new_unit, (a,b) => a.full_name.localeCompare(b.full_name))
            return new_unit
        },

        async sendNewRecipe(recipe) {
            return sendNewThing(recipe, '/recipes')
        },



        async updateRecipe(id, recipe) {
            let headers = {
                'Accept': 'application/json',
                'Content-Type': 'application/json;charset=UTF-8'
            }

            return axios.put(`${API_ROOT}/recipes/${id}`, recipe, { 'headers': headers })
        },

        async toggleFavorite(recipe) {
            recipe.is_favorite = !recipe.is_favorite
            let headers = {
                'Accept': 'application/json',
                'Content-Type': 'application/json;charset=UTF-8'
            }
            let f = null
            if (recipe.is_favorite) {
                f = axios.put
            } else {
                f= axios.delete
            }
            return f(`${API_ROOT}/accounts/me/favorites/${recipe.id}`, { 'headers': headers }).catch(() => {
                recipe.is_favorite = !recipe.is_favorite
            })
        }
    }
})
