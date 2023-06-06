module Opts where

import Model

data Task = 
  Serve Int
  | RefreshDB String
  | InsertItem ContestItem