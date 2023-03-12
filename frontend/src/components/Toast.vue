<template>
    <div class="toast">
        <div v-for="msg in msg_queue" :key="msg" class="alert alert-error">
            <div>
                <span>{{ msg }}</span>
            </div>
        </div>
    </div>
</template>

<script>
    export default {
        props: {
            msg_queue: {
                type: Array()
            }
        }
    }
</script>

