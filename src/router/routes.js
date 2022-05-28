import { createRouter, createWebHistory} from 'vue-router';
import NewRecipe from '@/views/NewRecipe.vue';
import RecipeList from '@/views/RecipeList.vue';
import Recipe from '@/views/Recipe.vue';
import {Filters} from '@/components/RecipeFilters.vue';

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
        props: route => ({
            page: parseInt(route.query.page) || 1,
            url_filters: new Filters(
                route.query.search,
                route.query.i ? route.query.i.split(',') : [],
                route.query.t ? route.query.t.split(',') : [],
                route.query.c ? route.query.c.split(',') : [],
                route.query.s ? route.query.s.split(',') : []
            )
        }),
        meta: {
            title: 'Liste des recettes - PickEat'
        }
    },
    {
        name: 'recipe',
        path: '/recipe/:id',
        component: Recipe,
        props: route => ({ id: parseInt(route.params.id) }),
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
    linkActiveClass: 'is-active',
    scrollBehavior: (to, from, savedPosition) => {
        if (to.params.noscroll) {
            return {}
        } if (savedPosition) {
            return new Promise((resolve) => {
                setTimeout(() => {
                    resolve(savedPosition)
                    }, 200)
            })
        } else {
          return { left: 0, top: 0 };
        }
    }
});

export default router;
