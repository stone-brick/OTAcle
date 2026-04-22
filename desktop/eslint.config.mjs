import { defineConfig } from "eslint/config";
import eslint from "@eslint/js";
import vuePlugin from "eslint-plugin-vue";
import tseslint from "typescript-eslint";
import vueParser from "vue-eslint-parser";
import globals from "globals";

export default defineConfig([
  // 1. 忽略文件
  {
    ignores: ["dist/**", "node_modules/**", "src-tauri/**", "*.min.js"],
  },

  // 2. JS 基础推荐配置
  eslint.configs.recommended,

  // 3. TypeScript 推荐配置
  ...tseslint.configs.recommended,

  // 4. Vue 3 推荐配置（使用 flat/recommended，包含 vue-eslint-parser）
  vuePlugin.configs["flat/recommended"],

  // 5. Vue SFC 使用 vue-eslint-parser + ts ESLint 解析器
  {
    files: ["**/*.vue"],
    languageOptions: {
      parser: vueParser,
      parserOptions: {
        parser: tseslint.parser,
        extraFileExtensions: [".vue"],
        ecmaVersion: 2022,
        sourceType: "module",
      },
      globals: {
        ...globals.browser,
        defineProps: "readonly",
        defineEmits: "readonly",
        defineExpose: "readonly",
        defineComponent: "readonly",
        onMounted: "readonly",
        onUnmounted: "readonly",
        ref: "readonly",
        computed: "readonly",
        watch: "readonly",
        nextTick: "readonly",
      },
    },
  },

  // 6. TypeScript 文件（排除 Vue，因为已在上面单独处理）
  {
    files: ["**/*.ts", "**/*.tsx"],
    languageOptions: {
      parser: tseslint.parser,
      parserOptions: {
        ecmaVersion: 2022,
        sourceType: "module",
      },
      globals: {
        ...globals.browser,
        process: "readonly",
        module: "readonly",
      },
    },
  },

  // 7. 项目自定义规则
  {
    name: "otacle/custom-rules",
    rules: {
      // TypeScript
      "@typescript-eslint/no-unused-vars": ["warn", { argsIgnorePattern: "^_" }],
      "@typescript-eslint/no-explicit-any": "warn",
      "@typescript-eslint/explicit-function-return-type": "off",

      // Vue
      "vue/multi-word-component-names": "off",
      "vue/no-unused-vars": "warn",
      "vue/require-default-prop": "off",

      // 通用
      "no-console": "warn",
      "no-debugger": "warn",
    },
  },
]);
