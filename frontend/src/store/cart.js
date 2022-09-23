import { reactive, readonly } from 'vue'

const cart = () => {
    const content = reactive(new Map())

    const containsId = (r_id) => { return content.has(r_id)
    }
    const addRecipe = (recipe, shares) => {
        content.set(recipe.id, { 'recipe': recipe, 'shares': shares })
        console.log(content)
    }
    const removeRecipeWithId = (r_id) => {
        content.delete(r_id)
    }
    const recipeCount = () => {
        return content.size
    }
    const updateShares = (r_id, shares) => {
        content.get(r_id).shares = shares
    }

    return {
        content: readonly(content),
        containsId,
        addRecipe,
        removeRecipeWithId,
        recipeCount,
        updateShares
    }
}

export default cart();
