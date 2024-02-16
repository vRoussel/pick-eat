import { ref, computed } from 'vue'
import { defineStore } from 'pinia'
import axios from 'axios'

import { insert_sorted } from '@/utils/utils.js'

const API_PROTO = window.location.protocol
const API_HOST = window.location.hostname
const API_ROOT = `${API_PROTO}//${API_HOST}/api/v1`

async function sendNewThing(post, endpoint) {
    let headers = {
        Accept: 'application/json',
        'Content-Type': 'application/json;charset=UTF-8',
    }

    let resp = await axios.post(`${API_ROOT}${endpoint}`, post, { headers: headers })
    let location = resp.headers['location']

    let resp2 = await axios.get(`${API_ROOT}${endpoint}${location}`)
    return resp2.data
}

export const useFoodStore = defineStore('food', () => {
    const tags = ref([])
    const categories = ref([])
    const seasons = ref([])
    const ingredients = ref([])
    const units = ref([])
    const diets = ref([])
    const accounts_with_recipes = ref([])



    const tagsById = computed(() => {
        return new Map(tags.value.map((t) => [t.id, t]))
    })
    const categoriesById = computed(() => {
        return new Map(categories.value.map((c) => [c.id, c]))
    })
    const seasonsById = computed(() => {
        return new Map(seasons.value.map((s) => [s.id, s]))
    })
    const ingredientsById = computed(() => {
        return new Map(ingredients.value.map((i) => [i.id, i]))
    })
    const unitsById = computed(() => {
        return new Map(units.value.map((u) => [u.id, u]))
    })
    const dietsById = computed(() => {
        return new Map(diets.value.map((d) => [d.id, d]))
    })

    async function fetchTags() {
        return axios.get(`${API_ROOT}/tags`).then((resp) => {
            tags.value = resp.data
        })
    }

    async function fetchCategories() {
        return axios.get(`${API_ROOT}/categories`).then((resp) => {
            categories.value = resp.data
        })
    }

    async function fetchSeasons() {
        return axios.get(`${API_ROOT}/seasons`).then((resp) => {
            seasons.value = resp.data
        })
    }

    async function fetchIngredients() {
        return axios.get(`${API_ROOT}/ingredients`).then((resp) => {
            ingredients.value = resp.data
        })
    }

    async function fetchUnits() {
        return axios.get(`${API_ROOT}/units`).then((resp) => {
            units.value = resp.data
        })
    }

    async function fetchDiets() {
        return axios.get(`${API_ROOT}/diets`).then((resp) => {
            diets.value = resp.data
        })
    }

    async function fetchAccountsWithRecipes() {
        return axios.get(`${API_ROOT}/accounts?withrecipes=true`).then((resp) => {
            accounts_with_recipes.value = resp.data
        })
    }

    async function fetchAll() {
        return axios.get(`${API_ROOT}/bundle`).then((resp) => {
            tags.value = resp.data.tags
            categories.value = resp.data.categories
            ingredients.value = resp.data.ingredients
            units.value = resp.data.units
            seasons.value = resp.data.seasons
            diets.value = resp.data.diets
            accounts_with_recipes.value = resp.data.accounts_with_recipes
        })
    }

    async function getRecipes(from, to, filters, sort_method) {
        let f = filters
        let url = `${API_ROOT}/recipes`
        let params = {
            range: `${from}-${to}`,
            sort: sort_method,
        }
        if (f.search_query) params.search = f.search_query
        if (f.ingredients.length > 0) params.ingredients = f.ingredients.join(',')
        if (f.tags.length > 0) params.tags = f.tags.join(',')
        if (f.categories.length > 0) params.categories = f.categories.join(',')
        if (f.seasons.length > 0) params.seasons = f.seasons.join(',')
        if (f.account) params.account = f.account
        if (f.diets.length > 0) params.diets = f.diets.join(',')
        let resp = await axios.get(url, { params: params })
        let total_count = parseInt(resp.headers['content-range'].split('/')[1])
        return [resp.data, total_count]
    }

    async function getRecipesFromIds(ids) {
        if (ids.length > 25) {
            console.warn("More than 25 recipes asked by ID, this should not happen")
        }
        let url = `${API_ROOT}/recipes`
        let params = {
            range: `1-25`,
            sort: 'random',
        }
        params.ids = ids.join(',')
        let resp = await axios.get(url, { params: params })
        return resp.data
    }

    function getTagById(id) {
        return tagsById.value.get(id)
    }

    function getIngredientById(id) {
        return ingredientsById.value.get(id)
    }

    function getCategoryById(id) {
        return categoriesById.value.get(id)
    }

    function getUnitById(id) {
        return unitsById.value.get(id)
    }

    function getSeasonById(id) {
        return seasonsById.value.get(id)
    }

    function getDietById(id) {
        return dietsById.value.get(id)
    }

    async function getRecipeById(id) {
        let resp = await axios.get(`${API_ROOT}/recipes/${id}`)
        return resp.data
    }

    async function sendNewTag(tag) {
        let new_tag = await sendNewThing(tag, '/tags')
        insert_sorted(tags.value, new_tag, (a, b) => a.name.localeCompare(b.name))
        return new_tag
    }

    async function sendNewCategory(category) {
        let new_categ = await sendNewThing(category, '/categories')
        insert_sorted(categories.value, new_categ, (a, b) => a.name.localeCompare(b.name))
        return new_categ
    }

    async function sendNewSeason(season) {
        let new_season = await sendNewThing(season, '/seasons')
        insert_sorted(seasons.value, new_season, (a, b) => a.name.localeCompare(b.name))
        return new_season
    }

    async function sendNewDiet(diet) {
        let new_diet = await sendNewThing(diet, '/diets')
        insert_sorted(diets.value, new_diet, (a, b) => a.name.localeCompare(b.name))
        return new_diet
    }

    async function sendNewIngredient(ingredient) {
        let new_ingredient = await sendNewThing(ingredient, '/ingredients')
        insert_sorted(ingredients.value, new_ingredient, (a, b) => a.name.localeCompare(b.name))
        return new_ingredient
    }

    async function sendNewUnit(unit) {
        let new_unit = await sendNewThing(unit, '/units')
        insert_sorted(units.value, new_unit, (a, b) => a.full_name.localeCompare(b.full_name))
        return new_unit
    }

    async function sendNewRecipe(recipe) {
        return sendNewThing(recipe, '/recipes')
    }

    async function updateRecipe(id, recipe) {
        let headers = {
            Accept: 'application/json',
            'Content-Type': 'application/json;charset=UTF-8',
        }

        return axios.put(`${API_ROOT}/recipes/${id}`, recipe, { headers: headers })
    }

    async function toggleFavorite(recipe) {
        recipe.is_favorite = !recipe.is_favorite
        let headers = {
            Accept: 'application/json',
            'Content-Type': 'application/json;charset=UTF-8',
        }
        let f = null
        if (recipe.is_favorite) {
            f = axios.put
        } else {
            f = axios.delete
        }
        return f(`${API_ROOT}/accounts/me/favorites/${recipe.id}`, { headers: headers }).catch(
            () => {
                recipe.is_favorite = !recipe.is_favorite
            }
        )
    }

    return { tags, categories, seasons, ingredients, units, diets, accounts_with_recipes, fetchTags, fetchCategories, fetchSeasons, fetchIngredients, fetchUnits, fetchDiets, fetchAccountsWithRecipes, fetchAll, getRecipes, getRecipesFromIds, getTagById, getIngredientById, getCategoryById, getUnitById, getSeasonById, getDietById, getRecipeById, sendNewTag, sendNewCategory, sendNewSeason, sendNewDiet, sendNewIngredient, sendNewUnit, sendNewRecipe, updateRecipe, toggleFavorite }
})
