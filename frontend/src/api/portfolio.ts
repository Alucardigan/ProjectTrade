import axios from 'axios';
import type { PortfolioResponse } from '../types/Portfolio_Response';

export const fetchPortfolio = async (): Promise<PortfolioResponse> => {
    console.log("Fetching portfolio...");
    try {
        const { data } = await axios.get('/api/portfolio');
        console.log("Portfolio data received:", data);
        return data;
    } catch (error) {
        console.error("Error fetching portfolio:", error);
        throw error;
    }
};

export interface PortfolioHistoryPoint {
    date: string;
    total_value: string;
}

export const fetchPortfolioHistory = async (timeframe: string): Promise<PortfolioHistoryPoint[]> => {
    try {
        const { data } = await axios.get(`/api/portfolio/history?timeframe=${timeframe}`);
        return data;
    } catch (error) {
        console.error("Error fetching portfolio history:", error);
        throw error;
    }
}
