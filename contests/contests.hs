{-# LANGUAGE FlexibleContexts #-}
import           Control.Applicative
import           Control.Monad
import           Data.Array
import           Data.Bits
import           Data.Int
import qualified Data.IntMap.Lazy    as IntMap
import qualified Data.IntSet         as IntSet
import           Data.List
import qualified Data.Map.Lazy       as Map
import           Data.Maybe
import           Data.Ord
import qualified Data.Sequence       as Seq
import           Data.Tuple

-- contests.hs --- Contests

-- author: Seong Yong-ju <sei40kr@gmail.com>

main :: IO ()
main = do
  n <-
    (readLn :: Read String =>
                 IO Int)
  [a, b] <- map (read :: String -> Int) . words <$> getLine
  ps <- map (read :: String -> Int) . take n . words <$> getLine
  print $
    let class_a = length $ filter (<= a) ps
        class_b = length $ filter (> a) $ filter (<= b) ps
        class_c = length $ filter (> b) ps
     in class_a `min` class_b `min` class_c
