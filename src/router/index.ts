import { createRouter, createWebHistory } from "vue-router";

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: "/",
      redirect: "/philosophers",
    },
    {
      path: "/philosophers",
      name: "philosophers",
      component: () =>
        import("@/modules/philosophers/views/PhilosophersView.vue"),
    },
    {
      path: "/dialogue",
      name: "dialogue",
      component: () => import("@/modules/dialogue/views/DialogueView.vue"),
    },
    {
      path: "/dialogue/:philosopherId",
      name: "dialogue-with",
      component: () => import("@/modules/dialogue/views/DialogueView.vue"),
      props: true,
    },
    {
      path: "/me",
      name: "me",
      component: () => import("@/modules/me/views/MeView.vue"),
    },
  ],
});

export default router;
