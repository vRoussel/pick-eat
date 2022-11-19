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
        <label
          :for="modal_id"
          class="btn btn-primary btn-sm btn-wide mx-auto"
          @click="sendCategory"
        >Ajouter</label>
      </div>
    </div>
  </div>
</template>

<script>
export default {
    name: 'NewCategoryModal',
    inject: ["store"],
    props: {
        modal_id: {
            required: true
        }
    },
    emits: ['closed', 'created'],
    data: function() {
        return {
            name: null,
            modal_opened: false
        }
    },
    watch: {
        modal_opened(val) {
            if (val) {
                this.$refs.categName.focus()
            } else {
                this.name = ""
                this.$emit('closed')
            }
        }
    },
    methods: {
        sendCategory() {
            let category = {
                "name": this.name,
            }
            this.store.addCategory(category)
                .catch((e) => console.error(e))
                .then((new_categ) => {
                    this.$emit('created', new_categ)
                    this.modal_opened = false
                })
        },
    }
}
</script>
