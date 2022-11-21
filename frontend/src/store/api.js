import { defineStore } from 'pinia'
import {insert_sorted} from '@/utils/utils.js'

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
            let ret = await fetch(`${API_ROOT}/tags`)
            if (ret.ok) {
                this.tags = await ret.json()
                return ret
            }
            else
                throw ret
        },

        async fetchCategories() {
            let ret = await fetch(`${API_ROOT}/categories`)
            if (ret.ok) {
                this.categories = await ret.json()
                return ret
            }
            else
                throw ret
        },

        async fetchSeasons() {
            let ret = await fetch(`${API_ROOT}/seasons`)
            if (ret.ok) {
                this.seasons = await ret.json()
                return ret
            }
            else
                throw ret
        },

        async fetchIngredients() {
            let ret = await fetch(`${API_ROOT}/ingredients`)
            if (ret.ok) {
                this.ingredients = await ret.json()
                return ret
            }
            else
                throw ret
        },

        async fetchUnits() {
            let ret = await fetch(`${API_ROOT}/units`)
            if (ret.ok) {
                this.units = await ret.json()
                return ret
            }
            else
                throw ret
        },

        async getRecipes(from, to, filters) {
            let ret = null;
            let f = filters;
            let url = (`${API_ROOT}/recipes?range=${from}-${to}`)
            if (f.search_query)
                url += `&search=${f.search_query}`
            if (f.ingredients.length > 0)
                url += `&ingredients=${f.ingredients.join(',')}`
            if (f.tags.length > 0)
                url += `&tags=${f.tags.join(',')}`
            if (f.categories.length > 0)
                url += `&categories=${f.categories.join(',')}`
            if (f.seasons.length > 0)
                url += `&seasons=${f.seasons.join(',')}`
            ret = await fetch(url)
            if (ret.ok) {
                let json = await ret.json()
                let total_count = parseInt(ret.headers.get('content-range').split('/')[1])
                return [json, total_count]
            }
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
            let ret = await fetch(`${API_ROOT}/recipes/${id}`)
            if (ret.ok) {
                let json = await ret.json()
                return json
            }
        },



        async sendNewTag(tag) {
            const options = {
                method: 'POST',
                headers: {
                    'Accept': 'application/json',
                    'Content-Type': 'application/json;charset=UTF-8'
                },
                body: JSON.stringify(tag)
            };
            let ret = await fetch(`${API_ROOT}/tags`, options)
            if (!ret.ok)
                throw ret

            let location = ret.headers.get('location')
            let ret2 = await fetch(`${API_ROOT}/tags${location}`)
            if (!ret2.ok)
                throw ret2

            let new_tag = await ret2.json()
            insert_sorted(this.tags, new_tag, (a,b) => a.name.localeCompare(b.name))
            return new_tag
        },

        async sendNewCategory(category) {
            const options = {
                method: 'POST',
                headers: {
                    'Accept': 'application/json',
                    'Content-Type': 'application/json;charset=UTF-8'
                },
                body: JSON.stringify(category)
            };
            let ret = await fetch(`${API_ROOT}/categories`, options)
            if (!ret.ok)
                throw ret

            let location = ret.headers.get('location')
            let ret2 = await fetch(`${API_ROOT}/categories${location}`)
            if (!ret2.ok)
                throw ret2

            let new_category = await ret2.json()
            insert_sorted(this.categories, new_category, (a,b) => a.name.localeCompare(b.name))
            return new_category
        },

        async sendNewSeason(season) {
            const options = {
                method: 'POST',
                headers: {
                    'Accept': 'application/json',
                    'Content-Type': 'application/json;charset=UTF-8'
                },
                body: JSON.stringify(season)
            };
            let ret = await fetch(`${API_ROOT}/seasons`, options)
            if (!ret.ok)
                throw ret

            let location = ret.headers.get('location')
            let ret2 = await fetch(`${API_ROOT}/seasons${location}`)
            if (!ret2.ok)
                throw ret2

            let new_season = await ret2.json()
            insert_sorted(this.seasons, new_season, (a,b) => a.name.localeCompare(b.name))
            return new_season
        },

        async sendNewIngredient(ingredient) {
            const options = {
                method: 'POST',
                headers: {
                    'Accept': 'application/json',
                    'Content-Type': 'application/json;charset=UTF-8'
                },
                body: JSON.stringify(ingredient)
            };
            let ret = await fetch(`${API_ROOT}/ingredients`, options)
            if (!ret.ok)
                throw ret

            let location = ret.headers.get('location')
            let ret2 = await fetch(`${API_ROOT}/ingredients${location}`)
            if (!ret2.ok)
                throw ret2

            let new_ingredient = await ret2.json()
            insert_sorted(this.ingredients, new_ingredient, (a,b) => a.name.localeCompare(b.name))
            return new_ingredient
        },

        async sendNewUnit(unit) {
            const options = {
                method: 'POST',
                headers: {
                    'Accept': 'application/json',
                    'Content-Type': 'application/json;charset=UTF-8'
                },
                body: JSON.stringify(unit)
            };
            let ret = await fetch(`${API_ROOT}/units`, options)
            if (!ret.ok)
                throw ret

            let location = ret.headers.get('location')
            let ret2 = await fetch(`${API_ROOT}/units${location}`)
            if (!ret2.ok)
                throw ret2

            let new_unit = await ret2.json()
            insert_sorted(this.units, new_unit, (a,b) => a.full_name.localeCompare(b.full_name))
            return new_unit
        },

        async sendNewRecipe(recipe) {
            const options = {
                method: 'POST',
                headers: {
                    'Accept': 'application/json',
                    'Content-Type': 'application/json;charset=UTF-8'
                },
                body: JSON.stringify(recipe)
            };
            let ret = await fetch(`${API_ROOT}/recipes`, options)
            if (!ret.ok)
                throw ret

            let location = ret.headers.get('location')
            let id = parseInt(location.substring(1))
            //TODO this seems bad
            return {id: id}
        },



        async updateRecipe(id, recipe) {
            const options = {
                method: 'PUT',
                headers: {
                    'Accept': 'application/json',
                    'Content-Type': 'application/json;charset=UTF-8'
                },
                body: JSON.stringify(recipe)
            };
            let ret = await fetch(`${API_ROOT}/recipes/${id}`, options)
            if (ret.ok)
                return ret
            else
                throw ret
        },

        async toggleFavorite(recipe) {
            recipe.is_favorite = !recipe.is_favorite
            const options = {
                method: 'PATCH',
                headers: {
                    'Accept': 'application/json',
                    'Content-Type': 'application/json;charset=UTF-8'
                },
                body: JSON.stringify({'is_favorite': recipe.is_favorite})
            };
            let ret = await fetch(`${API_ROOT}/recipes/${recipe.id}`, options)

            if (!ret.ok) {
                recipe.is_favorite = !recipe.is_favorite
                throw ret
            }
            return ret
        }
    }
})
