import { createRouter, createWebHistory } from 'vue-router'
import StatsView from '@/views/StatsView.vue'
import ArtistView from '@/views/ArtistView.vue'
import ProfileView from '@/views/ProfileView.vue'
import AlbumView from '@/views/AlbumView.vue'
import ImportView from '@/views/ImportView.vue'
import UsersView from '@/views/UsersView.vue'
import ApiKeysView from '@/views/ApiKeysView.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'stats',
      component: StatsView
    },
    {
      path: '/artist/:id',
      name: 'artist',
      component: ArtistView
    },
    {
      path: '/album/:id',
      name: 'album',
      component: AlbumView
    },
    {
      path: '/profile',
      name: 'profile',
      component: ProfileView
    },
    {
      path: '/import',
      name: 'import',
      component: ImportView
    },
    {
      path: '/users',
      name: 'users',
      component: UsersView
    },
    {
      path: '/api-keys',
      name: 'apiKeys',
      component: ApiKeysView
    }
  ]
})

export default router 
