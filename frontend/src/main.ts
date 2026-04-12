import { createApp } from "vue";
import App from "./App.vue";
import router from "./router";
import "./styles/variables.css";
import "./styles/_scrollbars.css";

// Set default theme to light on html element
document.documentElement.setAttribute("data-theme", "light");

const app = createApp(App);
app.use(router);
app.mount("#app");
