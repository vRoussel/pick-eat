<template>
    <div class="my-8">
        <form class="mx-auto space-y-4 p-8 border-primary border-[1px] rounded-xl max-w-md" @submit.prevent="login">
        <h1 class="text-xl font-bold text-center">Connexion</h1>
        <p class="text-center">Tu n'as pas encore de compte ? <router-link to="/register">Créer un compte</router-link></p>
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
                    <span class="label-text">Mot de passe</span>
                </label>
                <input
                    v-model="password"
                    type="password"
                    class="input input-bordered w-full"
                >
            </div>
            <div class="form-control">
                <button class="btn btn-primary w-full btn-lg">
                  Connexion
                </button>
            </div>
        </form>
    </div>
</template>

<script>
import { mapStores } from 'pinia'
import { useAuthStore } from '@/store/auth.js'

export default {
    name: 'Login',
    data: function() {
        return {
            email: null,
            password: null
        }
    },
    computed: {
      ...mapStores(useAuthStore)
    },
    methods: {
        login() {
            this.authStore.login(this.email, this.password).then(() => {
                this.$router.push(this.authStore.return_url || '/account')
                this.authStore.return_url = null
                this.email = null
                this.password = null
            }).catch(() => {})
        }
    }
}
</script>
