import { ref, computed } from 'vue'
import { defineStore } from 'pinia'

import { useNotifStore } from '@/store/notif.js'

export const useCartStore = defineStore('cart', () => {
    const notifStore = useNotifStore()
    const content = ref(new Map())

    let recipeCount = computed(() => {
        return content.value.size
    })

    function addRecipe(recipe, shares) {
        if (recipeCount.value >= 25) {
            notifStore.show_error("Vous ne pouvez pas ajouter plus de 25 recettes à votre panier")
            return
        }
        content.value.set(recipe.id, { recipe: recipe, shares: shares })
        save_cart(content.value)
    }

    function removeRecipe(r_id) {
        content.value.delete(r_id)
        save_cart(content.value)
    }

    function hasRecipe(r_id) {
        return content.value.has(r_id)
    }

    function updateRecipeShares(r_id, shares) {
        content.value.get(r_id).shares = shares
        save_cart(content.value)
    }

    function restore() {
        try {
            let saved = new Map(JSON.parse(localStorage.getItem("cart")))
            if (saved)
                content.value = saved
        } catch {
            console.error("Unable to restore cart from local storage")
        }
    }

    return { content, recipeCount, addRecipe, removeRecipe, hasRecipe, updateRecipeShares, restore }
})

function save_cart(content) {
    localStorage.setItem("cart", JSON.stringify([...content]))
}
