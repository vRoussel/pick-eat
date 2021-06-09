import { reactive, readonly } from 'vue'

const state =  reactive ({
    tags: [],
    categories: [],
    seasons: [],
    ingredients: [],
    units: []
});

const API_HOST = '192.168.1.60';
const API_ROOT = `http://${API_HOST}/api/v1`

const getRecipes = async function(from, to) {
    let ret = await fetch(`${API_ROOT}/recipes?range=${from}-${to}`)
    if (ret.ok) {
        console.log(ret)
        console.log(ret.headers.get('content-range'))
        let json = await ret.json()
        console.log(json)
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

const getSeasons = async function() {
    let ret = await fetch(`${API_ROOT}/seasons`)
    if (ret.ok) {
        state.seasons = await ret.json()
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

const addCategory = async function(tag) {
    const options = {
        method: 'POST',
        headers: {
            'Accept': 'application/json',
            'Content-Type': 'application/json;charset=UTF-8'
        },
        body: JSON.stringify(tag)
    };
    console.debug(options.body);
    let ret = await fetch(`${API_ROOT}/categories`, options)
    if (ret.ok)
        return ret
    else
        throw ret
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
    if (ret.ok)
        return ret
    else
        throw ret
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
    if (ret.ok)
        return ret
    else
        throw ret
}

const toggleFavorite = async function(recipe) {
    recipe.is_favorite = !recipe.is_favorite
    const options = {
        method: 'PATCH',
        headers: {
            'Accept': 'application/json',
            'Content-Type': 'application/json;charset=UTF-8'
        },
        body: JSON.stringify({'is_favorite': !recipe.is_favorite})
    };
    console.debug(options.body);
    let ret = await fetch(`${API_ROOT}/recipes/${recipe.id}`, options)
    if (ret.ok) {
        return ret
    } else
        recipe.is_favorite = !recipe.is_favorite
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
    addRecipe,
    addTag,
    addCategory,
    addIngredient,
    addUnit,
    toggleFavorite,
    getOneRecipe,
}
