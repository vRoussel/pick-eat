import { createRouter, createWebHistory} from 'vue-router';

import NewRecipe from '@/views/NewRecipe.vue'
import Recipe from '@/views/Recipe.vue'
import RecipeList from '@/views/RecipeList.vue'
import Login from '@/views/Login.vue'
import Account from '@/views/Account.vue'
import AccountValidation from '@/views/AccountValidation.vue'
import ForgottenPassword from '@/views/ForgottenPassword.vue'
import PasswordReset from '@/views/PasswordReset.vue'
import Register from '@/views/Register.vue'
import { useAuthStore } from '@/store/auth.js'

const routes = [
    {
        name: 'new-recipe',
        path: '/new-recipe',
        component: NewRecipe,
        meta: {
            title: 'Ajouter une recette - PickEat'
        }
    },
    {
        name: 'recipe-list',
        path: '/recipes',
        component: RecipeList,
        meta: {
            title: 'Liste des recettes - PickEat',
            public: true
        }
    },
    {
        name: 'recipe',
        path: '/recipe/:id',
        component: Recipe,
        props: route => ({ id: parseInt(route.params.id), edit: 'edit' in route.query }),
        meta: {
            title: 'Recette - PickEat',
            public: true
        }
    },
    {
        name: 'register',
        path: '/register',
        component: Register,
        meta: {
            title: 'Inscription - PickEat',
            public: true
        }
    },
    {
        name: 'login',
        path: '/login',
        component: Login,
        meta: {
            title: 'Connection - PickEat',
            public: true
        }
    },
    {
        name: 'account',
        path: '/account',
        component: Account,
        props: route => ({ edit: 'edit' in route.query }),
        meta: {
            title: 'Mon Compte - PickEat'
        }
    },
    {
        name: 'account_validation',
        path: '/account_validation',
        component: AccountValidation,
        props: route => ({ token: route.query.token }),
        meta: {
            title: 'Validation de compte - PickEat',
            public: true
        }
    },
    {
        name: '/forgotten_password',
        path: '/forgotten_password',
        component: ForgottenPassword,
        meta: {
            title: 'Mot de passe oublié - PickEat',
            public: true
        }
    },
    {
        name: 'password_reset',
        path: '/password_reset',
        component: PasswordReset,
        props: route => ({ token: route.query.token }),
        meta: {
            title: 'Réinitialisation du mot de passe - PickEat',
            public: true
        }
    },
    {
        name: 'default',
        path: '/',
        redirect: '/recipes',
    },
];

const router = createRouter({
    mode: "history",
    history: createWebHistory(),
    routes,
    linkActiveClass: 'active',
    scrollBehavior: (to, from, savedPosition) => {
        if (to.query.ns !== undefined) {
            return {}
        }
        else if (savedPosition) {
            return new Promise((resolve) => {
                setTimeout(() => {
                    resolve(savedPosition)
                    }, 200)
            })
        } else {
            return new Promise((resolve) => {
                setTimeout(() => {
                    resolve({left: 0,top: 0})
                    }, 200)
            })
        }
    }
});

router.beforeEach(async (to) => {
    // redirect to login page if not logged in and trying to access a restricted page
    const authRequired = !to.meta.public;
    const auth = useAuthStore();

    if (to.path == "/login" && auth.is_logged_in) {
        return '/account'
    }

    if (authRequired && !auth.is_logged_in) {
        auth.return_url = to.fullPath;
        return '/login';
    }
});

export default router;
