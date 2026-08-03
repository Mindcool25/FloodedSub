# Flooded Sub
Flooded Submarine is (intended) to be a lightweight OpenSubsonic server written in rust.
The main thing that this software aims to do is create a bit more of a secure way of accessing a subsonic server. The users can get an "API key", which is just used as the password for the subsonic server, while being able to have authentication done by stronger methods (hopefully including OIDC)

## Features (and plans)
transcoding will be done by just calling the `ffmpeg` command line tool for now until there is a reason to not do that

Use lofty for metadata extraction

## Types
### Child (usually song)
- id (required)
- isDir (required, defaults to false)
- title (required)
- parent
- album
- artist
- track
- year
- coverArt
- size
- contentType
- suffix
- starred
- duration
- bitRate
- samplingRate
- channelCount
- path
- playCount
- played
- discNumber
- created
- albumId
- artistId
- type
- mediaType
- isVideo
- bpm
- user rating
- comment
- sortName
- musicBrainzId
- genre
- genres (list, optional)
    - name
- artists (list, optional, ID3 format)
    - id
    - name
- displayArtist
- albumArtists (list)
    - id
    - name
- displayAlbumArtist
- contributors (list)
    - role
    - artist
        - id
        - name
- displayComposer
- moods
- replayGain
    - trackGain
    - albumGain
    - trackPeak
    - album Peak
    - baseGain

## Database schema
Everything is optionl unless specified. Optional should always be an empty or default value.
### Tables
- Media  
Based off of https://opensubsonic.netlify.app/docs/responses/child/
    * id: str (req)
    * parent: str
    * title: str (req)
    * album: str
    * artist: str
    * track: num
    * year: num
    * genre: str
    * coverArt: str
    * size: num
    * contentType: str
    * suffix: str (file suffix)
    * duration: num
    * bitRate: num
    * bitDepth: num
    * samplingRate: num
    * channel count: num
    * path: str
    * played: str (date)
    * discNubmer: num
    * created: str (date)
    * starred: str (date)
    * albumId: str
    * artistId: str
    * type: str
    * mediaType: str
    * isVideo: bool
    * bpm: num
    * comment: str
    * sortName: str
    * musicBrainzId: str
    * genres: json list of names
    * artists: json list of artist IDs and names
    * displayArtist: str
    * albumArtists: json list of artis IDs and names
    * displayAlbumArtists: str
    * contributors: json list of role and sometimes subrole, then artist elementA
    * displayComposer: str
    * explicit status: str ("explicit", "clean" or "", can be gotten from ITUNESADVISORY, prolly others)
    * moods: json list of moods
    * trackGain: num | These are stored as a single element on return value
    * albumGain: num | it might also be worth storing album stuff in the album
    * trackPeak: num | and getting that info on request
    * albumPeak: num |
    * baseGain: num  |

- Albums  
Based off of https://opensubsonic.netlify.app/docs/responses/albumid3/
    * id: str (req)
    * name: str (req)
    * version: str
    * artist: str
    * artistId: str
    * coverArt: str
    * songCount: num
    * duration: num - in seconds
    * playCount: num
    * created: str (date, req) - Date added
    * starred: str (date)
    * year: num
    * genre: str
    * played: str (date) - date last played
    * musicBrainzId: str
    * genres: list of genre names
    * artists: list of artist IDs
    * displayArtist: str
    * releaseTypes: list of str
    * moods: list of str
    * sortName: string
    * originalReleaseDate: str (date)
    * explicit status: str ("explicit", "clean" or "")

- Artist  
based off of https://opensubsonic.netlify.app/docs/responses/artistid3/
    * id: str (req)
    * name: str (req)
    * coverArt: str
    * artistImageUrl: str
    * albumCount: num
    * starred: str (date)
    * musicBrainzId: str
    * sortName: str

- Playlists  
based off of navidrome's playlist storage
    * id: str (req)
    * name: str (req)
    * comment: str
    * owner: str (user id)
    * public: bool (default to false?)
    * songCount: num (req)
    * duration: num (req)
    * created: str (req, date)
    * changed: str (req, date)
    * coverArt: str
    * allowedUser: list of user IDs (maybe?)
    * readonly: bool (true if the playlist cannot be edited by current user)

- PlaylistTracks  
    * playlistId: key (req)
    * mediaId: key (req)

- user  
based off of https://opensubsonic.netlify.app/docs/responses/user/
    * id: str (req)
    * username: str (req)
    * password: str (req, hashed)
    * scrobblingEnabled: bool (req)
    * maxBitRate: num
    * adminRole: bool (req)
    * settingsRole: bool (req)
    * downloadRole: bool (req)
    * uploadRole: bool (req)
    * playlistRole: bool (req)
    * coverArtRole: bool (req)
    * commentRole: bool (req)
    * podcastRole: bool (req)
    * streamRole: bool (req)
    * jukeboxRole: bool (req)
    * shareRole: bool (req)
    * videoConversionRole: bool (req)
    * avatarLastChanged: str (date)

- ClientKeys  
    * userId: str (req)
    * key: str (req, hashed) - Used for opensubsonic authentication, randomly generated

- Annotations  
based off of how navidrome does these
    * userId: str (req)
    * itemId: str (req)
    * itemType: str (req) - should be album, song, artist, etc.
    * playCount: num (req)
    * playDate: str (req, date) - last played date
    * rating: num (req, default 0)
    * starred: bool (req)
    * starredAt: str (date)
    * ratedAt: str (date)
