export interface TickerSummary {
  symbol: string;
  name: string;
  sector: string;
  description: string;
  current_price: string | number;
  open_price: string | number;
  day_high: string | number;
  day_low: string | number;
  day_volume: number;
  change_percent: number;
  is_positive: boolean;
  market_cap: string | number;
}

export interface TickerCandle {
  ticker: string;
  date: string;
  close: string | number;
  volume?: number;
  open?: string | number;
  high?: string | number;
  low?: string | number;
}
