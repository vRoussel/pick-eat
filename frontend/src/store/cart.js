import { ref, computed } from 'vue'
import { defineStore } from 'pinia'

import { useNotifStore } from '@/store/notif.js'
import { useFoodStore } from '@/store/food.js'

export const useCartStore = defineStore('cart', () => {
    const notifStore = useNotifStore()
    const foodStore = useFoodStore()

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
        backup()
    }

    function removeRecipe(r_id) {
        content.value.delete(r_id)
        backup()
    }

    function hasRecipe(r_id) {
        return content.value.has(r_id)
    }

    function updateRecipeShares(r_id, shares) {
        content.value.get(r_id).shares = shares
        backup()
    }

    function updateRecipe(id, new_recipe, should_backup = true) {
        content.value.get(id).recipe = new_recipe
        if (should_backup)
            backup()
    }

    function backup() {
        localStorage.setItem("cart", JSON.stringify([...content.value]))
    }

    async function restore() {
        try {
            let saved = new Map(JSON.parse(localStorage.getItem("cart")))
            if (saved) {
                content.value = saved
            }
        } catch {
            console.error("Unable to restore cart from local storage")
        }

        try {
            let ids = Array.from(content.value.keys())
            let up_to_date_recipes = await foodStore.getRecipesFromIds(ids)
            for (const r of up_to_date_recipes) {
                updateRecipe(r.id, r, should_backup = false)
            }
            backup()
        } catch (e) {
            console.error("Unable to update cart recipes from API")
        }
    }

    return { content, recipeCount, addRecipe, removeRecipe, hasRecipe, updateRecipe, updateRecipeShares, restore }
})

