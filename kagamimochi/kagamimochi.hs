import           Control.Monad
import           Data.List

-- kagamimochi.hs --- Kagamimochi

-- author: Seong Yong-ju <sei40kr@gmail.com>

solve :: [Int] -> Int
solve = length . nub

main :: IO ()
main = do
  n <- (read :: String -> Int) <$> getLine
  rs <- replicateM n $ (read :: String -> Int) <$> getLine
  print $ solve rs
