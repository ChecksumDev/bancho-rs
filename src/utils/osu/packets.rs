use std::{collections::HashMap, sync::Arc};
use serde::{Deserialize, Serialize};
use super::packet_reader::{Message, self, PacketReader};
use futures::future::{BoxFuture, FutureExt};


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

// pub struct OsuChangeAction {
//     pub id: u32,
//     pub action: u8,
//     pub beatmap_id: u32,
//     pub beatmap_set_id: u32,
//     pub mods: u32,
// }

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
    OsuBeatmapInfoRequest = 68, // ?
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
// -= Client Packets =-
// -===========-

pub trait ClientPacket {}

pub struct OsuChangeAction {
    id: i16,
    action: u8,
    info_text: String,
    map_md5: String,
    mods: u32,
    mode: u8,
}

impl ClientPacket for OsuChangeAction {}
impl OsuChangeAction {
    pub async fn new(reader: &mut PacketReader) -> Self {
        Self {
            id: Packets::OsuChangeAction as i16,
            action: reader.read_u8(),
            info_text: reader.read_string(),
            map_md5: reader.read_string(),
            mods: reader.read_u32(),
            mode: reader.read_u8(),
        }
    }
}

pub struct OsuSendPublicMessage {
    id: i16,
    message: String,
}

impl ClientPacket for OsuSendPublicMessage {}
impl OsuSendPublicMessage {
    pub fn new(reader: &mut PacketReader) -> Self {
        Self {
            id: Packets::OsuSendPublicMessage as i16,
            message: reader.read_string(),
        }
    }
}

pub struct OsuLogout {
    id: i16,
}

impl ClientPacket for OsuLogout {}
impl OsuLogout {
    pub fn new() -> Self {
        Self {
            id: Packets::OsuLogout as i16,
        }
    }
}

pub struct OsuRequestStatusUpdate {
    id: i16,
}

impl ClientPacket for OsuRequestStatusUpdate {}
impl OsuRequestStatusUpdate {
    pub fn new() -> Self {
        Self {
            id: Packets::OsuRequestStatusUpdate as i16,
        }
    }
}

pub struct OsuPing {
    id: i16,
}

impl ClientPacket for OsuPing {}
impl OsuPing {
    pub fn new() -> Self {
        Self {
            id: Packets::OsuPing as i16,
        }
    }
}

pub struct OsuStartSpectating {
    id: i16,
    target_user_id: i32,
}

impl ClientPacket for OsuStartSpectating {}
impl OsuStartSpectating {
    pub fn new(reader: &mut PacketReader) -> Self {
        Self {
            id: Packets::OsuStartSpectating as i16,
            target_user_id: reader.read_i32(),
        }
    }
}

pub struct OsuStopSpectating {
    id: i16,
}

impl ClientPacket for OsuStopSpectating {}
impl OsuStopSpectating {
    pub fn new() -> Self {
        Self {
            id: Packets::OsuStopSpectating as i16,
        }
    }
}

pub struct OsuSpectateFrames {
    id: i16
}

impl ClientPacket for OsuSpectateFrames {}
impl OsuSpectateFrames {
    pub fn new(reader: &mut PacketReader) -> Self {
        Self {
            id: Packets::OsuSpectateFrames as i16,
        }
    }
}

pub struct OsuErrorReport {
    id: i16
}

impl ClientPacket for OsuErrorReport {}
impl OsuErrorReport {
    pub fn new(reader: &mut PacketReader) -> Self {
        Self {
            id: Packets::OsuErrorReport as i16,
        }
    }
}

pub struct OsuCantSpectate {
    id: i16
}

impl ClientPacket for OsuCantSpectate {}
impl OsuCantSpectate {
    pub fn new(reader: &mut PacketReader) -> Self {
        Self {
            id: Packets::OsuCantSpectate as i16,
        }
    }
}

pub struct OsuSendPrivateMessage {
    id: i16,
    message: Message
}

impl ClientPacket for OsuSendPrivateMessage {}
impl OsuSendPrivateMessage {
    pub fn new(reader: &mut PacketReader) -> Self {
        Self {
            id: Packets::OsuSendPrivateMessage as i16,
            message: reader.read_message()
        }
    }
}

pub struct OsuPartLobby {
    id: i16,
}

impl ClientPacket for OsuPartLobby {}
impl OsuPartLobby {
    pub fn new() -> Self {
        Self {
            id: Packets::OsuPartLobby as i16,
        }
    }
}

pub struct OsuJoinLobby {
    id: i16,
}

