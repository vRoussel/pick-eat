import {defineStore } from 'pinia'

export const useNotifStore = defineStore('notif', {
    state: () => {
        return {
            error_msgs: new Array(),
        }
    },
    actions: {
        show_error(msg) {
            this.error_msgs.push(msg)
            setTimeout(() => this.error_msgs.shift(), 5000)
        }
    }
})

