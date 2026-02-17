// src/router/index.js
import { createRouter, createWebHistory } from 'vue-router'
import LandingView from '../views/LandingView.vue'
import GameView from '../views/GameView.vue'
import ArcadeMachineView from '../views/ArcadeMachineView.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'landing',
      component: LandingView
    },
    {
      path: '/play',
      name: 'play',
      component: GameView
      // You might want to add a route guard here later
      // to ensure the Wasm game is loaded or user is "logged in"
    },
    {
      path: '/arcade-machine',
      name: 'ArcadeMachine',
      component: ArcadeMachineView
      // You might want to add a route guard here later
      // to ensure the Wasm game is loaded or user is "logged in"
    }
    // You can remove the default '/about' route if you don't need it
    // {
    //   path: '/about',
    //   name: 'about',
    //   // route level code-splitting
    //   // this generates a separate chunk (About.[hash].js) for this route
    //   // which is lazy-loaded when the route is visited.
    //   component: () => import('../views/AboutView.vue')
    // }
  ]
})

export default router
