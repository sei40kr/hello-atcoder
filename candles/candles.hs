-- candles.hs --- Candles

-- author: Seong Yong-ju <sei40kr@gmail.com>

main :: IO ()
main = do
  [n, k] <- map (read :: String -> Int) . words <$> getLine
  xs <- map (read :: String -> Integer) . take n . words <$> getLine
  print $ minimum $ zipWith f xs $ drop (k - 1) xs
  where
    f l r
      | r < 0 = -l
    f l r
      | 0 < l = r
    f l r = min (-l * 2 + r) (-l + r * 2)
