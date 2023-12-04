<template>
    <div class="container my-8">
        <account-view v-if="!edit" @edit="editAccount" />
        <account-update-form v-else @done="afterEdit" />
    </div>
</template>

<script>
import AccountUpdateForm from '@/components/AccountUpdateForm.vue'
import AccountView from '@/components/AccountView.vue'

export default {
    name: 'Account',
    components: {
        AccountUpdateForm,
        AccountView,
    },
    props: {
        edit: {
            type: Boolean,
        },
    },
    methods: {
        editAccount() {
            this.$router.push({ query: { ...this.$route.query, edit: null } })
        },
        afterEdit() {
            this.$router.push({ query: { ...this.$route.query, edit: undefined } })
        },
    },
}
</script>
