<template>
    <div class="my-8">
        <form v-if="this.account" class="mx-auto space-y-4 p-8 border-primary border-[1px] rounded-xl max-w-md" @submit.prevent="logout">
        <h1 class="text-xl font-bold text-center">Mon Compte</h1>
        <div class="form-control">
            <label class="label">
                <span class="label-text">Nom</span>
            </label>
            <input
                v-model="this.account.display_name"
                type="text"
                class="input input-bordered w-full"
                disabled
            >
        </div>
        <div class="form-control">
            <label class="label">
                <span class="label-text">Adresse mail</span>
            </label>
            <input
                v-model="this.account.email"
                type="text"
                class="input input-bordered w-full"
                disabled
            >
        </div>
        <p class="italic">Membre depuis le {{ this.account.creation_date }}</p>
        <div class="form-control">
            <button class="btn btn-primary">
              Se deconnecter
            </button>
        </div>
        </form>
    </div>
</template>

<script>
import { mapStores } from 'pinia'
import { useAuthStore } from '@/store/auth.js'

export default {
    name: 'MyAccount',
    inject: ["icons"],
    computed: {
      ...mapStores(useAuthStore),
      account() {
          return this.authStore.account
      }
    },
    methods: {
        async logout() {
            this.authStore.logout().then(() => {
                this.$router.push('/recipes')
            }).catch(() => {})
        }
    }
}
</script>
