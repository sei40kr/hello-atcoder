-- skip.hs --- Skip

-- author: Seong Yong-ju <sei40kr@gmail.com>

main :: IO ()
main = do
  [n, x0] <- map (read :: String -> Int) . take 2 . words <$> getLine
  xs <- map (read :: String -> Int) . take n . words <$> getLine
  let ds = map (abs . (-x0 +)) xs
  print $ foldl gcd (head ds) (tail ds)
