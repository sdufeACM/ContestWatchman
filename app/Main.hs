{-# LANGUAGE OverloadedStrings #-}

import Application ()
-- for YesodDispatch instance
import Foundation
import Options.Applicative (execParser, fullDesc, info)
import Opts
import Yesod.Core
import Model
import Database.Persist.Sqlite (runSqlite)
import Control.Monad.IO.Class (liftIO)

exec :: CWTask -> IO ()
exec (Serve port) = putStrLn ("contest.db" ++ show port) >> warp port App
exec (InsertItem (ContestItem start_time duration source name link)) = runSqlite ":memory:" $  do
  liftIO $ print "123"
exec _ = undefined

main :: IO ()
main = exec =<< execParser opts
  where
    opts = info cwOpts fullDesc
