<template>
  <div class="min-h-screen bg-gray-100 p-8">
    <div class="max-w-md mx-auto bg-white rounded-xl shadow-md overflow-hidden md:max-w-2xl p-6">
      <h1 class="text-2xl font-bold mb-6 text-gray-800">感應卡充值系統</h1>
      
      <!-- 卡片狀態顯示區 -->
      <div class="mb-8 p-4 border-2 border-dashed border-blue-400 rounded-lg text-center">
        <p class="text-gray-600">請將卡片放置於讀取器上</p>
        <div v-if="cardDetected" class="mt-4 p-3 bg-green-100 text-green-700 rounded">
          卡片已感應：{{ cardId }}
        </div>
      </div>

      <!-- 充值選擇區 -->
      <div v-if="cardDetected" class="space-y-4">
        <h2 class="text-xl font-semibold">選擇充值金額</h2>
        <div class="grid grid-cols-3 gap-4">
          <button 
            v-for="amount in [100, 200, 500, 1000]" 
            :key="amount"
            @click="selectAmount(amount)"
            class="p-4 border rounded-lg hover:bg-blue-50 transition"
          >
            ${{ amount }}
          </button>
        </div>

        <button 
          @click="handleTopup"
          :disabled="!selectedAmount"
          class="w-full py-3 bg-blue-600 text-white rounded-lg disabled:bg-gray-400"
        >
          立即充值
        </button>
      </div>

      <!-- 結果提示 -->
      <div v-if="message" class="mt-6 p-4 bg-blue-50 text-blue-700 rounded-lg">
        {{ message }}
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue';
import { getCardInfo, postTopup } from '../api';

const cardId = ref('');
const cardDetected = ref(false);
const selectedAmount = ref(0);
const message = ref('');

// 模擬偵測到新卡片 (實際開發中可透過 WebSocket 或 Web Serial API 聯動硬體)
const simulateCardDetection = () => {
  cardId.value = '1234567890';
  cardDetected.value = true;
};

const selectAmount = (amount) => {
  selectedAmount.value = amount;
};

const handleTopup = async () => {
  try {
    const result = await postTopup(cardId.value, selectedAmount.value);
    message.value = result.data;
  } catch (error) {
    message.value = '充值失敗，請稍後再試';
  }
};
</script>

<style scoped>
/* Tailwind 處理樣式 */
</style>
