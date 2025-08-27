import { createRouter, createWebHistory } from "vue-router";
import StatsView from "@/views/StatsView.vue";
import ArtistView from "@/views/ArtistView.vue";
import AlbumView from "@/views/AlbumView.vue";
import ImportView from "@/views/ImportView.vue";
import ExportView from "@/views/ExportView.vue";
import UsersView from "@/views/UsersView.vue";
import ApiKeysView from "@/views/ApiKeysView.vue";
import TrackView from "@/views/TrackView.vue";
import CallbackView from "@/views/CallbackView.vue";
import { useAuthStore } from "@/stores";
import TopDetailView from "@/views/TopDetailView.vue";

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: "/",
      name: "stats",
      component: StatsView,
      meta: { requiresAuth: true },
    },
    {
      path: "/artist/:id",
      name: "artist",
      component: ArtistView,
      meta: { requiresAuth: true },
    },
    {
      path: "/top-artists",
      name: "topArtists",
      component: TopDetailView,
      meta: { requiresAuth: true },
    },
    {
      path: "/album/:id",
      name: "album",
      component: AlbumView,
      meta: { requiresAuth: true },
    },
    {
      path: "/top-albums",
      name: "topAlbums",
      component: TopDetailView,
      meta: { requiresAuth: true },
    },
    {
      path: "/track/:id",
      name: "track",
      component: TrackView,
      meta: { requiresAuth: true },
    },
    {
      path: "/top-tracks",
      name: "topTracks",
      component: TopDetailView,
      meta: { requiresAuth: true },
    },
    {
      path: "/import",
      name: "import",
      component: ImportView,
      meta: { requiresAuth: true },
    },
    {
      path: "/export",
      name: "export",
      component: ExportView,
      meta: { requiresAuth: true },
    },
    {
      path: "/users",
      name: "users",
      component: UsersView,
      meta: { requiresAuth: true },
    },
    {
      path: "/api-keys",
      name: "apiKeys",
      component: ApiKeysView,
      meta: { requiresAuth: true },
    },
    {
      path: "/callback",
      name: "callback",
      component: CallbackView,
      meta: { requiresAuth: false },
    },
  ],
});

router.beforeEach(async (to, _, next) => {
  const requiresAuth = to.matched.some((record) => record.meta.requiresAuth);

  if (!requiresAuth) {
    return next();
  }

  const authStore = useAuthStore();

  if (authStore.isLoading) {
    await authStore.initialize();
  }

  const isAuthenticated = authStore.isAuthenticated;

  if (requiresAuth && !isAuthenticated) {
    await authStore.login();
    return;
  }

  next();
});

export default router;
