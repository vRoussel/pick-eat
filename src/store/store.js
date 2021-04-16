import { reactive, readonly } from 'vue'

const state =  reactive ({
    tags: [],
    categories: [],
    ingredients: [],
    units: []
});

const API_HOST = '192.168.1.59';
const API_ROOT = `http://${API_HOST}/api/v1`

const getTags = async function() {
    let ret = await fetch(`${API_ROOT}/tags`)
    if (ret.ok) {
        state.tags = await ret.json()
        return ret
    }
    else
        throw ret
}

const getCategories = async function() {
    let ret = await fetch(`${API_ROOT}/categories`)
    if (ret.ok) {
        state.categories = await ret.json()
        return ret
    }
    else
        throw ret
}

const getIngredients = async function() {
    let ret = await fetch(`${API_ROOT}/ingredients`)
    if (ret.ok) {
        state.ingredients = await ret.json()
        return ret
    }
    else
        throw ret
}

const getUnits = async function() {
    let ret = await fetch(`${API_ROOT}/units`)
    if (ret.ok) {
        state.units = await ret.json()
        return ret
    }
    else
        throw ret
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
    if (ret.ok)
        return ret
    else
        throw ret
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
    addRecipe,
    addTag,
}
