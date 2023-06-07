module Opts where

import Data.Semigroup ((<>))
import Model
import Options.Applicative

data CWTask
  = Serve Int
  | RefreshDB String
  | InsertItem ContestItem

serveCommand :: Parser CWTask
serveCommand =
  Serve
    <$> option
      auto
      ( long "port"
          <> short 'p'
          <> metavar "PORT"
          <> value 3000
          <> help "Server port to be binding"
      )
    <**> helper

refreshCommand :: Parser CWTask
refreshCommand =
  RefreshDB
    <$> strOption
      ( metavar "URL"
          <> value "https://contests.sdutacm.cn/contests.json"
          <> help "Recent contest source, see https://github.com/MeiK2333/recent_contests for more infomation"
      )
    <**> helper

insertCommand :: Parser CWTask
insertCommand = fmap InsertItem itemParser
  where
    itemParser =
      ContestItem
        <$> option
          auto
          ( long "start"
              <> metavar "TIMESTAMP"
              <> help "Contest start time"
          )
        <*> option
          auto
          ( long "duration"
              <> metavar "MILLSECOND"
              <> value 0
              <> help "Contest lasted time"
          )
        <*> strOption
          ( long "source"
              <> short 's'
              <> help "Contest source"
          )
        <*> strOption
          ( long "name"
              <> short 'n'
              <> help "Contest name"
          )
        <*> optional
          ( strOption
              ( long "link"
                  <> short 'l'
                  <> help "Contest link"
              )
          )
        <**> helper

cwOpts :: Parser CWTask
cwOpts =
  subparser
    ( command "serve" (info serveCommand (fullDesc <> progDesc "Start server"))
        <> command "refresh" (info refreshCommand (fullDesc <> progDesc "Refresh contest info from remote"))
        <> command "insert" (info insertCommand (fullDesc <> progDesc "Insert local contest info"))
    )
    <**> helper