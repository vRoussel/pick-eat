import { defineStore } from 'pinia'

export const useCartStore = defineStore('cart', {
    state: () => {
        return {
            content: new Map(),
        }
    },
    getters: {
        recipeCount(state) {
            return state.content.size
        },
    },
    actions: {
        addRecipe(recipe, shares) {
            this.content.set(recipe.id, { recipe: recipe, shares: shares })
        },

        removeRecipe(r_id) {
            this.content.delete(r_id)
        },

        hasRecipe(r_id) {
            return this.content.has(r_id)
        },

        updateRecipeShares(r_id, shares) {
            this.content.get(r_id).shares = shares
        },
    },
})
