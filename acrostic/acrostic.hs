-- acrostic.hs --- Acrostic

-- author: Seong Yong-ju <sei40kr@gmail.com>

solve :: String -> Int -> String
solve s w = map (s !!) $ filter ((== 0) . (`mod` w)) [0 .. (length s - 1)]

main :: IO ()
main = do
  s <- getLine
  w <- (read :: String -> Int) <$> getLine
  putStrLn $ solve s w
