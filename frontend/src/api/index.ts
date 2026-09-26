import axios from 'axios';

const apiClient = axios.create({
  baseURL: 'http://localhost:8080/api', // 根據實際環境調整
  headers: {
    'Content-Type': 'application/json',
  },
});

export const getCardInfo = (cardUid: string) => {
  return apiClient.get(`/card/info?card_uid=${card_uid}`);
};

export const postTopup = (cardUid: string, amount: number) => {
  return apiClient.post('/topup', { card_uid, amount });
};

export const getTransactions = () => {
  return apiClient.get('/transactions');
};

export const getSystemStatus = () => {
  return apiClient.get('/status');
};
