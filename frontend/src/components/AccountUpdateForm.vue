<template>
    <form class="mx-auto space-y-4 p-8 border-primary border-[1px] rounded-xl max-w-md" @submit.prevent="updateAccount">
        <h1 class="text-xl font-bold text-center">Modification du compte</h1>
        <div class="form-control">
            <label class="label">
                <span class="label-text">Mot de passe actuel</span>
            </label>
            <input
                v-model="old_password"
                type="password"
                placeholder="Requis"
                class="input input-bordered w-full"
            >
        </div>
        <div class="form-control">
            <label class="label">
                <span class="label-text">Nom</span>
            </label>
            <input
                v-model="name"
                type="text"
                class="input input-bordered w-full"
            >
        </div>
        <div class="form-control">
            <label class="label">
                <span class="label-text">Adresse mail</span>
            </label>
            <input
                v-model="email"
                type="text"
                class="input input-bordered w-full"
            >
        </div>
        <div class="form-control">
            <label class="label">
                <span class="label-text">Nouveau mot de passe (si changement)</span>
            </label>
            <input
                v-model="password"
                type="password"
                placeholder="Laisser vide pour conserver l'actuel"
                class="input input-bordered w-full"
            >
        </div>
        <button class="btn btn-primary w-full">
          Valider
        </button>
        <button
          type="button"
          class="btn btn-accent w-full"
          @click="cancel"
        >
          Annuler
        </button>
    </form>
</template>

<script>
import { mapStores } from 'pinia'
import { useAuthStore } from '@/store/auth.js'

export default {
    name: 'AccountUpdateForm',
    data: function() {
        return {
            email: null,
            password: null,
            old_password: null,
            name: null
        }
    },
    emits: ['done'],
    computed: {
        ...mapStores(useAuthStore),
        account() {
            return this.authStore.account
        }
    },
    mounted() {
        this.email = this.account.email
        this.name = this.account.display_name
    },
    methods: {
        updateAccount() {
            this.authStore.update_account(this.old_password, this.email, this.password, this.name).then(() => {
                this.email = null
                this.password = null
                this.name = null
                this.old_password = null
                this.$emit('done')
            })
        },
        cancel() {
            this.$emit('done')
        }
    }
}
</script>
