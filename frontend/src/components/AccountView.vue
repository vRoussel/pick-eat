<script setup>
import { inject } from 'vue'
import { useRouter } from 'vue-router'

import { useAuthStore } from '@/store/auth.js'

const authStore = useAuthStore()
const router = useRouter()

const emit = defineEmits(['edit'])
const icons = inject('icons')

async function logout() {
    authStore
        .logout()
        .then(() => {
            router.push('/recipes')
        })
        .catch(() => {})
}

function editAccount() {
    emit('edit')
}
</script>

<template>
    <form
        v-if="authStore.account"
        class="mx-auto space-y-4 p-8 border-primary border-[1px] rounded-xl max-w-md relative"
    >
        <span
            class="icon absolute right-2 top-2 text-xl sm:right-4 sm:top-4 sm:text-2xl md:right-6 md:top-6 md:text-3xl"
        >
            <Icon class="text-primary cursor-pointer" :icon="icons.pencil" @click="editAccount()" />
        </span>
        <h1 class="text-xl font-bold text-center">Mon Compte</h1>
        <div class="form-control">
            <label class="label">
                <span class="label-text">Pseudo</span>
            </label>
            <input
                v-model="authStore.account.display_name"
                type="text"
                class="input input-bordered w-full disabled:bg-accent disabled:text-accent-content"
                disabled
            />
        </div>
        <div class="form-control">
            <label class="label">
                <span class="label-text">Adresse mail</span>
            </label>
            <input
                v-model="authStore.account.email"
                type="text"
                class="input input-bordered w-full disabled:bg-accent disabled:text-accent-content"
                disabled
            />
        </div>
        <p class="italic">Membre depuis le {{ authStore.account.creation_date }}</p>
        <div class="form-control">
            <button class="btn btn-primary" type="button" @click="logout">Se deconnecter</button>
        </div>
    </form>
</template>
