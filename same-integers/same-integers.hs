-- same-integers.hs --- Same Integers

-- author: Seong Yong-ju <sei40kr@gmail.com>

solve :: Int -> Int -> Int -> Int
solve a b c =
  if mod m 2 == mod s 2
    then quot (3 * m - s) 2
    else quot (3 * (m + 1) - s) 2
  where
    s = a + b + c
    m = max a $ max b c
main :: IO ()
main = do
  [a, b, c] <- map (read :: String -> Int) . words <$> getLine
  print $ solve a b c
