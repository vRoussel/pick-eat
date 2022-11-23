import { defineStore } from 'pinia'
import {insert_sorted} from '@/utils/utils.js'
import axios from 'axios';

const API_PROTO = window.location.protocol
const API_HOST = window.location.hostname
const API_ROOT = `${API_PROTO}//${API_HOST}/api/v1`

export const useApiStore = defineStore('api', {
    state: () => {
        return {
            tags: [],
            categories: [],
            seasons: [],
            ingredients: [],
            units: []
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
            let headers = {
                'Accept': 'application/json',
                'Content-Type': 'application/json;charset=UTF-8'
            }

            let resp = await axios.post(`${API_ROOT}/tags`, tag, { 'headers': headers })
            let location = resp.headers['location']

            let resp2 = await axios.get(`${API_ROOT}/tags${location}`)
            let new_tag = resp2.data
            insert_sorted(this.tags, new_tag, (a,b) => a.name.localeCompare(b.name))
            return new_tag
        },

        async sendNewCategory(category) {
            let headers = {
                'Accept': 'application/json',
                'Content-Type': 'application/json;charset=UTF-8'
            }

            let resp = await axios.post(`${API_ROOT}/categories`, category, { 'headers': headers })
            let location = resp.headers['location']

            let resp2 = await axios.get(`${API_ROOT}/categories${location}`)
            let new_categ = resp2.data
            insert_sorted(this.categories, new_categ, (a,b) => a.name.localeCompare(b.name))
            return new_categ
        },

        async sendNewSeason(season) {
            let headers = {
                'Accept': 'application/json',
                'Content-Type': 'application/json;charset=UTF-8'
            }

            let resp = await axios.post(`${API_ROOT}/seasons`, season, { 'headers': headers })
            let location = resp.headers['location']

            let resp2 = await axios.get(`${API_ROOT}/seasons${location}`)
            let new_season = resp2.data
            insert_sorted(this.seasons, new_season, (a,b) => a.name.localeCompare(b.name))
            return new_season
        },

        async sendNewIngredient(ingredient) {
            let headers = {
                'Accept': 'application/json',
                'Content-Type': 'application/json;charset=UTF-8'
            }

            let resp = await axios.post(`${API_ROOT}/ingredients`, ingredient, { 'headers': headers })
            let location = resp.headers['location']

            let resp2 = await axios.get(`${API_ROOT}/ingredients${location}`)
            let new_ingredient = resp2.data
            insert_sorted(this.ingredients, new_ingredient, (a,b) => a.name.localeCompare(b.name))
            return new_ingredient
        },

        async sendNewUnit(unit) {
            let headers = {
                'Accept': 'application/json',
                'Content-Type': 'application/json;charset=UTF-8'
            }

            let resp = await axios.post(`${API_ROOT}/units`, unit, { 'headers': headers })
            let location = resp.headers['location']

            let resp2 = await axios.get(`${API_ROOT}/units${location}`)
            let new_unit = resp2.data
            insert_sorted(this.units, new_unit, (a,b) => a.full_name.localeCompare(b.full_name))
            return new_unit
        },

        async sendNewRecipe(recipe) {
            let headers = {
                'Accept': 'application/json',
                'Content-Type': 'application/json;charset=UTF-8'
            }

            let resp = await axios.post(`${API_ROOT}/recipes`, recipe, { 'headers': headers })
            let location = resp.headers['location']

            let resp2 = await axios.get(`${API_ROOT}/recipes${location}`)
            let new_recipe = resp2.data
            return new_recipe
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
            let body = { 'is_favorite': recipe.is_favorite }

            return axios.patch(`${API_ROOT}/recipes/${recipe.id}`, body, { 'headers': headers }).catch((e) => {
                recipe.is_favorite = !recipe.is_favorite
                throw e
            })
        }
    }
})