impl ClientPacket for OsuJoinLobby {}
impl OsuJoinLobby {
    pub fn new() -> Self {
        Self {
            id: Packets::OsuJoinLobby as i16,
        }
    }
}

pub struct OsuCreateMatch {
    id: i16
}

impl ClientPacket for OsuCreateMatch {}
impl OsuCreateMatch {
    pub fn new(reader: &mut PacketReader) -> Self {
        Self {
            id: Packets::OsuCreateMatch as i16,
        }
    }
}

pub struct OsuJoinMatch {
    id: i16
}

impl ClientPacket for OsuJoinMatch {}
impl OsuJoinMatch {
    pub fn new(reader: &mut PacketReader) -> Self {
        Self {
            id: Packets::OsuJoinMatch as i16,
        }
    }
}

pub struct OsuPartMatch {
    id: i16
}

impl ClientPacket for OsuPartMatch {}
impl OsuPartMatch {
    pub fn new() -> Self {
        Self {
            id: Packets::OsuPartMatch as i16,
        }
    }
}

pub struct OsuMatchChangeSlot {
    id: i16
}

impl ClientPacket for OsuMatchChangeSlot {}
impl OsuMatchChangeSlot {
    pub fn new(reader: &mut PacketReader) -> Self {
        Self {
            id: Packets::OsuMatchChangeSlot as i16,
        }
    }
}

pub struct OsuMatchReady {
    id: i16
}

impl ClientPacket for OsuMatchReady {}
impl OsuMatchReady {
    pub fn new() -> Self {
        Self {
            id: Packets::OsuMatchReady as i16,
        }
    }
}

pub struct OsuMatchLock {
    id: i16
}

impl ClientPacket for OsuMatchLock {}
impl OsuMatchLock {
    pub fn new(reader: &mut PacketReader) -> Self {
        Self {
            id: Packets::OsuMatchLock as i16,
        }
    }
}

pub struct OsuMatchChangeSettings {
    id: i16
}

impl ClientPacket for OsuMatchChangeSettings {}
impl OsuMatchChangeSettings {
    pub fn new(reader: &mut PacketReader) -> Self {
        Self {
            id: Packets::OsuMatchChangeSettings as i16,
        }
    }
}

pub struct OsuMatchStart {
    id: i16
}

impl ClientPacket for OsuMatchStart {}
impl OsuMatchStart {
    pub fn new() -> Self {
        Self {
            id: Packets::OsuMatchStart as i16,
        }
    }
}

pub struct OsuMatchScoreUpdate {
    id: i16
}

impl ClientPacket for OsuMatchScoreUpdate {}
impl OsuMatchScoreUpdate {
    pub fn new(reader: &mut PacketReader) -> Self {
        Self {
            id: Packets::OsuMatchScoreUpdate as i16,
        }
    }
}

pub struct OsuMatchComplete {
    id: i16
}

impl ClientPacket for OsuMatchComplete {}
impl OsuMatchComplete {
    pub fn new() -> Self {
        Self {
            id: Packets::OsuMatchComplete as i16,
        }
    }
}

pub struct OsuMatchChangeMods {
    id: i16
}

impl ClientPacket for OsuMatchChangeMods {}
impl OsuMatchChangeMods {
    pub fn new(reader: &mut PacketReader) -> Self {
        Self {
            id: Packets::OsuMatchChangeMods as i16,
        }
    }
}

pub struct OsuMatchLoadComplete {
    id: i16
}

impl ClientPacket for OsuMatchLoadComplete {}
impl OsuMatchLoadComplete {
    pub fn new() -> Self {
        Self {
            id: Packets::OsuMatchLoadComplete as i16,
        }
    }
}

pub struct OsuMatchNoBeatmap {
    id: i16
}

impl ClientPacket for OsuMatchNoBeatmap {}
impl OsuMatchNoBeatmap {
    pub fn new() -> Self {
        Self {
            id: Packets::OsuMatchNoBeatmap as i16,
        }
    }
}

pub struct OsuMatchNotReady {
    id: i16
}

impl ClientPacket for OsuMatchNotReady {}
impl OsuMatchNotReady {
    pub fn new() -> Self {
        Self {
            id: Packets::OsuMatchNotReady as i16,
        }
    }
}

pub struct OsuMatchFailed {
    id: i16
}

impl ClientPacket for OsuMatchFailed {}
impl OsuMatchFailed {
    pub fn new() -> Self {
        Self {
            id: Packets::OsuMatchFailed as i16,
        }
    }
}

pub struct  OsuMatchHasBeatmap {
    id: i16
}

