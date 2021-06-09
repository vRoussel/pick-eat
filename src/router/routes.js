import { createRouter, createWebHistory} from 'vue-router';
import NewRecipeForm from '@/views/NewRecipe.vue';
import RecipeList from '@/views/RecipeList.vue';

const routes = [
    {
        name: 'new-recipe',
        path: '/new-recipe',
        component: NewRecipeForm,
    },
    {
        name: 'recipe-list',
        path: '/recipes',
        component: RecipeList,
        props: route => ({ page: parseInt(route.query.page) })
    }
];

const router = createRouter({
    history: createWebHistory(),
    routes,
    linkActiveClass: 'is-active'
});

export default router;
