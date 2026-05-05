<script setup lang="ts">
import { ref, reactive } from 'vue'
import BaseIcon from '../../components/ui/BaseIcon.vue'
import { mdiLogin, mdiAccountPlus } from '@mdi/js'
import { useRemote } from '../../composables/useRemote'

const { login, register, isLoading, error, serverUrl } = useRemote()

const mode = ref<'login' | 'register'>('login')
const form = reactive({
  username: '',
  password: '',
  email: '',
})
const localError = ref<string | null>(null)

async function handleSubmit() {
  localError.value = null
  try {
    if (mode.value === 'login') {
      await login(form.username, form.password)
    } else {
      await register(form.username, form.password, form.email || undefined)
    }
    emit('success')
  } catch (e) {
    localError.value = String(e)
  }
}

const emit = defineEmits<{
  success: []
}>()

function switchMode() {
  mode.value = mode.value === 'login' ? 'register' : 'login'
  localError.value = null
}

function toggleShowPassword() {
  const pwd = document.getElementById('remote-password') as HTMLInputElement
  pwd.type = pwd.type === 'password' ? 'text' : 'password'
}
</script>

<template>
  <div class="flex items-center justify-center h-full p-4">
    <div class="w-full max-w-sm bg-white dark:bg-slate-900 rounded-xl p-6" style="box-shadow: 0 4px 24px rgba(0, 0, 0, 0.08);">
      <div class="text-sm text-gray-400 text-center mb-4">{{ serverUrl }}</div>

      <form @submit.prevent="handleSubmit" class="space-y-4">
      <div>
        <label class="block text-sm font-medium text-gray-600 dark:text-gray-400 mb-1">用户名</label>
        <input
          v-model="form.username"
          type="text"
          placeholder="请输入用户名"
          class="w-full px-3 py-2 text-sm border border-gray-200 dark:border-slate-700 rounded-lg bg-white dark:bg-slate-800 focus:outline-none focus:border-blue-400"
          required
        />
      </div>

      <div>
        <label class="block text-sm font-medium text-gray-600 dark:text-gray-400 mb-1">密码</label>
        <div class="relative">
          <input
            id="remote-password"
            v-model="form.password"
            type="password"
            placeholder="请输入密码"
            class="w-full px-3 py-2 text-sm border border-gray-200 dark:border-slate-700 rounded-lg bg-white dark:bg-slate-800 focus:outline-none focus:border-blue-400 pr-10"
            required
          />
          <button
            type="button"
            class="absolute right-3 top-1/2 -translate-y-1/2 text-gray-400 hover:text-gray-600"
            @click="toggleShowPassword"
          >
            <BaseIcon path="mdiEye" :size="16" />
          </button>
        </div>
      </div>

      <div v-if="mode === 'register'">
        <label class="block text-sm font-medium text-gray-600 dark:text-gray-400 mb-1">邮箱（可选）</label>
        <input
          v-model="form.email"
          type="email"
          placeholder="请输入邮箱"
          class="w-full px-3 py-2 text-sm border border-gray-200 dark:border-slate-700 rounded-lg bg-white dark:bg-slate-800 focus:outline-none focus:border-blue-400"
        />
      </div>

      <div v-if="localError || error" class="text-sm text-red-500 py-2 text-center">
        {{ localError || error }}
      </div>

      <button
        type="submit"
        :disabled="isLoading"
        class="w-full py-2.5 text-sm rounded-lg transition-colors duration-150 flex items-center justify-center gap-2 font-medium"
        :class="isLoading
          ? 'bg-gray-300 cursor-not-allowed'
          : mode === 'login'
            ? 'bg-blue-500 hover:bg-blue-600 text-white'
            : 'bg-green-500 hover:bg-green-600 text-white'"
      >
        <BaseIcon :path="mode === 'login' ? mdiLogin : mdiAccountPlus" :size="18" />
        {{ isLoading ? '处理中...' : (mode === 'login' ? '登录' : '注册') }}
      </button>
    </form>

    <div class="mt-4 text-center">
      <button
        type="button"
        class="text-sm text-blue-500 hover:text-blue-600"
        @click="switchMode"
      >
        {{ mode === 'login' ? '没有账号？去注册' : '已有账号？去登录' }}
      </button>
    </div>
    </div>
  </div>
</template>