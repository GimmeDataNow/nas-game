# NasGame

# TODO

## API access
- Steam (🟥 high)
- Gog (🟨medium)
- Epic Games (🟩low)
- others (plugins?) (⬜super low)

## Client

- gui interface (🟥 high)
- game download from nas (🟨medium)
- start games (through cli args and configs) (🟧medium-low)
- better game starting (built in tmux or the like with console) (⬜super low)
- stat tracker (🟩low)
- combine duplicate games (⬜super low)

##  Auth

- use https (🟩low)
- make sure each action requires some auth (maybe trough things like ssh-like-keys) (🟩low)
- make multiple accounts (⬜super low)


# Steam API

Sourced from https://developer.valvesoftware.com/wiki/Steam_Web_API


# GetGlobalAchievementPercentagesForApp (v0002)
Returns on global achievements overview of a specific game in percentages.
# GetPlayerSummaries (v0002)
Returns basic profile information for a list of 64-bit Steam IDs.
# GetPlayerAchievements (v0001)
Returns a list of achievements for this user by app id 
# GetOwnedGames (v0001)
GetOwnedGames returns a list of games a player owns along with some playtime information, if the profile is publicly visible. Private, friends-only, and other privacy settings are not supported unless you are asking for your own personal details (ie the WebAPI key you are using is linked to the steamid you are requesting). 
# GetRecentlyPlayedGames (v0001)
GetRecentlyPlayedGames returns a list of games a player has played in the last two weeks, if the profile is publicly visible. Private, friends-only, and other privacy settings are not supported unless you are asking for your own personal details (ie the WebAPI key you are using is linked to the steamid you are requesting). 
