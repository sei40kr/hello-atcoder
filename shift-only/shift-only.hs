-- shift-only.hs --- Shift Only

-- author: Seong Yong-ju <sei40kr@gmail.com>

f :: Int -> Int
f a =
  if mod a 2 == 0
    then 1 + f (quot a 2)
    else 0

solve :: [Int] -> Int
solve = minimum . map f

main :: IO ()
main = do
  n <- (read :: String -> Int) <$> getLine
  as <- map (read :: String -> Int) . take n . words <$> getLine
  print $ solve as
