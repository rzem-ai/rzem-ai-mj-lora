import { createRouter, createWebHistory } from 'vue-router';
import UploadView from '../views/UploadView.vue';
import AnalysisView from '../views/AnalysisView.vue';
import BatchEditorView from '../views/BatchEditorView.vue';
import ExportView from '../views/ExportView.vue';
import SettingsView from '../views/SettingsView.vue';

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: '/',
      name: 'upload',
      component: UploadView,
      meta: { title: 'Upload Images' },
    },
    {
      path: '/analysis',
      name: 'analysis',
      component: AnalysisView,
      meta: { title: 'Style Analysis' },
    },
    {
      path: '/batches',
      name: 'batches',
      component: BatchEditorView,
      meta: { title: 'Edit Batches' },
    },
    {
      path: '/export',
      name: 'export',
      component: ExportView,
      meta: { title: 'Export' },
    },
    {
      path: '/settings',
      name: 'settings',
      component: SettingsView,
      meta: { title: 'Settings' },
    },
  ],
});

export default router;
