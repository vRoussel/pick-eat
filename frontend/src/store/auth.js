import { defineStore } from 'pinia'
import axios from 'axios';

const API_PROTO = window.location.protocol
const API_HOST = window.location.hostname
const API_ROOT = `${API_PROTO}//${API_HOST}/api/v1`


export const useAuthStore = defineStore('auth', {
    state: () => {
        return {
            account: null,
            return_url: null
        }
    },
    getters: {
        is_logged_in(state) {
            return state.account != null
        }
    },
    actions: {
        async login(email, password) {
            let post = {'email': email, 'password': password}
            let headers = {
                'Accept': 'application/json',
                'Content-Type': 'application/json;charset=UTF-8'
            }

            await axios.post(`${API_ROOT}/sessions`, post, { 'headers': headers })

            let resp2 = await axios.get(`${API_ROOT}/accounts/me`)
            this.account = resp2.data
            return resp2.data
        },
        async load_account() {
            return axios.get(`${API_ROOT}/accounts/me`).then(resp => {
                this.account = resp.data
            })
        },
        async logout() {
            this.account = null
            let headers = {
                'Accept': 'application/json',
                'Content-Type': 'application/json;charset=UTF-8'
            }

            await axios.delete(`${API_ROOT}/sessions/current`, { 'headers': headers })
        },
        async register(email, password, display_name) {
            let post = {'email': email, 'password': password, 'display_name': display_name}
            let headers = {
                'Accept': 'application/json',
                'Content-Type': 'application/json;charset=UTF-8'
            }

            await axios.post(`${API_ROOT}/accounts`, post, { 'headers': headers })
        },
        async delete_account() {
            let headers = {
                'Accept': 'application/json',
                'Content-Type': 'application/json;charset=UTF-8'
            }

            await axios.delete(`${API_ROOT}/accounts/me`, { 'headers': headers })
            await this.logout()
        },
    }
})
