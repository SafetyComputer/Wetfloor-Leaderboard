import {createRouter, createWebHistory} from 'vue-router'
import IndexView from "../views/IndexView.vue";
import AddMatchView from "../views/AddMatchView.vue";
import RecentMatchView from "../views/RecentMatchView.vue";

const router = createRouter({
    history: createWebHistory(),
    routes: [
        {
            path: '/',
            name: 'index',
            component: IndexView
        },
        {
            path: '/match',
            name: 'match',
            component: RecentMatchView
        },
        {
            path: '/add',
            name: 'add',
            component: AddMatchView
        },
    ],
})


export default router
