-- ordinary-beauty.hs --- Ordinary Beauty

-- author: Seong Yong-ju <sei40kr@gmail.com>

solve :: Double -> Double -> Double -> Double
solve n m 0 = 1 / n * (m - 1)
solve n m d = 2 * (n - d) / (n * n) * (m - 1)

main :: IO ()
main = do
  [n, m, d] <- map (read :: String -> Double) . words <$> getLine
  print $ solve n m d
