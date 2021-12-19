# ⚙️ ``iengine``
Binary module that adds engine functions from IEngineClientV015 to the ``iengine`` library

## iengine.concmd(command: string)
Runs a concommand on yourself, internally calls ``ExecuteClientCmd``

## iengine.getResolution() -> number, number
Returns your screen resolution, internally using ``GetScreenSize``

## iengine.getGameDirectory() -> string
Returns the absolute location of your garrysmod dir, internally using ``GetGameDirectory``

## iengine.getLevel() -> string
Returns the current level/map name, internally using ``GetLevelName``

## iengine.isRecording() -> boolean
Returns ``true`` if you are recording a demo, else ``false``, internally using ``IsRecordingDemo``

## iengine.isPaused() -> boolean
Returns ``true`` if the game is paused, else ``false``, internally using ``IsPaused``
