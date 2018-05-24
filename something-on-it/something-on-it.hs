-- something-on-it.hs --- Something on It

-- author: Seong Yong-ju <sei40kr@gmail.com>

solve :: Int -> Int
solve n = 700 + (100 * n)

main :: IO ()
main = do
  n <- length . filter (== 'o') <$> getLine
  print $ solve n
