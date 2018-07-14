-- f.hs --- F

-- author: Seong Yong-ju <sei40kr@gmail.com>

main :: IO ()
main = do
  [a, b] <- map (read :: String -> Int) . words <$> getLine
  putStrLn $ case (a + b, a * b) of
    (15, _) -> "+"
    (_, 15) -> "*"
    _ -> "x"
