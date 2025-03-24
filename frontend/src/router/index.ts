import { createRouter, createWebHistory } from 'vue-router'
import StatsView from '@/views/StatsView.vue'
import ArtistView from '@/views/ArtistView.vue'
import ProfileView from '@/views/ProfileView.vue'
import AlbumView from '@/views/AlbumView.vue'

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
    }
  ]
})

export default router 