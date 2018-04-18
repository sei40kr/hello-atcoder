-- walk-and-teleport.hs --- Walk and Teleport

-- author: Seong Yong-ju <sei40kr@gmail.com>

solve :: Int -> Int -> [Int] -> Int
solve _ _ []        = 0
solve _ _ [_]       = 0
solve a b (x:x':xs) = min (a * (x' - x)) b + solve a b (x' : xs)

main :: IO ()
main = do
  [n, a, b] <- map (read :: String -> Int) . words <$> getLine
  xs <- map (read :: String -> Int) . take n . words <$> getLine
  print $ solve a b xs
