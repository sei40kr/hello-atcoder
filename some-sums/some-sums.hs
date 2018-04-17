-- some-sums.hs ---

-- author: Seong Yong-ju <sei40kr@gmail.com>

main :: IO ()
main = do
  [n, a, b] <- map (read :: String -> Int) . words <$> getLine
  print $
    sum
      [ 10 * x + y
      | x <- [0 .. 9]
      , y <- [0 .. 9]
      , a <= x + y && x + y <= b && 10 * x + y <= n
      ]
