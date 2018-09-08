-- abc333.hs --- ABC333

-- author: Seong Yong-ju <sei40kr@gmail.com>

main :: IO ()
main = do
  [a, b] <- map (read :: String -> Int) . take 2 . words <$> getLine
  putStrLn $
    if a `mod` 2 == 1 && b `mod` 2 == 1
      then "Yes"
      else "No"
