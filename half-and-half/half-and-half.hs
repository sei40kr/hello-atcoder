-- half-and-half.hs --- Half and Half

-- author: Seong Yong-ju <sei40kr@gmail.com>

solve :: Int -> Int -> Int -> Int -> Int -> Int
solve a b c x y = a_min * p + b_min * q + c2_min * r2
  where
    a_min = min a $ 2 * c
    b_min = min b $ 2 * c
    c2_min = min (2 * c) (a + b)
    r2 = min x y
    p = x - r2
    q = y - r2

main :: IO ()
main = do
  [a, b, c, x, y] <- map (read :: String -> Int) . words <$> getLine
  print $ solve a b c x y
