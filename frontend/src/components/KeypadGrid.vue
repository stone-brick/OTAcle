<script setup lang="ts">
const emit = defineEmits<{
  keyPress: [key: string];
  comboPress: [keys: string[]];
}>();

const arrowKeys = [
  { key: 'up', label: '↑' },
  { key: 'left', label: '←' },
  { key: 'down', label: '↓' },
  { key: 'right', label: '→' },
];

const commonKeys = [
  { key: 'space', label: 'Space' },
  { key: 'enter', label: 'Enter' },
  { key: 'escape', label: 'Esc' },
  { key: 'tab', label: 'Tab' },
];

const comboKeys = [
  { keys: ['ctrl', 'c'], label: 'Ctrl+C' },
  { keys: ['ctrl', 'v'], label: 'Ctrl+V' },
  { keys: ['ctrl', 'a'], label: 'Ctrl+A' },
  { keys: ['ctrl', 'z'], label: 'Ctrl+Z' },
  { keys: ['ctrl', 's'], label: 'Ctrl+S' },
  { keys: ['alt', 'f4'], label: 'Alt+F4' },
];

const modifierKeys = [
  { keys: ['shift', 'left'], label: 'Shift+←' },
  { keys: ['shift', 'right'], label: 'Shift+→' },
];

function handleKeyClick(key: string) {
  emit('keyPress', key);
}

function handleComboClick(keys: string[]) {
  emit('comboPress', keys);
}
</script>

<template>
  <div class="keypad-grid">
    <!-- Direction Keys -->
    <div class="key-section">
      <h4>方向键</h4>
      <div class="arrow-keys">
        <div class="arrow-row">
          <button class="key-btn" @click="handleKeyClick('up')">{{ arrowKeys[0].label }}</button>
        </div>
        <div class="arrow-row">
          <button class="key-btn" @click="handleKeyClick('left')">{{ arrowKeys[1].label }}</button>
          <button class="key-btn" @click="handleKeyClick('down')">{{ arrowKeys[2].label }}</button>
          <button class="key-btn" @click="handleKeyClick('right')">{{ arrowKeys[3].label }}</button>
        </div>
      </div>
    </div>

    <!-- Common Keys -->
    <div class="key-section">
      <h4>常用键</h4>
      <div class="common-keys">
        <button
          v-for="item in commonKeys"
          :key="item.key"
          class="key-btn"
          @click="handleKeyClick(item.key)"
        >
          {{ item.label }}
        </button>
      </div>
    </div>

    <!-- Combo Keys -->
    <div class="key-section">
      <h4>组合键</h4>
      <div class="combo-keys">
        <button
          v-for="item in comboKeys"
          :key="item.label"
          class="key-btn combo"
          @click="handleComboClick(item.keys)"
        >
          {{ item.label }}
        </button>
      </div>
    </div>

    <!-- Shift Combinations -->
    <div class="key-section">
      <h4>Shift 组合</h4>
      <div class="combo-keys">
        <button
          v-for="item in modifierKeys"
          :key="item.label"
          class="key-btn combo"
          @click="handleComboClick(item.keys)"
        >
          {{ item.label }}
        </button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.keypad-grid {
  display: flex;
  flex-direction: column;
  gap: 20px;
  padding: 16px;
}

.key-section h4 {
  margin: 0 0 10px 0;
  font-size: 12px;
  font-weight: 600;
  color: var(--color-text-secondary);
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.arrow-keys {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 4px;
}

.arrow-row {
  display: flex;
  gap: 4px;
}

.key-btn {
  min-width: 48px;
  height: 40px;
  padding: 0 12px;
  font-size: 14px;
  font-weight: 500;
  background: var(--color-surface-secondary);
  color: var(--color-text);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-md);
  cursor: pointer;
  transition: all 0.15s;
}

.key-btn:hover {
  background: var(--color-hover);
  border-color: var(--color-primary);
}

.key-btn:active {
  background: var(--color-primary);
  color: white;
}

.common-keys,
.combo-keys {
  display: flex;
  flex-wrap: wrap;
  gap: 6px;
}

.combo .key-btn {
  min-width: 60px;
  font-size: 12px;
}
</style>
