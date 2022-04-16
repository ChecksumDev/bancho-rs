use serde::{Deserialize, Serialize};

pub type ClientPacketData = Vec<u8>;
pub struct Player; // TODO: actually make this

pub struct PacketError {
    pub packet_id: u8,          // Packet ID
    pub data: ClientPacketData, // The data that caused the error
    pub message: String,        // The error message
}

/// The base packet for all packets sent and received.
pub trait BasePacket {
    fn handle(&self, p: Player) -> Result<(), PacketError>;
    fn to_bytes(&self) -> Vec<u8>;
}

pub struct OsuChangeAction {
    pub id: u32,
    pub action: u8,
    pub beatmap_id: u32,
    pub beatmap_set_id: u32,
    pub mods: u32,
}

impl OsuChangeAction {
    pub fn new(action: u8, beatmap_id: u32, beatmap_set_id: u32, mods: u32) -> Self {
        Self {
            id: 0,
            action,
            beatmap_id,
            beatmap_set_id,
            mods,
        }
    }
}

impl BasePacket for OsuChangeAction {
    fn handle(&self, p: Player) -> Result<(), PacketError> {
        Ok(())
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut r = Vec::new();
        // r.extend(write_u32(self.id));
        // r.extend(write_u8(self.action));
        // r.extend(write_u32(self.beatmap_id));
        // r.extend(write_u32(self.beatmap_set_id));
        // r.extend(write_u32(self.mods));
        r
    }
}


#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(i16)]
pub enum Packets {
    OsuChangeAction = 0,
    OsuSendPublicMessage = 1,
    OsuLogout = 2,
    OsuRequestStatusUpdate = 3,
    OsuPing = 4,
    ChoUserId = 5,                  //
    ChoSendMessage = 7,             //
    ChoPong = 8,                    //
    ChoHandleIrcChangeUsername = 9, //
    ChoHandleIrcQuit = 10,
    ChoUserStats = 11,       //
    ChoUserLogout = 12,      //
    ChoSpectatorJoined = 13, //
    ChoSpectatorLeft = 14,   //
    ChoSpectateFrames = 15,  //
    OsuStartSpectating = 16,
    OsuStopSpectating = 17,
    OsuSpectateFrames = 18,
    ChoVersionUpdate = 19, //
    OsuErrorReport = 20,
    OsuCantSpectate = 21,
    ChoSpectatorCantSpectate = 22, //
    ChoGetAttention = 23,          //
    ChoNotification = 24,          //
    OsuSendPrivateMessage = 25,
    ChoUpdateMatch = 26,
    ChoNewMatch = 27,
    ChoDisposeMatch = 28, //
    OsuPartLobby = 29,
    OsuJoinLobby = 30,
    OsuCreateMatch = 31,
    OsuJoinMatch = 32,
    OsuPartMatch = 33,
    ChoToggleBlockNonFriendDms = 34, //
    ChoMatchJoinSuccess = 36,
    ChoMatchJoinFail = 37, //
    OsuMatchChangeSlot = 38,
    OsuMatchReady = 39,
    OsuMatchLock = 40,
    OsuMatchChangeSettings = 41,
    ChoFellowSpectatorJoined = 42, //
    ChoFellowSpectatorLeft = 43,   //
    OsuMatchStart = 44,
    ChoAllPlayersLoaded = 45,
    ChoMatchStart = 46,
    OsuMatchScoreUpdate = 47,
    ChoMatchScoreUpdate = 48,
    OsuMatchComplete = 49,
    ChoMatchTransferHost = 50, //
    OsuMatchChangeMods = 51,
    OsuMatchLoadComplete = 52,
    ChoMatchAllPlayersLoaded = 53, //
    OsuMatchNoBeatmap = 54,
    OsuMatchNotReady = 55,
    OsuMatchFailed = 56,
    ChoMatchPlayerFailed = 57, //
    ChoMatchComplete = 58,     //
    OsuMatchHasBeatmap = 59,
    OsuMatchSkipRequest = 60,
    ChoMatchSkip = 61,    //
    ChoUnauthorized = 62, // unused
    OsuChannelJoin = 63,
    ChoChannelJoinSuccess = 64, //
    ChoChannelInfo = 65,        //
    ChoChannelKick = 66,        //
    ChoChannelAutoJoin = 67,    //
    OsuBeatmapInfoRequest = 68,
    ChoBeatmapInfoReply = 69,
    OsuMatchTransferHost = 70,
    ChoPrivileges = 71,  //
    ChoFriendsList = 72, //
    OsuFriendAdd = 73,
    OsuFriendRemove = 74,
    ChoProtocolVersion = 75, //
    ChoMainMenuIcon = 76,    //
    OsuMatchChangeTeam = 77,
    OsuChannelPart = 78,
    OsuReceiveUpdates = 79,
    ChoMonitor = 80,            // unused //
    ChoMatchPlayerSkipped = 81, //
    OsuSetAwayMessage = 82,
    ChoUserPresence = 83, //
    OsuIrcOnly = 84,
    OsuUserStatsRequest = 85,
    ChoRestart = 86, //
    OsuMatchInvite = 87,
    ChoMatchInvite = 88,    //
    ChoChannelInfoEnd = 89, //
    OsuMatchChangePassword = 90,
    ChoMatchChangePassword = 91, //
    ChoSilenceEnd = 92,          //
    OsuTournamentMatchInfoRequest = 93,
    ChoUserSilenced = 94,       //
    ChoUserPresenceSingle = 95, //
    ChoUserPresenceBundle = 96, //
    OsuUserPresenceRequest = 97,
    OsuUserPresenceRequestAll = 98,
    OsuToggleBlockNonFriendDms = 99,
    ChoUserDmBlocked = 100,          //
    ChoTargetIsSilenced = 101,       //
    ChoVersionUpdateForced = 102,    //
    ChoSwitchServer = 103,           //
    ChoAccountRestricted = 104,      //
    ChoRtx = 105,                    // unused //
    ChoMatchAbort = 106,             //
    ChoSwitchTournamentServer = 107, //
    OsuTournamentJoinMatchChannel = 108,
    OsuTournamentLeaveMatchChannel = 109,
}

// -===========-
// -= Packets =-
// -===========-

//#[derive(Debug, Clone, Serialize, Deserialize)]
