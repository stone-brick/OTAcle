import { createApp } from "vue";
import App from "./App.vue";
import router from "./router";
import "./styles/_base.css";
import "./styles/_scrollbars.css";
import "./styles/variables.css";

// 在 html 元素上设置默认主题为浅色
document.documentElement.setAttribute("data-theme", "light");

const app = createApp(App);
app.use(router);
app.mount("#app");
