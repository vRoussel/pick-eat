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
        @submit.prevent="sendUnit"
      >
        <div class="form-control w-full">
          <label class="label">
            <span class="label-text">Nom complet</span>
          </label>
          <input
            id="full_name"
            ref="unitName"
            v-model="full_name"
            class="input input-bordered w-full"
            type="text"
            name="full_name"
            required
          >
        </div>
        <div class="form-control w-full">
          <label class="label">
            <span class="label-text">Nom court</span>
          </label>
          <input
            id="short_name"
            v-model="short_name"
            class="input input-bordered w-full"
            type="text"
            name="short_name"
            required
          >
        </div>
      </form>
      <div class="modal-action">
        <button
          class="btn btn-primary btn-sm btn-wide mx-auto"
          type="button"
          @click="sendUnit"
        >Ajouter</button>
      </div>
    </div>
  </div>
</template>

<script>
import { mapStores } from 'pinia'
import { useFoodStore } from '@/store/food.js'

export default {
    name: 'NewUnitModal',
    props: {
        input: null
    },
    emits: ['closed', 'created'],
    data: function() {
        return {
            full_name: this.input,
            short_name: null,
            opened: false
        }
    },
    computed: {
        ...mapStores(useFoodStore),
    },
    watch: {
        input: function() {
            this.full_name = this.input;
        }
    },
    methods: {
        sendUnit() {
            let unit = {
                "full_name": this.full_name,
                "short_name": this.short_name,
            }
            this.foodStore.sendNewUnit(unit)
                .then((new_unit) => {
                    this.$emit('created', new_unit)
                    this.close()
                })
        },
        open() {
            this.opened = true
            setTimeout(() => this.$refs.unitName.focus(), 50)
        },
        close() {
            this.opened = false
                this.full_name = null
                this.short_name = null
            this.$emit('closed')
        }
    }
}
</script>
