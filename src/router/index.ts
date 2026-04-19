import {createRouter, createWebHashHistory} from "vue-router";

const EventList = () => import("../template/EventList.vue");
const AddEvent = () => import("../template/AddEvent.vue");
const Settings = () => import("../template/Settings.vue");

const routes = [
    { path: '/', name: 'list', component: EventList},
    { path: '/add', name: 'add', component: AddEvent},
    { path: '/settings', name: 'settings', component: Settings},
]

export const router = createRouter({
    history:createWebHashHistory(),
    routes,
})