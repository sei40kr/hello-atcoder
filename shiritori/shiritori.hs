{-# LANGUAGE FlexibleContexts #-}

import           Control.Monad
import           Data.List

-- shiritori.hs --- Shiritori

-- author: Seong Yong-ju <sei40kr@gmail.com>

main :: IO ()
main = do
  n <- readLn
  words <- replicateM n getLine
  putStrLn $
    if (length $ nub words) == length words &&
       all (\(w1, w2) -> last w1 == w2 !! 0) (zip (init words) (tail words))
      then "Yes"
      else "No"
