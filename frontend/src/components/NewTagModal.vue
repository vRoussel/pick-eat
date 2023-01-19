<template>
  <div
    class="modal cursor-pointer"
    :class="{'modal-open': opened}"
    tabindex="-1"
    @click.self="close"
    @keyup.esc.stop="close"
  >
    <div class="modal-box relative overflow-visible cursor-default">
      <form
        autocomplete="off"
        @submit.prevent="sendTag"
      >
        <div class="form-control w-full">
          <label class="label">
            <span class="label-text">Nom</span>
          </label>
          <input
            id="name"
            ref="tagName"
            v-model="name"
            class="input input-bordered w-full"
            type="text"
            name="name"
            required
          >
        </div>
      </form>
      <div class="modal-action">
        <button
          class="btn btn-primary btn-sm btn-wide mx-auto"
          type="button"
          @click="sendTag"
        >Ajouter</button>
      </div>
    </div>
  </div>
</template>

<script>
import { mapStores } from 'pinia'
import { useFoodStore } from '@/store/food.js'

export default {
    name: 'NewTagModal',
    emits: ['closed', 'created'],
    data: function() {
        return {
            name: null,
            opened: false,
        }
    },
    computed: {
        ...mapStores(useFoodStore),
    },
    methods: {
        sendTag() {
            let tag = {
                "name": this.name,
            }
            this.foodStore.sendNewTag(tag)
                .then((new_tag) => {
                    this.$emit('created', new_tag)
                    this.close()
                })
        },
        open() {
            this.opened = true
            setTimeout(() => this.$refs.tagName.focus(), 50)
        },
        close() {
            this.opened = false
            this.name = ""
            this.$emit('closed')
        }
    }
}
</script>
