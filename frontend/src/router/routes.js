import { createRouter, createWebHistory } from 'vue-router'
import { useAuthStore } from '@/store/auth.js'

const NewRecipe = () => import('@/views/NewRecipe.vue')
const Recipe = () => import('@/views/Recipe.vue')
const RecipeList = () => import('@/views/RecipeList.vue')
const Login = () => import('@/views/Login.vue')
const Account = () => import('@/views/Account.vue')
const AccountValidation = () => import('@/views/AccountValidation.vue')
const ForgottenPassword = () => import('@/views/ForgottenPassword.vue')
const PasswordReset = () => import('@/views/PasswordReset.vue')
const Register = () => import('@/views/Register.vue')

const routes = [
    {
        name: 'new-recipe',
        path: '/new-recipe',
        component: NewRecipe,
        meta: {
            title: 'Ajouter une recette - PickEat',
        },
    },
    {
        name: 'recipe-list',
        path: '/recipes',
        component: RecipeList,
        meta: {
            title: 'Liste des recettes - PickEat',
            public: true,
        },
    },
    {
        name: 'recipe',
        path: '/recipe/:id',
        component: Recipe,
        props: (route) => ({ id: parseInt(route.params.id), edit: 'edit' in route.query }),
        meta: {
            title: 'Recette - PickEat',
            public: true,
        },
    },
    {
        name: 'register',
        path: '/register',
        component: Register,
        meta: {
            title: 'Inscription - PickEat',
            public: true,
        },
    },
    {
        name: 'login',
        path: '/login',
        component: Login,
        meta: {
            title: 'Connection - PickEat',
            public: true,
        },
    },
    {
        name: 'account',
        path: '/account',
        component: Account,
        props: (route) => ({ edit: 'edit' in route.query }),
        meta: {
            title: 'Mon Compte - PickEat',
        },
    },
    {
        name: 'account_validation',
        path: '/account_validation',
        component: AccountValidation,
        props: (route) => ({ token: route.query.token }),
        meta: {
            title: 'Validation de compte - PickEat',
            public: true,
        },
    },
    {
        name: '/forgotten_password',
        path: '/forgotten_password',
        component: ForgottenPassword,
        meta: {
            title: 'Mot de passe oublié - PickEat',
            public: true,
        },
    },
    {
        name: 'password_reset',
        path: '/password_reset',
        component: PasswordReset,
        props: (route) => ({ token: route.query.token }),
        meta: {
            title: 'Réinitialisation du mot de passe - PickEat',
            public: true,
        },
    },
    {
        name: 'default',
        path: '/',
        redirect: '/recipes',
        meta: {
            title: 'PickEat',
            public: true,
        },
    },
]

const router = createRouter({
    mode: 'history',
    history: createWebHistory(),
    routes,
    linkActiveClass: 'active',
    scrollBehavior: (to, from, savedPosition) => {
        if (savedPosition) {
            return new Promise((resolve) => {
                setTimeout(() => {
                    resolve(savedPosition)
                }, 300)
            })
        } else if (to.query.ns !== undefined) {
            return {}
        } else {
            return new Promise((resolve) => {
                setTimeout(() => {
                    resolve({ left: 0, top: 0 })
                }, 300)
            })
        }
    },
})

router.beforeEach(async (to) => {
    // redirect to login page if not logged in and trying to access a restricted page
    const authRequired = !to.meta.public
    const auth = useAuthStore()

    if (to.path == '/login' && auth.is_logged_in) {
        return '/account'
    }

    if (authRequired && !auth.is_logged_in) {
        auth.return_url = to.fullPath
        return '/login'
    }
})

export default router
