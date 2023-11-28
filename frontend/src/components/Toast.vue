<template>
  <div class="toast">
    <div
      v-for="msg in error_queue"
      :key="msg"
      class="alert alert-error"
    >
      <div>
        <span>{{ msg }}</span>
      </div>
    </div>
    <div
      v-for="msg in info_queue"
      :key="msg"
      class="alert alert-info"
    >
      <div>
        <span>{{ msg }}</span>
      </div>
    </div>
  </div>
</template>

<script>
    export default {
        props: {
            error_queue: {
                type: Array
            },
            info_queue: {
                type: Array
            }
        }
    }
</script>

