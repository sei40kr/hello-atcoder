-- daydream.hs ---

-- author: Seong Yong-ju <sei40kr@gmail.com>

solve ""                              = True
solve ('m':'a':'e':'r':'d':s)         = solve s
solve ('r':'e':'m':'a':'e':'r':'d':s) = solve s
solve ('e':'s':'a':'r':'e':s)         = solve s
solve ('r':'e':'s':'a':'r':'e':s)     = solve s
solve _                               = False

main :: IO ()
main = do
  s <- getLine
  putStrLn $ if solve $ reverse s then "YES" else "NO"
