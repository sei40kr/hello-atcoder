-- module-summation.hs ---

-- author: Seong Yong-ju <sei40kr@gmail.com>

main :: IO ()
main = do
  n <- (read :: String -> Int) <$> getLine
  as <- map (read :: String -> Int) . take n . words <$> getLine
  print $ (sum . map (-1 +)) as
