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
    return fetch(`${API_ROOT}/tags`)
        .then(response => response.json())
        .then(json => {
            state.tags = json;
        });
}

const getCategories = function() {
    fetch(`${API_ROOT}/categories`)
        .then(response => response.json())
        .then(json => {
            state.categories = json;
        });
}

const getIngredients = function() {
    fetch(`${API_ROOT}/ingredients`)
        .then(response => response.json())
        .then(json => {
            state.ingredients = json;
        });
}

const getUnits = function() {
    fetch(`${API_ROOT}/units`)
        .then(response => response.json())
        .then(json => {
            state.units = json;
        });
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
