-- abs.hs --- ABS

-- author: Seong Yong-ju <sei40kr@gmail.com>

solve :: Int -> Int -> [Int] -> Int
solve z w []        = abs $ z - w
solve _ w [a]       = abs $ a - w
solve _ w (a1:a2:_) = max (abs $ a1 - w) (abs $ a1 - a2)

main :: IO ()
main = do
  [n, z, w] <- map (read :: String -> Int) . words <$> getLine
  as <- map (read :: String -> Int) . take n . words <$> getLine
  print $ solve z w $ reverse as
