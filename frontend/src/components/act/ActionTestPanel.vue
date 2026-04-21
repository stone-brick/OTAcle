<script setup lang="ts">
import { ref, computed, watch } from 'vue';
import type { ActionItem, WindowInfo, Variable } from '../../types';
import CardBox from '@/components/ui/CardBox.vue';
import CardBoxComponentBody from '@/components/ui/CardBoxComponentBody.vue';
import BaseButton from '@/components/ui/BaseButton.vue';
import FormControl from '@/components/ui/FormControl.vue';

const props = defineProps<{
  selectedWindow: WindowInfo | null;
  actions: ActionItem[];
  isLoaded: boolean;
  selectedActionIndex: number | null;
}>();

const emit = defineEmits<{
  execute: [payload: ExecutePayload];
}>();

// Runtime params - key is param_name, value is the override value
const runtimeParams = ref<Record<string, string>>({});

type ExecutePayload = { actionIdx: number; params: Record<string, number | string> };

const selectedAction = computed(() => {
  if (props.selectedActionIndex === null) return null;
  return props.actions[props.selectedActionIndex] || null;
});

// Extract variables from the selected action
const variables = computed((): Variable[] => {
  if (!selectedAction.value) return [];

  const action = selectedAction.value;
  switch (action.type) {
    case 'mouse_move':
    case 'mouse_click':
      return action.variables || [];
    default:
      return [];
  }
});

// Get the action's current value for a given field_name
function getFieldValue(fieldName: string): number | string | undefined {
  if (!selectedAction.value) return undefined;

  const action = selectedAction.value;
  switch (action.type) {
    case 'mouse_move':
      if (fieldName === 'x') return action.x;
      if (fieldName === 'y') return action.y;
      break;
    case 'mouse_click':
      if (fieldName === 'count') return action.count;
      break;
  }
  return undefined;
}

// Get field type for input rendering
function getFieldType(fieldName: string): 'number' | 'text' {
  const action = selectedAction.value;
  if (!action) return 'text';

  switch (action.type) {
    case 'mouse_move':
      if (fieldName === 'x' || fieldName === 'y') return 'number';
      break;
    case 'mouse_click':
      if (fieldName === 'count') return 'number';
      break;
  }
  return 'text';
}

// When action changes, reset runtime params
watch(selectedAction, () => {
  runtimeParams.value = {};
  // Initialize params with empty values
  for (const v of variables.value) {
    runtimeParams.value[v.param_name] = '';
  }
});

function buildParams(): Record<string, number | string> {
  const params: Record<string, number | string> = {};

  // Only include params that have non-empty values
  for (const [key, value] of Object.entries(runtimeParams.value)) {
    if (value !== '' && value !== undefined && value !== null) {
      // Convert to appropriate type based on field
      const fieldType = getFieldType(
        variables.value.find(v => v.param_name === key)?.field_name || ''
      );
      if (fieldType === 'number') {
        params[key] = Number(value);
      } else {
        params[key] = value;
      }
    }
  }

  return params;
}

function handleExecute() {
  if (props.selectedActionIndex === null) {
    return;
  }

  if (variables.value.length > 0) {
    const hasFilledParam = Object.values(runtimeParams.value).some(v => v !== '');
    if (!hasFilledParam) {
      return;
    }
  }

  const params = buildParams();
  emit('execute', {
    actionIdx: props.selectedActionIndex,
    params,
  });
}

function formatFieldName(fieldName: string): string {
  const labels: Record<string, string> = {
    x: 'X 坐标',
    y: 'Y 坐标',
    count: '点击次数',
  };
  return labels[fieldName] || fieldName;
}

// Action type color map
const actionTypeColors: Record<string, string> = {
  key: 'bg-blue-500 text-white',
  key_sequence: 'bg-purple-500 text-white',
  mouse_click: 'bg-orange-500 text-white',
  mouse_move: 'bg-green-500 text-white',
  mouse_scroll: 'bg-cyan-500 text-white',
  delay: 'bg-gray-500 text-white',
  text: 'bg-pink-500 text-white',
};
</script>

