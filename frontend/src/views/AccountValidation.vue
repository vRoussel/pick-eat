<template>
    <div class="my-8">
        <form class="mx-auto space-y-4 p-8 border-primary border-[1px] rounded-xl max-w-md" @submit.prevent="validate_account">
        <div>
            <img :src="welcome_gif">
        </div>
        <div class="form-control">
            <button class="btn btn-primary w-full">
              Valider mon compte
            </button>
        </div>
        </form>
    </div>
</template>

<script>
import { mapStores } from 'pinia'
import { useNotifStore } from '@/store/notif.js'
import { useAuthStore } from '@/store/auth.js'
import Swal from 'sweetalert2'
import {handle_form_api_errors} from '@/utils/utils.js'
import welcome_gif from '@/assets/gatsby_welcome.gif'


export default {
    name: 'Register',
    data: function() {
        return {
            welcome_gif: welcome_gif
        }
    },
    props: {
        token: {
            type: String
        }
    },
    computed: {
      ...mapStores(useNotifStore, useAuthStore)
    },
    methods: {
        async validate_account() {
            this.authStore.validate_account(this.token).then(() => {
                Swal.fire({
                  title: 'Bienvenue !',
                  icon: 'success',
                  text: 'Votre compte a bien été validé'
                })
                this.$router.push('/login')
            })
            .catch(err => {
                console.error(err)
                handle_form_api_errors(err.response, {}, this.notifStore)
            });
        },
    }
}
</script>
