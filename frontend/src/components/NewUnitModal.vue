<template>
  <input
    :id="modal_id"
    v-model="modal_opened"
    type="checkbox"
    class="modal-toggle"
  >
  <div
    class="modal"
    tabindex="-1"
    @click.self="modal_opened=false"
    @keyup.esc.stop="modal_opened=false"
  >
    <div class="modal-box relative overflow-visible">
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
        <label
          :for="modal_id"
          class="btn btn-primary btn-sm btn-wide mx-auto"
          @click="sendUnit"
        >Ajouter</label>
      </div>
    </div>
  </div>
</template>

<script>
import { mapStores } from 'pinia'
import { useApiStore } from '@/store/api.js'

export default {
    name: 'NewUnitModal',
    props: {
        modal_id: {
            required: true
        },
        input: null
    },
    emits: ['closed', 'created'],
    data: function() {
        return {
            full_name: this.input,
            short_name: null,
            modal_opened: false
        }
    },
    computed: {
        ...mapStores(useApiStore),
    },
    watch: {
        input: function() {
            this.full_name = this.input;
        },
        modal_opened(val) {
            if (val) {
                this.$refs.unitName.focus()
            } else {
                this.full_name = null
                this.short_name = null
                this.$emit('closed')
            }
        }
    },
    methods: {
        sendUnit() {
            let unit = {
                "full_name": this.full_name,
                "short_name": this.short_name,
            }
            this.apiStore.sendNewUnit(unit)
                .catch((e) => console.error(e))
                .then((new_unit) => {
                    this.$emit('created', new_unit)
                    this.modal_opened = false
                })
        }
    }
}
</script>
