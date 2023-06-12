{-# LANGUAGE OverloadedStrings #-}
{-# LANGUAGE TemplateHaskell   #-}
{-# LANGUAGE QuasiQuotes       #-}
{-# LANGUAGE TypeFamilies      #-}
{-# LANGUAGE ViewPatterns      #-}
module Foundation where

import Yesod.Core

data App = App

mkYesodData "App" $(parseRoutesFile "routes.yesodroutes")
instance Yesod App where
    errorHandler NotFound = fmap toTypedContent $ defaultLayout $ do
        setTitle "404 - NotFound"
        toWidget [hamlet|
<h1> :(
<p> 你似乎来到了没有知识的荒原
<hr>
<p> It seems like you are losing ways in wasteland
|]
    errorHandler other = defaultErrorHandler other
