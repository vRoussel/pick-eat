<script setup>
import { useRouter, useRoute } from 'vue-router'

import AccountUpdateForm from '@/components/AccountUpdateForm.vue'
import AccountView from '@/components/AccountView.vue'


const router = useRouter()
const route = useRoute()

defineProps({
    edit: Boolean
})

function editAccount() {
    router.push({ query: { ...route.query, edit: null } })
}

function viewAccount() {
    router.push({ query: { ...route.query, edit: undefined } })
}
</script>

<template>
    <div class="container my-8">
        <account-view v-if="!edit" @edit="editAccount" />
        <account-update-form v-else @done="viewAccount" />
    </div>
</template>