<template>
  <CardBox class="flex flex-col flex-1 border-l border-gray-100 dark:border-slate-800">
    <div class="px-3 py-2 border-b border-gray-100 dark:border-slate-800">
      <h3 class="m-0 text-sm font-semibold text-gray-700 dark:text-slate-200">
        动作测试
      </h3>
    </div>

    <CardBoxComponentBody
      v-if="!isLoaded"
      class="flex flex-col items-center justify-center flex-1"
    >
      <p class="text-gray-500 text-sm">
        请先在「配置」标签页加载动作配置文件
      </p>
    </CardBoxComponentBody>

    <template v-else>
      <!-- Target Window Info -->
      <div class="px-4 py-3 border-b border-gray-100 dark:border-slate-800">
        <div class="text-xs font-medium text-gray-500 dark:text-slate-400 uppercase tracking-wide mb-2">
          目标窗口
        </div>
        <div
          v-if="selectedWindow"
          class="bg-gray-50 dark:bg-slate-800 rounded-lg p-3"
        >
          <div class="text-sm font-medium text-gray-700 dark:text-slate-200 mb-1 truncate">
            {{ selectedWindow.title }}
          </div>
          <div class="flex gap-3 text-xs text-gray-500 dark:text-slate-400">
            <span class="font-mono">HWND: 0x{{ selectedWindow.hwnd.toString(16) }}</span>
            <span>{{ selectedWindow.process_name }}</span>
          </div>
        </div>
        <div
          v-else
          class="text-sm text-gray-400 italic"
        >
          未选择窗口（将使用前台窗口）
        </div>
      </div>

      <!-- Selected Action Info -->
      <div class="px-4 py-3 border-b border-gray-100 dark:border-slate-800">
        <div class="text-xs font-medium text-gray-500 dark:text-slate-400 uppercase tracking-wide mb-2">
          当前动作
        </div>
        <div
          v-if="selectedAction"
          class="flex items-center gap-2"
        >
          <span
            class="px-2.5 py-1 text-xs font-semibold rounded uppercase"
            :class="actionTypeColors[selectedAction.type] || 'bg-gray-500 text-white'"
          >
            {{ selectedAction.type }}
          </span>
          <span class="text-sm font-medium text-gray-700 dark:text-slate-200">{{ selectedAction.name || '(无名称)' }}</span>
        </div>
        <div
          v-else
          class="text-sm text-gray-400 italic"
        >
          请在中间列表选择一个动作
        </div>
      </div>

      <!-- Variable Params Form -->
      <div
        v-if="selectedAction && variables.length > 0"
        class="px-4 py-3 border-b border-gray-100 dark:border-slate-800"
      >
        <div class="text-xs font-medium text-gray-500 dark:text-slate-400 uppercase tracking-wide mb-2">
          可调参数
        </div>
        <p class="text-xs text-gray-400 mb-3">
          以下参数可在运行时动态调整
        </p>
        <div class="flex flex-col gap-3">
          <div
            v-for="variable in variables"
            :key="variable.param_name"
            class="flex flex-col gap-1"
          >
            <label class="text-sm font-medium text-gray-700 dark:text-slate-200">
              {{ variable.param_name }}
              <span class="text-xs text-gray-500 font-normal">({{ formatFieldName(variable.field_name) }})</span>
            </label>
            <FormControl
              v-model="runtimeParams[variable.param_name]"
              :type="getFieldType(variable.field_name)"
              :placeholder="`默认值: ${getFieldValue(variable.field_name)}`"
            />
          </div>
        </div>
      </div>

      <!-- No variables hint -->
      <div
        v-else-if="selectedAction && variables.length === 0"
        class="px-4 py-3 border-b border-gray-100 dark:border-slate-800"
      >
        <p class="text-sm text-gray-400 italic">
          此动作未配置可调参数
        </p>
      </div>

      <!-- Execute Button -->
      <div class="px-4 py-3">
        <BaseButton
          label="执行动作"
          color="info"
          class="w-full"
          :disabled="selectedActionIndex === null"
          @click="handleExecute"
        />
      </div>
    </template>
  </CardBox>
</template>
