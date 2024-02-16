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
            save_cart(this.content)
        },

        removeRecipe(r_id) {
            this.content.delete(r_id)
            save_cart(this.content)
        },

        hasRecipe(r_id) {
            return this.content.has(r_id)
        },

        updateRecipeShares(r_id, shares) {
            this.content.get(r_id).shares = shares
            save_cart(this.content)
        },

        restore() {
            try {
                let saved = new Map(JSON.parse(localStorage.getItem("cart")))
                if (saved)
                    this.content = saved
            } catch {
                console.error("Unable to restore cart from local storage")
            }
        }
    },
})

function save_cart(content) {
    localStorage.setItem("cart", JSON.stringify([...content]))
}
