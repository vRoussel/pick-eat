import { ref, computed } from 'vue'
import { defineStore } from 'pinia'
import debounce from 'lodash.debounce'
import isEqual from 'lodash.isequal'

import { useNotifStore } from '@/store/notif.js'
import { useAuthStore } from '@/store/auth.js'

export const useCartStore = defineStore('cart', () => {
    const notifStore = useNotifStore()
    const authStore = useAuthStore()

    const content = ref(new Map())
    const last_update = ref(null)

    let recipeCount = computed(() => {
        return content.value.size
    })

    function addRecipe(recipe, shares) {
        if (recipeCount.value >= 25) {
            notifStore.show_error('Vous ne pouvez pas ajouter plus de 25 recettes à votre panier')
            return
        }
        content.value.set(recipe.id, { recipe: recipe, shares: shares })
        last_update.value = Date.now()
        do_backup()
    }

    function removeRecipe(r_id) {
        content.value.delete(r_id)
        last_update.value = Date.now()
        do_backup()
    }

    function hasRecipe(r_id) {
        return content.value.has(r_id)
    }

    function toggleRecipe(recipe, shares = undefined) {
        if (hasRecipe(recipe.id)) {
            removeRecipe(recipe.id)
        } else {
            addRecipe(recipe, shares || recipe.n_shares)
        }
    }

    function updateRecipeShares(r_id, shares) {
        content.value.get(r_id).shares = shares
        last_update.value = Date.now()
        do_backup()
    }

    function updateRecipe(id, new_recipe, should_backup = true) {
        content.value.get(id).recipe = new_recipe
        if (should_backup) do_backup()
    }

    var do_backup_api_debounced = debounce(async (data) => {
        await authStore.save_account_data('grocery_list', data)
    }, 5000)

    async function do_backup() {
        let data = JSON.stringify({ content: [...content.value], last_update: last_update.value })
        do_backup_local(data)
        if (authStore.is_logged_in) {
            do_backup_api_debounced(data)
        }
    }

    function do_backup_local(data) {
        localStorage.setItem('cart', data)
    }

    async function do_restore() {
        let backup, local_backup, api_backup
        try {
            local_backup = get_local_backup()
        } catch (e) {
            local_backup = null
            console.error('Unable to restore cart from local storage and/or api')
        }

        if (authStore.is_logged_in) {
            try {
                api_backup = await authStore.get_account_data('grocery_list')
                api_backup.content = new Map(api_backup.content)
            } catch (e) {
                api_backup = null
                console.error('Unable to restore cart from local storage and/or api')
            }
        }

        let backups_merged = false
        if (isEqual(local_backup, api_backup)) {
            backup = local_backup
        } else {
            backup = merge_backups(local_backup, api_backup)
            backups_merged = true
        }

        if (backup) {
            content.value = backup.content
            last_update.value = backup.last_update
            if (backups_merged) {
                do_backup()
            }
        }
    }

    function get_local_backup() {
        let tmp = JSON.parse(localStorage.getItem('cart'))
        tmp.content = new Map(tmp.content)
        return tmp
    }

    function merge_backups(cart1, cart2) {
        let merged_last_update
        let merged_content
        if (!cart1 && !cart2) return null
        else if (!cart1) return cart2
        else if (!cart2) return cart1

        merged_last_update = Math.max(cart1.last_update, cart2.last_update)
        if (cart1.last_update >= cart2.last_update) {
            merged_content = new Map([...cart2.content, ...cart1.content])
        } else {
            merged_content = new Map([...cart1.content, ...cart2.content])
        }

        return {
            last_update: merged_last_update,
            content: merged_content,
        }
    }

    return {
        content,
        recipeCount,
        addRecipe,
        removeRecipe,
        hasRecipe,
        toggleRecipe,
        updateRecipe,
        updateRecipeShares,
        restore: do_restore,
    }
})
