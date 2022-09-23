import { reactive, readonly } from 'vue'
import {insert_sorted} from '@/utils/utils.js'

//TODO split this in multiple files (api, store, cart)

const state =  reactive ({
    tags: [],
    categories: [],
    seasons: [],
    ingredients: [],
    units: []
});

const internal = {
    tags_by_id: {},
    categories_by_id: {},
    seasons_by_id: {},
    ingredients_by_id: {},
    units_by_id: {}
}

const API_PROTO = window.location.protocol
const API_HOST = window.location.hostname
const API_ROOT = `${API_PROTO}//${API_HOST}/api/v1`

const getRecipes = async function(from, to, filters) {
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
}

const getOneRecipe = async function(id) {
    let ret = await fetch(`${API_ROOT}/recipes/${id}`)
    if (ret.ok) {
        let json = await ret.json()
        return json
    }
}

const getTags = async function() {
    let ret = await fetch(`${API_ROOT}/tags`)
    if (ret.ok) {
        state.tags = await ret.json()
        internal.tags_by_id = new Map(state.tags.map(t => [t.id, t.name]))
        return ret
    }
    else
        throw ret
}

const getCategories = async function() {
    let ret = await fetch(`${API_ROOT}/categories`)
    if (ret.ok) {
        state.categories = await ret.json()
        internal.categories_by_id = new Map(state.categories.map(c => [c.id, c.name]))
        return ret
    }
    else
        throw ret
}

const getSeasons = async function() {
    let ret = await fetch(`${API_ROOT}/seasons`)
    if (ret.ok) {
        state.seasons = await ret.json()
        internal.seasons_by_id = new Map(state.seasons.map(s => [s.id, s.name]))
        return ret
    }
    else
        throw ret
}

const getIngredients = async function() {
    let ret = await fetch(`${API_ROOT}/ingredients`)
    if (ret.ok) {
        state.ingredients = await ret.json()
        internal.ingredients_by_id = new Map(state.ingredients.map(i => [i.id, i.name]))
        return ret
    }
    else
        throw ret
}

const getUnits = async function() {
    let ret = await fetch(`${API_ROOT}/units`)
    if (ret.ok) {
        state.units = await ret.json()
        internal.units_by_id = new Map(state.units.map(u => [u.id, u.short_name]))
        return ret
    }
    else
        throw ret
}

const getTagById = function(id) {
    return internal.tags_by_id.get(id)
}

const getIngredientById = function(id) {
    return internal.ingredients_by_id.get(id)
}

const getCategoryById = function(id) {
    return internal.categories_by_id.get(id)
}

const getUnitById = function(id) {
    return internal.units_by_id.get(id)
}

const addRecipe = async function(recipe) {
    const options = {
        method: 'POST',
        headers: {
            'Accept': 'application/json',
            'Content-Type': 'application/json;charset=UTF-8'
        },
        body: JSON.stringify(recipe)
    };
    console.debug(options.body);
    let ret = await fetch(`${API_ROOT}/recipes`, options)
    console.log(ret)
    if (!ret.ok)
        throw ret

    let location = ret.headers.get('location')
    let id = parseInt(location.substring(1))
    return {id: id}
}

const addTag = async function(tag) {
    const options = {
        method: 'POST',
        headers: {
            'Accept': 'application/json',
            'Content-Type': 'application/json;charset=UTF-8'
        },
        body: JSON.stringify(tag)
    };
    console.debug(options.body);
    let ret = await fetch(`${API_ROOT}/tags`, options)
    if (!ret.ok)
        throw ret

    let location = ret.headers.get('location')
    let ret2 = await fetch(`${API_ROOT}/tags${location}`)
    if (!ret2.ok)
        throw ret

    let new_tag = await ret2.json()
    insert_sorted(state.tags, new_tag, (a,b) => a.name.localeCompare(b.name))
    internal.tags_by_id.set(new_tag.id, new_tag.name)
    return new_tag
}

const addCategory = async function(category) {
    const options = {
        method: 'POST',
        headers: {
            'Accept': 'application/json',
            'Content-Type': 'application/json;charset=UTF-8'
        },
        body: JSON.stringify(category)
    };
    console.debug(options.body);
    let ret = await fetch(`${API_ROOT}/categories`, options)
    if (!ret.ok)
        throw ret

    let location = ret.headers.get('location')
    let ret2 = await fetch(`${API_ROOT}/categories${location}`)
    if (!ret2.ok)
        throw ret

    let new_categ = await ret2.json()
    insert_sorted(state.categories, new_categ, (a,b) => a.name.localeCompare(b.name))
    internal.categories_by_id.set(new_categ.id, new_categ.name)
    return new_categ
}

const addIngredient = async function(ingredient) {
    const options = {
        method: 'POST',
        headers: {
            'Accept': 'application/json',
            'Content-Type': 'application/json;charset=UTF-8'
        },
        body: JSON.stringify(ingredient)
    };
    console.debug(options.body);
    let ret = await fetch(`${API_ROOT}/ingredients`, options)
    if (!ret.ok)
        throw ret

    let location = ret.headers.get('location')
    let ret2 = await fetch(`${API_ROOT}/ingredients${location}`)
    if (!ret2.ok)
        throw ret

    let new_ingr = await ret2.json()
    insert_sorted(state.ingredients, new_ingr, (a,b) => a.name.localeCompare(b.name))
    internal.ingredients_by_id.set(new_ingr.id, new_ingr.name)
    return new_ingr
}

const addUnit = async function(unit) {
    const options = {
        method: 'POST',
        headers: {
            'Accept': 'application/json',
            'Content-Type': 'application/json;charset=UTF-8'
        },
        body: JSON.stringify(unit)
    };
    console.debug(options.body);
    let ret = await fetch(`${API_ROOT}/units`, options)
    if (!ret.ok)
        throw ret

    let location = ret.headers.get('location')
    let ret2 = await fetch(`${API_ROOT}/units${location}`)
    if (!ret2.ok)
        throw ret

    let new_unit = await ret2.json()
    insert_sorted(state.units, new_unit, (a,b) => a.full_name.localeCompare(b.full_name))
    internal.units_by_id.set(new_unit.id, new_unit.short_name)
    return new_unit
}

const toggleFavorite = async function(recipe) {
    recipe.is_favorite = !recipe.is_favorite
    const options = {
        method: 'PATCH',
        headers: {
            'Accept': 'application/json',
            'Content-Type': 'application/json;charset=UTF-8'
        },
        body: JSON.stringify({'is_favorite': recipe.is_favorite})
    };
    console.debug(options.body);
    let ret = await fetch(`${API_ROOT}/recipes/${recipe.id}`, options)
    if (ret.ok) {
        return ret
    } else
        recipe.is_favorite = !recipe.is_favorite
        throw ret

}

const updateRecipe = async function(id, recipe) {
    const options = {
        method: 'PUT',
        headers: {
            'Accept': 'application/json',
            'Content-Type': 'application/json;charset=UTF-8'
        },
        body: JSON.stringify(recipe)
    };
    console.debug(options.body);
    let ret = await fetch(`${API_ROOT}/recipes/${id}`, options)
    if (ret.ok)
        return ret
    else
        throw ret
}

export default {
    state: readonly(state),
    getTags,
    getIngredients,
    getCategories,
    getUnits,
    getSeasons,
    getRecipes,
    getTagById,
    getIngredientById,
    getCategoryById,
    getUnitById,
    addRecipe,
    addTag,
    addCategory,
    addIngredient,
    addUnit,
    toggleFavorite,
    getOneRecipe,
    updateRecipe,
}
