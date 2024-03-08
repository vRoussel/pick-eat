<script setup>
import { useRouter } from 'vue-router'
import { useHead } from '@unhead/vue'
import Swal from 'sweetalert2'

import { useNotifStore } from '@/store/notif.js'
import { useAuthStore } from '@/store/auth.js'

import { handle_form_api_errors } from '@/utils/utils.js'
import welcome_video from '@/assets/gatsby_welcome.mp4'

const notifStore = useNotifStore()
const authStore = useAuthStore()
const router = useRouter()

useHead({
    title: 'Validation de compte',
    meta: {
        name: 'robots',
        content: 'noindex',
    },
})

const props = defineProps({
    token: String,
})

async function validate_account() {
    authStore
        .validate_account(props.token)
        .then(() => {
            Swal.fire({
                title: 'Bienvenue !',
                icon: 'success',
                text: 'Votre compte a bien été validé',
            })
            router.push('/login')
        })
        .catch((err) => {
            console.error(err)
            handle_form_api_errors(err.response, {}, notifStore)
        })
}
</script>

<template>
    <div class="my-8">
        <form
            class="mx-auto space-y-4 p-8 border-primary border-[1px] rounded-xl max-w-md"
            @submit.prevent="validate_account"
        >
            <div>
                <video :src="welcome_video" autoplay loop muted playsinline />
            </div>
            <div class="form-control">
                <button class="btn btn-primary w-full">Valider mon compte</button>
            </div>
        </form>
    </div>
</template>
