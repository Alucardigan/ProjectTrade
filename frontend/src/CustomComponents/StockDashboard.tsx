import { useState, useEffect } from "react"
import { Card, CardHeader, CardTitle, CardContent } from '@/components/ui/card'
import { Table, TableHeader, TableRow, TableHead, TableBody, TableCell } from '@/components/ui/table'
import { Skeleton } from '@/components/ui/skeleton'
import { Button } from '../components/ui/button'
import { Input } from '@/components/ui/input'
import {
  LineChart,
  Line,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  Legend,
  ResponsiveContainer
} from 'recharts'
import axios from "axios"
interface trendData{
  date:string, 
  resPrice: number
}
interface StockData{
  symbol: string,
  price : string,
  trend : number[],
  line : trendData[]
}
const majorIndices = [
  { symbol: '^GSPC', name: 'S&P 500' },
  { symbol: '^DJI', name: 'Dow Jones' },
  { symbol: '^IXIC', name: 'NASDAQ' },
  { symbol: '^FTSE', name: 'FTSE 100' },
  { symbol: '^GDAXI', name: 'DAX' }
]
export default function StockDashboard() {
  const [stocks, setStocks] = useState<StockData[]>([])
  const [loading, setLoading] = useState(true)
  const [searchTerm, setSearchTerm] = useState('')
  const [error, setError] = useState<string | null>(null)
  const fetchStockData = async () => {
      try {
        setLoading(true)
        const response = await axios.get("/api/tickers")
        const stockDataArray:StockData[] = response.data
        for(let i = 0; i < stockDataArray.length; i++){
          stockDataArray[i].line = []
          for(let j = 0; j < stockDataArray[i].trend.length;j++){
            let d = new Date();
            const date = new Date(d);
            date.setDate(date.getDate() + j);
            stockDataArray[i].line.push({date: date.toISOString().slice(0, 10), resPrice: stockDataArray[i].trend[j]})
          }
        }
        console.log(stockDataArray)
        setStocks(stockDataArray)
      } catch (err) {
        setError('Failed to fetch stock data. Please try again later.')
        console.error('Error fetching stock data:', err)
      } finally {
        setLoading(false)
      }
    }
  useEffect(() => {
    fetchStockData()
    console.log(stocks)
  }, [])


  if (error) {
    return (
      <div className="p-6">
        <Card>
          <CardHeader>
            <CardTitle>Error</CardTitle>
          </CardHeader>
          <CardContent>
            <p className="text-red-500">{error}</p>
            <Button 
              className="mt-4"
              onClick={() => window.location.reload()}
            >
              Retry
            </Button>
          </CardContent>
        </Card>
      </div>
    )
  }

  return (
    <div className="p-6 space-y-6">
      <div className="flex justify-between items-center">
        <h1 className="text-3xl font-bold">Market Indices</h1>
        <div className="w-1/3">
          <Input
            placeholder="Search indices..."
            value={searchTerm}
            onChange={(e) => setSearchTerm(e.target.value)}
          />
        </div>
      </div>

      {loading ? (
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          {[...Array(5)].map((_, i) => (
            <Card key={i}>
              <CardHeader>
                <Skeleton className="h-6 w-3/4" />
              </CardHeader>
              <CardContent className="space-y-4">
                <Skeleton className="h-4 w-full" />
                <Skeleton className="h-4 w-1/2" />
                <Skeleton className="h-64 w-full" />
              </CardContent>
            </Card>
          ))}
        </div>
      ) : (
        <>
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
            {stocks.map((stock) => (
              <Card key={stock.symbol}>
                <CardHeader>
                  <CardTitle className="flex justify-between items-center">
                    <span>{stock.symbol}</span>
                    <span
                      className={
                        `text-sm font-medium ${'text-green-500'
                      }`}
                    >
                      {5} ({5})
                    </span>
                  </CardTitle>
                </CardHeader>
                <CardContent>
                  <div className="text-2xl font-bold mb-4">${stock.price}</div>
                  <div className="h-64">
                    <ResponsiveContainer width="100%" height="100%">
                      <LineChart data={stock.line}>
                        <CartesianGrid strokeDasharray="3 3" />
                        <XAxis dataKey="date" />
                        <YAxis domain={['auto', 'auto']} />
                        <Tooltip />
                        <Legend />
                        <Line
                          type="monotone"
                          dataKey="resPrice"
                          stroke="#8884d8"
                          dot={false}
                        />
                      </LineChart>
                    </ResponsiveContainer>
                  </div>
                </CardContent>
              </Card>
            ))}
          </div>

          <Card>
            <CardHeader>
              <CardTitle>Index Performance</CardTitle>
            </CardHeader>
            <CardContent>
              <Table>
                <TableHeader>
                  <TableRow>
                    <TableHead>Index</TableHead>
                    <TableHead>Symbol</TableHead>
                    <TableHead>Price</TableHead>
                    <TableHead>Change</TableHead>
                    <TableHead>% Change</TableHead>
                  </TableRow>
                </TableHeader>
                <TableBody>
                  {stocks.map((stock) => (
                    <TableRow key={stock.symbol}>
                      <TableCell className="font-medium">{stock.symbol}</TableCell>
                      <TableCell>{stock.symbol}</TableCell>
                      <TableCell>${stock.price}</TableCell>
                      <TableCell
                        className={'text-green-500'
                        }
                      >
                        {"up"}
                      </TableCell>
                      <TableCell
                        className={'text-green-500'
                        }
                      >
                        {"changePercent"
                        }
                      </TableCell>
                    </TableRow>
                  ))}
                </TableBody>
              </Table>
            </CardContent>
          </Card>
        </>
      )}
    </div>
  )
}