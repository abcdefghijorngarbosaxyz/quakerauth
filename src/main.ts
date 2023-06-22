import './tailwind.css';

import { createApp, type App as VueApp } from 'vue';
import App from './App.vue';

const main = (): void => {
  const app: VueApp<Element> = createApp(App);

  app.mount('#app');
};

main();