impl ClientPacket for OsuMatchHasBeatmap {}
impl OsuMatchHasBeatmap {
    pub fn new(reader: &mut PacketReader) -> Self {
        Self {
            id: Packets::OsuMatchHasBeatmap as i16,
        }
    }
}

pub struct OsuMatchSkipRequest {
    id: i16
}

impl ClientPacket for OsuMatchSkipRequest {}
impl OsuMatchSkipRequest {
    pub fn new() -> Self {
        Self {
            id: Packets::OsuMatchSkipRequest as i16,
        }
    }
}

pub struct OsuChannelJoin {
    id: i16,
    channel_name: String
}

impl ClientPacket for OsuChannelJoin {}
impl OsuChannelJoin {
    pub fn new(reader: &mut PacketReader) -> Self {
        Self {
            id: Packets::OsuChannelJoin as i16,
            channel_name: reader.read_string()
        }
    }
}

pub struct OsuBeatmapInfoRequest {
    id: i16
}

impl ClientPacket for OsuBeatmapInfoRequest {}
impl OsuBeatmapInfoRequest {
    pub fn new(reader: &mut PacketReader) -> Self {
        Self {
            id: Packets::OsuBeatmapInfoRequest as i16,
        }
    }
}

pub struct OsuMatchTransferHost {
    id: i16
}

impl ClientPacket for OsuMatchTransferHost {}
impl OsuMatchTransferHost {
    pub fn new(reader: &mut PacketReader) -> Self {
        Self {
            id: Packets::OsuMatchTransferHost as i16,
        }
    }
}

pub struct OsuFriendAdd {
    id: i16,
    user_id: i32
}

impl ClientPacket for OsuFriendAdd {}
impl OsuFriendAdd {
    pub fn new(reader: &mut PacketReader) -> Self {
        Self {
            id: Packets::OsuFriendAdd as i16,
            user_id: reader.read_i32()
        }
    }
}

pub struct OsuFriendRemove {
    id: i16,
    user_id: i32
}

impl ClientPacket for OsuFriendRemove {}
impl OsuFriendRemove {
    pub fn new(reader: &mut PacketReader) -> Self {
        Self {
            id: Packets::OsuFriendRemove as i16,
            user_id: reader.read_i32()
        }
    }
}

pub struct OsuMatchChangeTeam {
    id: i16
}

impl ClientPacket for OsuMatchChangeTeam {}
impl OsuMatchChangeTeam {
    pub fn new(reader: &mut PacketReader) -> Self {
        Self {
            id: Packets::OsuMatchChangeTeam as i16,
        }
    }
}

pub struct  OsuChannelPart {
    id: i16,
    channel_name: String
}

impl ClientPacket for OsuChannelPart {}
impl OsuChannelPart {
    pub fn new(reader: &mut PacketReader) -> Self {
        Self {
            id: Packets::OsuChannelPart as i16,
            channel_name: reader.read_string()
        }
    }
}

pub struct OsuReceiveUpdates {
    id: i16,
    value: i32
}

impl ClientPacket for OsuReceiveUpdates {}
impl OsuReceiveUpdates {
    pub fn new(reader: &mut PacketReader) -> Self {
        Self {
            id: Packets::OsuReceiveUpdates as i16,
            value: reader.read_i32()
        }
    }
}

pub struct OsuSetAwayMessage {
    id: i16,
    message: Message
}

impl ClientPacket for OsuSetAwayMessage {}
impl OsuSetAwayMessage {
    pub fn new(reader: &mut PacketReader) -> Self {
        Self {
            id: Packets::OsuSetAwayMessage as i16,
            message: reader.read_message()
        }
    }
}

pub struct OsuIrcOnly {
    id: i16
}

impl ClientPacket for OsuIrcOnly {}
impl OsuIrcOnly {
    pub fn new(reader: &mut PacketReader) -> Self {
        Self {
            id: Packets::OsuIrcOnly as i16,
        }
    }
}

/// i16l
pub struct OsuUserStatsRequest {
    id: i16,
    user_ids: Vec<i32>
}

impl ClientPacket for OsuUserStatsRequest {}
impl OsuUserStatsRequest {
    pub fn new(reader: &mut PacketReader) -> Self {
        Self {
            id: Packets::OsuUserStatsRequest as i16,
            user_ids: reader.read_i32_list_i16l()
        }
    }
}

pub struct OsuMatchInvite {
    id: i16,
}

impl ClientPacket for OsuMatchInvite {}
impl OsuMatchInvite {
    pub fn new(reader: &mut PacketReader) -> Self {
        Self {
            id: Packets::OsuMatchInvite as i16,
        }
    }
}

