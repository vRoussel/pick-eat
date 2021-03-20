import { createRouter, createWebHistory} from 'vue-router';
import NewRecipeForm from '@/views/NewRecipe.vue';

const routes = [
    {
        name: 'new-recipe',
        path: '/new-recipe',
        component: NewRecipeForm,
    },
];

const router = createRouter({
    history: createWebHistory(),
    routes,
    linkActiveClass: 'is-active'
});

export default router;
