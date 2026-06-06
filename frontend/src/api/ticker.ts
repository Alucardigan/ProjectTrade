import axios from 'axios';

export const fetchTickerHistory = async (ticker: string, timeframe: string) => {
  try {
    const response = await axios.get(`/api/tickers/${ticker}/history`, {
      params: { timeframe }
    });
    return response.data;
  } catch (error) {
    console.error(`Failed to fetch history for ${ticker}:`, error);
    throw error;
  }
};

export const fetchTicker = async (ticker: string) => {
  try {
    const response = await axios.get(`/api/tickers/${ticker}`);
    return response.data[0]; // Returns array of 1
  } catch (error) {
    console.error(`Failed to fetch ticker ${ticker}:`, error);
    throw error;
  }
};
