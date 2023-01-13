import { createRouter, createWebHistory} from 'vue-router';

import NewRecipe from '@/views/NewRecipe.vue'
import Recipe from '@/views/Recipe.vue'
import RecipeList from '@/views/RecipeList.vue'
import Login from '@/views/Login.vue'
import Account from '@/views/Account.vue'
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
            title: 'Liste des recettes - PickEat'
        }
    },
    {
        name: 'recipe',
        path: '/recipe/:id',
        component: Recipe,
        props: route => ({ id: parseInt(route.params.id), edit: 'edit' in route.query }),
        meta: {
            title: 'Recette - PickEat'
        }
    },
    {
        name: 'register',
        path: '/register',
        component: Register,
        meta: {
            title: 'Inscription - PickEat'
        }
    },
    {
        name: 'login',
        path: '/login',
        component: Login,
        meta: {
            title: 'Connection - PickEat'
        }
    },
    {
        name: 'account',
        path: '/account',
        component: Account,
        meta: {
            title: 'Mon Compte - PickEat'
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
        if (to.params.noscroll == "true") {
            return {}
        } if (savedPosition) {
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
    const publicPages = ['/register', '/login', '/recipes'];
    const authRequired = !publicPages.includes(to.path);
    const auth = useAuthStore();

    if (to == "/login" && auth.is_logged_in) {
        return '/account'
    }

    if (authRequired && !auth.is_logged_in) {
        auth.return_url = to.fullPath;
        return '/login';
    }
});

export default router;
