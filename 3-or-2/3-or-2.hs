-- 3-or-2.hs --- *3 or /2

-- author: Seong Yong-ju

f :: Int -> Int
f x =
  if x `mod` 2 == 0
    then 1 + f (x `quot` 2)
    else 0

solve :: [Int] -> Int
solve as = sum $ map f as

main :: IO ()
main = do
  n <- readLn :: IO Int
  as <- map (read :: String -> Int) . take n . words <$> getLine
  print $ solve as
