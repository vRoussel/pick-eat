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
        @submit.prevent="sendCategory"
      >
        <div class="form-control w-full">
          <label class="label">
            <span class="label-text">Nom</span>
          </label>
          <input
            id="name"
            ref="categName"
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
          @click="sendCategory"
        >Ajouter</button>
      </div>
    </div>
  </div>
</template>

<script>
import { mapStores } from 'pinia'
import { useFoodStore } from '@/store/food.js'

export default {
    name: 'NewCategoryModal',
    computed: {
        ...mapStores(useFoodStore),
    },
    emits: ['closed', 'created'],
    data: function() {
        return {
            name: null,
            opened: false
        }
    },
    methods: {
        sendCategory() {
            let category = {
                "name": this.name,
            }
            this.foodStore.sendNewCategory(category)
                .then((new_categ) => {
                    this.$emit('created', new_categ)
                    this.close()
                })
        },
        open() {
            this.opened = true
            setTimeout(() => this.$refs.categName.focus(), 50)
        },
        close() {
            this.opened = false
            this.name = ""
            this.$emit('closed')
        }
    }
}
</script>
