{-# LANGUAGE BangPatterns #-}

import           Data.Set (Set)
import qualified Data.Set as S
import           Data.Map (Map)
import qualified Data.Map as M

readInt :: String -> Int
readInt = read

getInts :: IO [Int]
getInts = map readInt . words <$> getLine