pub struct OsuMatchChangePassword {
    id: i16,
}

impl ClientPacket for OsuMatchChangePassword {}
impl OsuMatchChangePassword {
    pub fn new(reader: &mut PacketReader) -> Self {
        Self {
            id: Packets::OsuMatchChangePassword as i16,
        }
    }
}

pub struct OsuTournamentMatchInfoRequest {
    id: i16,
}

impl ClientPacket for OsuTournamentMatchInfoRequest {}
impl OsuTournamentMatchInfoRequest {
    pub fn new(reader: &mut PacketReader) -> Self {
        Self {
            id: Packets::OsuTournamentMatchInfoRequest as i16,
        }
    }
}

/// i16l
pub struct OsuUserPresenceRequest {
    id: i16,
    user_ids: Vec<i32>
}

impl ClientPacket for OsuUserPresenceRequest {}
impl OsuUserPresenceRequest {
    pub fn new(reader: &mut PacketReader) -> Self {
        Self {
            id: Packets::OsuUserPresenceRequest as i16,
            user_ids: reader.read_i32_list_i16l()
        }
    }
}

pub struct OsuUserPresenceRequestAll {
    id: i16,
    ingame_time: i32,
}

impl ClientPacket for OsuUserPresenceRequestAll {}
impl OsuUserPresenceRequestAll {
    pub fn new(reader: &mut PacketReader) -> Self {
        Self {
            id: Packets::OsuUserPresenceRequestAll as i16,
            ingame_time: reader.read_i32()
        }
    }
}

pub struct OsuToggleBlockNonFriendDms {
    id: i16,
    value: i32
}

impl ClientPacket for OsuToggleBlockNonFriendDms {}
impl OsuToggleBlockNonFriendDms {
    pub fn new(reader: &mut PacketReader) -> Self {
        Self {
            id: Packets::OsuToggleBlockNonFriendDms as i16,
            value: reader.read_i32()
        }
    }
}

pub struct  OsuTournamentJoinMatchChannel {
    id: i16,
}

impl ClientPacket for OsuTournamentJoinMatchChannel {}
impl OsuTournamentJoinMatchChannel {
    pub fn new(reader: &mut PacketReader) -> Self {
        Self {
            id: Packets::OsuTournamentJoinMatchChannel as i16,
        }
    }
}

pub struct  OsuTournamentLeaveMatchChannel {
    id: i16,
}

impl ClientPacket for OsuTournamentLeaveMatchChannel {}
impl OsuTournamentLeaveMatchChannel {
    pub fn new(reader: &mut PacketReader) -> Self {
        Self {
            id: Packets::OsuTournamentLeaveMatchChannel as i16,
        }
    }
}

pub type HandlerHashMap = HashMap<
    Packets,
    for<'lt> fn(
        player: &'lt mut Player,
        reader: &'lt mut PacketReader,
    ) -> BoxFuture<'lt, Result<Vec<u8>, Box<dyn std::error::Error>>>,
>;

macro_rules! register_packets {(
    $(
        #[packet($id:path, $res:expr $(,)?)]
     $( #[$attr:meta] )*
        $pub:vis
        async
        fn $fname:ident ($player:ident : & $('_)? mut Player, $reader:ident : & $('_)? mut PacketReader) -> Result<Vec<u8>, Box<dyn std::error::Error>>
        $body:block
    )*
) => (
    $(
     $( #[$attr] )*
        $pub
        fn $fname<'lt> (
            $player : &'lt mut Player,
            $reader : &'lt mut PacketReader,
        ) -> BoxFuture<'lt, Result<Vec<u8>, Box<dyn std::error::Error>>>
        {
            return FutureExt::boxed(async move {
                let _ = (&$player, &$reader);
                $body
            })
        }
    )*

    lazy_static::lazy_static! {
        pub static ref PACKET_HANDLERS: HandlerHashMap = {
            let mut map = HashMap::new();
            $( map.insert($id, $fname as _); )*
            map
        };

        pub static ref RESTRICTED_PACKET_HANDLERS: HandlerHashMap = {
            let mut map = HashMap::new();
            $(
                if $res {
                    map.insert($id, $fname as _);
                }
            )*
            map
        };
    }
)}

register_packets!(

    #[packet(Packets::OsuChangeAction, false)]
    pub async fn osu_change_action(player: &mut Player, reader: &mut PacketReader) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let packet = OsuChangeAction::new(reader);

        Ok(Vec::new())
    }
);

// -===========-
// -= Server Packets =-
// -===========-


