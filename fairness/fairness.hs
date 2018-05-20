-- fairness.hs --- Fairness

-- author: Seong Yong-ju <sei40kr@gmail.com>

solve :: Integer -> Integer -> Integer -> Integer -> Maybe Integer
solve a b _ k =
  if abs ans <= ((10 :: Integer) ^ (18 :: Integer))
    then Just ans
    else Nothing
  where
    ans =
      (a - b) *
      (if odd k
         then -1
         else 1)

main :: IO ()
main = do
  [a, b, c, k] <- map (read :: String -> Integer) . words <$> getLine
  putStrLn $ maybe "Unfair" show $ solve a b c k
