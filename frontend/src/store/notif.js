import {defineStore } from 'pinia'

export const useNotifStore = defineStore('notif', {
    state: () => {
        return {
            messages: new Array()
        }
    },
    actions: {
        show(msg) {
            this.messages.push(msg)
            setTimeout(() => this.messages.shift(), 5000)
        }
    }
})

