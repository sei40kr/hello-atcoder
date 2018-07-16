-- ordinary-beauty.hs --- Ordinary Beauty

-- author: Seong Yong-ju <sei40kr@gmail.com>

solve :: Double -> Double -> Double -> Double
solve n m 0 = (m - 1) / n
solve n m d = 2 * (n - d) * (m - 1) / (n * n)

main :: IO ()
main = do
  [n, m, d] <- map (read :: String -> Double) . words <$> getLine
  print $ solve n m d
