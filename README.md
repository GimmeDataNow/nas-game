# NasGame

# TODO

Change the name under which the game banners/icons/grids are being saved. (issues to to capital letters and mismatches. (such as FEZ))
the name that is displayed on the libaray card must exactly match the image name other wise it will fail.
If the request produces an image under a slightly different name such as game: EDITION then it will always attempt to re-download the images even if they might be present under a different name.

stop using env::current_dir()
might cause trouble in the future

## API access
- Steam (🟥 high)
- Gog (🟧medium)
- Epic Games (🟩low)
- others (plugins?) (⬜super low)

## Client

- gui interface (🟥 high)
- game download from nas (🟧medium)
- start games (through cli args and configs) (🟨medium-low)
- stat tracker (🟩low)
- better game starting (built in tmux or the like with console) (⬜super low)
- combine duplicate games (⬜super low)

##  Auth

- use https (🟩low)
- make sure each action requires some auth (maybe trough things like ssh-like-keys) (🟩low)
- make multiple accounts (⬜super low)


## Image compression
Assuming that the maximum size of the cover art will be 367x551px with no less than 5 items per row:
compression is now done using the server and it isn't done on the client (prev: curtail)

# Steam API

Sourced from https://developer.valvesoftware.com/wiki/Steam_Web_API


### GetGlobalAchievementPercentagesForApp (v0002)
Returns on global achievements overview of a specific game in percentages.
### GetPlayerSummaries (v0002)
Returns basic profile information for a list of 64-bit Steam IDs.
### GetPlayerAchievements (v0001)
Returns a list of achievements for this user by app id 
### GetOwnedGames (v0001)
GetOwnedGames returns a list of games a player owns along with some playtime information, if the profile is publicly visible. Private, friends-only, and other privacy settings are not supported unless you are asking for your own personal details (ie the WebAPI key you are using is linked to the steamid you are requesting). 
### GetRecentlyPlayedGames (v0001)
GetRecentlyPlayedGames returns a list of games a player has played in the last two weeks, if the profile is publicly visible. Private, friends-only, and other privacy settings are not supported unless you are asking for your own personal details (ie the WebAPI key you are using is linked to the steamid you are requesting). 
