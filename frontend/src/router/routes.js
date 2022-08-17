import { createRouter, createWebHistory} from 'vue-router';

import NewRecipe from '@/views/NewRecipe.vue'
import Recipe from '@/views/Recipe.vue'
import RecipeList from '@/views/RecipeList.vue'

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

export default router;
