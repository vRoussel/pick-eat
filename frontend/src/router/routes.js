import { createRouter, createWebHistory} from 'vue-router';
import { defineAsyncComponent } from 'vue'

const NewRecipe = defineAsyncComponent(() =>
  import('@/views/NewRecipe.vue')
)
const Recipe = defineAsyncComponent(() =>
  import('@/views/Recipe.vue')
)
const RecipeList = defineAsyncComponent(() =>
  import('@/views/RecipeList.vue')
)

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
