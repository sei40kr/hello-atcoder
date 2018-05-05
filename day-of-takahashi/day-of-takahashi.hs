-- day-of-takahashi.hs --- Day of Takahashi

-- author: Seong Yong-ju <sei40kr@gmail.com>

solve :: Int -> Int -> Int
solve a b =
  a +
  (if a <= b
     then 0
     else -1)

main :: IO ()
main = do
  [a, b] <- map (read :: String -> Int) . words <$> getLine
  print $ solve a b
