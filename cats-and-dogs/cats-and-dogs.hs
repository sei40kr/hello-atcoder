-- cats-and-dogs.hs --- Cats and Dogs

-- author: Seong Yong-ju <sei40kr@gmail.com>

solve :: Int -> Int -> Int -> Bool
solve a b x = a <= x && x <= a + b

main :: IO ()
main = do
  [a, b, x] <- map (read :: String -> Int) . words <$> getLine
  putStrLn $ if solve a b x then "YES" else "NO"
