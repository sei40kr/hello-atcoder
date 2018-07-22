-- string-rotation.hs --- String Rotation

-- author: Seong Yong-ju <sei40kr@gmail.com>

rotate :: Int -> [x] -> [x]
rotate n xs = take (length xs) (drop n (cycle xs))

solve :: String -> String -> Bool
solve s t = any ((== t) . flip rotate s) [0 .. (length s)]

main :: IO ()
main = do
  s <- getLine
  t <- getLine
  putStrLn $
    if solve s t
      then "Yes"
      else "No"
