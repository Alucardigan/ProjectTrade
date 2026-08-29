import axios from 'axios';
import type { TickerSummary, TickerCandle } from '../types/TickerSummary';

export const fetchAllTickers = async (): Promise<TickerSummary[]> => {
  try {
    const response = await axios.get<TickerSummary[]>('/api/tickers');
    return response.data;
  } catch (error) {
    console.error('Failed to fetch all tickers:', error);
    throw error;
  }
};

export const fetchTickerDetails = async (ticker: string): Promise<TickerSummary> => {
  try {
    const response = await axios.get<TickerSummary>(`/api/tickers/${ticker}/details`);
    return response.data;
  } catch (error) {
    console.error(`Failed to fetch details for ${ticker}:`, error);
    throw error;
  }
};

export const fetchTickerHistory = async (ticker: string, timeframe: string): Promise<TickerCandle[]> => {
  try {
    const response = await axios.get<TickerCandle[]>(`/api/tickers/${ticker}/history`, {
      params: { timeframe }
    });
    return response.data;
  } catch (error) {
    console.error(`Failed to fetch history for ${ticker}:`, error);
    throw error;
  }
};

export const fetchTicker = async (ticker: string): Promise<TickerCandle> => {
  try {
    const response = await axios.get<TickerCandle[]>(`/api/tickers/${ticker}`);
    return response.data[0];
  } catch (error) {
    console.error(`Failed to fetch ticker ${ticker}:`, error);
    throw error;
  }
};
