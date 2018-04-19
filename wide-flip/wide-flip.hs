-- wide-flip.hs --- Wide Flip

-- author: Seong Yong-ju <sei40kr@gmail.com>

solve :: Int -> Int -> Int -> String -> Int
solve _ _ t [] = t
solve _ _ t [_] = t
solve n k t (c:c':cs)
  | c /= c' = solve n (succ k) (min t . max k $ n - k) (c' : cs)
  | otherwise = solve n (succ k) t (c' : cs)

main :: IO ()
main = do
  s <- getLine
  let l = length s
  print $ solve l 1 l s
