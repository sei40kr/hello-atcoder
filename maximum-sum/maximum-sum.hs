-- maximum-sum.hs --- Maximum Sum

-- author: Seong Yong-ju <sei40kr@gmail.com>

solve :: Int -> Int -> Int -> Int -> Int
solve a b c k = max a (max b c) * (2 ^ k - 1) + a + b + c

main :: IO ()
main = do
  [a, b, c] <- map (read :: String -> Int) . words <$> getLine
  k <- (read :: String -> Int) <$> getLine
  print $ solve a b c k
