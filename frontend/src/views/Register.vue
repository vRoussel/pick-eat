<template>
    <div class="my-8">
        <form class="mx-auto space-y-4 p-8 border-primary border-[1px] rounded-xl max-w-md" @submit.prevent="register">
        <h1 class="text-xl font-bold text-center">Inscription</h1>
        <p class="text-center">Tu as déjà un compte ? <router-link to="/login">Se connecter</router-link></p>
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
                  Créer mon compte
                </button>
            </div>
        </form>
    </div>
</template>

<script>
import { mapStores } from 'pinia'
import { useAuthStore } from '@/store/auth.js'

export default {
    name: 'Register',
    data: function() {
        return {
            email: null,
            password: null,
            name: null
        }
    },
    computed: {
      ...mapStores(useAuthStore)
    },
    methods: {
        async register() {
            this.authStore.register(this.email, this.password, this.name).then(() => {
                this.$router.push('/login')
                this.email = null
                this.password = null
                this.name = null
            })
        }
    }
}
</script>
