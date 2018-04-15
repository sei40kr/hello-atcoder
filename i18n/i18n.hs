-- i18n.hs --- i18n

-- author: Seong Yong-ju <sei40kr@gmail.com>

solve :: String -> String
solve [] = ""
solve (c:s) = [c] ++ show (length s - 1) ++ [last s]

main :: IO ()
main = do
  s <- getLine
  putStrLn $ solve s
