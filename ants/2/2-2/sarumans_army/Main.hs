
{-# LANGUAGE ScopedTypeVariables #-}

import Data.List (sort)
import Control.Arrow (first)
import Data.Maybe (listToMaybe)
import Debug.Trace (trace)

main :: IO ()
main = do
  [n,r] <- map readInt . words <$> getLine
  xs <- map readInt . words <$> getLine
  print $ saruman n r $ sort xs

readInt :: String -> Int
readInt = read

saruman :: Int -> Int -> [Int] -> [Int]
saruman _ _ [] = []
saruman _ r (x:xs) = f x xs
  where
    f y ys = case dropWhileAndLast (<=y+r) ys of
      (Nothing, zs) -> trace "None" $ y: f' zs
      (Just z,  zs) -> trace "Some" $ z: f' (dropWhile (<=z+r) zs)
    f' [] = []
    f' (y:ys) = f y ys

dropWhileAndLast :: (a -> Bool) -> [a] -> (Maybe a, [a])
dropWhileAndLast p xs = first (listToMaybe.reverse) $ span p xs
--dropWhileAndLast _ [] = (Nothing, [])
--dropWhileAndLast p (x:xs)
--  | not (p x) = (Nothing, (x:xs))
--  | otherwise = case dropWhileAndLast p xs of
--      (Nothing, xs') -> (Just x, xs')
--      other          -> other




