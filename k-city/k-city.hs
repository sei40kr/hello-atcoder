-- k-city.hs --- K-City

-- author: Seong Yong-ju <sei40kr@gmail.com>

solve :: Int -> Int -> Int
solve n m = (n - 1) * (m - 1)

main :: IO ()
main = do
  [n, m] <- map (read :: String -> Int) . words <$> getLine
  print $ solve n m
