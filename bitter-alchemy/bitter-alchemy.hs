import           Control.Monad

-- bitter-alchemy.hs --- Bitter Alchemy

-- author: Seong Yong-ju <sei40kr@gmail.com>

solve :: Int -> Int -> [Int] -> Int
solve n x ms = n + quot (x - sum ms) (minimum ms)

main :: IO ()
main = do
  [n, x] <- map (read :: String -> Int) . words <$> getLine
  ms <- replicateM n $ (read :: String -> Int) <$> getLine
  print $ solve n x ms
