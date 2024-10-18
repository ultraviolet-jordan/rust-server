use std::cell::{Ref, RefCell, RefMut};
use std::thread::sleep;
use std::time::{Duration, Instant};

use cache::{
    CacheProvider, ObjType, ScriptEngine, ScriptFile, ScriptOpcode, ScriptPlayer, ScriptRunner,
    ScriptState,
};

use crate::entity::player::Player;
use crate::script::script::Ops;

#[repr(u8)]
pub enum EngineStat {
    Cycle,
    World,
    ClientsIn,
    Npcs,
    Players,
    Logouts,
    Logins,
    Zones,
    ClientsOut,
    Cleanup,
    BandwidthIn,
    BandwidthOut,
}

pub struct EngineTick {
    pub current_tick: i32,
}

impl EngineTick {
    pub fn new() -> EngineTick {
        return EngineTick { current_tick: 0 };
    }

    pub fn increment(&mut self) {
        self.current_tick += 1;
    }
}

pub struct Engine {
    pub tick: EngineTick,
    pub tick_rate: Duration,
    pub cache: CacheProvider,
    pub ops: Ops,
    pub players: Vec<Option<RefCell<Player>>>,
    pub stats: Vec<Duration>,
    pub last_stats: Vec<Duration>,
}

impl Engine {
    pub fn new(cache: CacheProvider) -> Engine {
        return Engine {
            tick: EngineTick::new(),
            tick_rate: Duration::from_millis(600),
            cache,
            ops: Ops::new(),
            players: vec![None; 2048],
            stats: vec![Duration::new(0, 0); 12],
            last_stats: vec![Duration::new(0, 0); 12],
        };
    }

    pub fn mock() -> Engine {
        return Engine {
            tick: EngineTick::new(),
            tick_rate: Duration::from_millis(600),
            cache: CacheProvider::mock(),
            ops: Ops::new(),
            players: vec![None; 2048],
            stats: vec![Duration::new(0, 0); 12],
            last_stats: vec![Duration::new(0, 0); 12],
        };
    }

    pub fn start(&mut self, start_cycle: bool) {
        println!("Starting world...");
        // TODO load maps
        println!("World ready!");

        if start_cycle {
            self.cycle();
        }
    }

    #[rustfmt::skip]
    fn cycle(&mut self) {
        loop {
            let start: Instant = Instant::now();

            // world processing
            // - world queue
            // - calculate afk event readiness
            // - npc spawn scripts
            // - npc hunt
            self.process_world();

            // client input
            // - decode packets
            // - process pathfinding/following
            self.process_in();

            // npc processing (if npc is not busy)
            // - resume suspended script
            // - stat regen
            // - timer
            // - queue
            // - movement
            // - modes
            self.process_npcs();

            // player processing
            // - resume suspended script
            // - primary queue
            // - weak queue
            // - timers
            // - soft timers
            // - engine queue
            // - interactions
            // - movement
            // - close interface if attempting to logout
            self.process_players();

            // player logout
            self.process_logouts();

            // player login, good spot for it (before packets so they immediately load but after processing so nothing hits them)
            self.process_logins();

            // process zones
            // - build list of active zones around players
            // - loc/obj despawn/respawn
            // - compute shared buffer
            self.process_zones();

            // process movement directions
            // - convert player movements
            // - convert npc movements
            self.process_movement_dirs();

            // client output
            // - map update
            // - player info
            // - npc info
            // - zone updates
            // - inv changes
            // - stat changes
            // - afk zones changes
            // - flush packets
            self.process_out();

            // cleanup
            // - reset zones
            // - reset players
            // - reset npcs
            // - reset invs
            self.process_cleanup();

            // cycle the world now
            self.tick.increment();
            self.stats[EngineStat::Cycle as usize] = Instant::now() - start;

            // update stats
            self.last_stats[EngineStat::Cycle as usize] = self.stats[EngineStat::Cycle as usize];
            self.last_stats[EngineStat::World as usize] = self.stats[EngineStat::World as usize];
            self.last_stats[EngineStat::ClientsIn as usize] = self.stats[EngineStat::ClientsIn as usize];
            self.last_stats[EngineStat::Npcs as usize] = self.stats[EngineStat::Npcs as usize];
            self.last_stats[EngineStat::Players as usize] = self.stats[EngineStat::Players as usize];
            self.last_stats[EngineStat::Logouts as usize] = self.stats[EngineStat::Logouts as usize];
            self.last_stats[EngineStat::Logins as usize] = self.stats[EngineStat::Logins as usize];
            self.last_stats[EngineStat::Zones as usize] = self.stats[EngineStat::Zones as usize];
            self.last_stats[EngineStat::ClientsOut as usize] = self.stats[EngineStat::ClientsOut as usize];
            self.last_stats[EngineStat::Cleanup as usize] = self.stats[EngineStat::Cleanup as usize];
            self.last_stats[EngineStat::BandwidthIn as usize] = self.stats[EngineStat::BandwidthIn as usize];
            self.last_stats[EngineStat::BandwidthOut as usize] = self.stats[EngineStat::BandwidthOut as usize];

            println!(
                "tick {} took {:?}",
                self.tick.current_tick,
                self.stats[EngineStat::Cycle as usize]
            );
            println!("----");
            
            sleep(self.tick_rate.saturating_sub(Instant::now() - start));
        }
    }

    // - world queue
    // - calculate afk event readiness
    // - npc spawn scripts
    // - npc hunt
    fn process_world(&mut self) {
        let start: Instant = Instant::now();
        // TODO
        self.stats[EngineStat::World as usize] = Instant::now() - start
    }

    // - decode packets
    // - process pathfinding/following
    fn process_in(&mut self) {
        let start: Instant = Instant::now();
        // - decode packets
        for player in &self.players {
            if let Some(ref player) = player {
                let _: Ref<Player> = player.borrow();
                // TODO
            }
        }
        // - process pathfinding/following
        for player in &self.players {
            if let Some(ref player) = player {
                let _: Ref<Player> = player.borrow();
                // TODO
            }
        }
        self.stats[EngineStat::ClientsIn as usize] = Instant::now() - start
    }

    // - resume suspended script
    // - stat regen
    // - timer
    // - queue
    // - movement
    // - modes
    fn process_npcs(&mut self) {
        let start: Instant = Instant::now();
        // TODO
        self.stats[EngineStat::Npcs as usize] = Instant::now() - start
    }

    // - resume suspended script
    // - primary queue
    // - weak queue
    // - timers
    // - soft timers
    // - engine queue
    // - interactions
    // - movement
    // - close interface if attempting to logout
    fn process_players(&mut self) {
        let start: Instant = Instant::now();
        for player in &self.players {
            if let Some(ref player) = player {
                let _: Ref<Player> = player.borrow();
                // TODO
            }
        }
        self.stats[EngineStat::Players as usize] = Instant::now() - start
    }

    fn process_logouts(&mut self) {
        let start: Instant = Instant::now();
        for player in &self.players {
            if let Some(ref player) = player {
                let _: Ref<Player> = player.borrow();
                // TODO
            }
        }
        self.stats[EngineStat::Logouts as usize] = Instant::now() - start
    }

    fn process_logins(&mut self) {
        let start: Instant = Instant::now();
        // TODO
        self.stats[EngineStat::Logins as usize] = Instant::now() - start
    }

    // - build list of active zones around players
    // - loc/obj despawn/respawn
    // - compute shared buffer
    fn process_zones(&mut self) {
        let start: Instant = Instant::now();
        // TODO
        self.stats[EngineStat::Zones as usize] = Instant::now() - start
    }

    // - convert player movements
    // - convert npc movements
    fn process_movement_dirs(&self) {
        // TODO: benchmark this?
        for player in &self.players {
            if let Some(ref player) = player {
                let _: Ref<Player> = player.borrow();
                // TODO
            }
        }
        // TODO
    }

    // - map update
    // - player info
    // - npc info
    // - zone updates
    // - inv changes
    // - stat changes
    // - afk zones changes
    // - flush packets
    fn process_out(&mut self) {
        let start: Instant = Instant::now();
        for player in &self.players {
            if let Some(ref player) = player {
                let _: Ref<Player> = player.borrow();
                // TODO
            }
        }
        self.stats[EngineStat::ClientsOut as usize] = Instant::now() - start
    }

    // - reset zones
    // - reset players
    // - reset npcs
    // - reset invs
    fn process_cleanup(&mut self) {
        let start: Instant = Instant::now();
        // - reset zones
        // - reset players
        for player in &self.players {
            if let Some(ref player) = player {
                let _: Ref<Player> = player.borrow();
                // TODO
            }
        }
        // - reset npcs
        // - reset invs
        self.stats[EngineStat::Cleanup as usize] = Instant::now() - start
    }

    pub fn add_player(&mut self, uid: i32, player: Player) {
        if let Some(slot) = self.players.get_mut(uid as usize) {
            *slot = Some(RefCell::new(player));
        }
    }

    pub fn get_player(&self, uid: i32) -> Result<Ref<Player>, String> {
        match self.players.get(uid as usize) {
            None => Err(format!("Player with uid {} not found in engine", uid)),
            Some(None) => Err(format!("Player with uid {} not initialized", uid)),
            Some(Some(player)) => Ok(player.borrow()),
        }
    }
}

/// It is important to note that these are not commands.
/// This is specifically for interfacing commands<->engine.
impl ScriptEngine for Engine {
    fn pop_obj(&self, id: i32) -> Result<&ObjType, String> {
        return self.cache.obj_provider.get_by_id(id as usize);
    }

    fn pop_script(&self, id: i32) -> Result<&ScriptFile, String> {
        return self.cache.script_provider.get_by_id(id as usize);
    }

    fn line_of_sight(&self, from: i32, to: i32) -> bool {
        // TODO: rsmod stuff.
        return false;
    }

    fn add_obj(&self, coord: i32, id: i32, count: i32, duration: i32) -> bool {
        // TODO: zone stuff.
        return false;
    }

    fn with_player_mut<F>(&self, uid: i32, on_found: F) -> Result<(), String>
    where
        F: FnOnce(RefMut<dyn ScriptPlayer>),
    {
        match self.players.get(uid as usize) {
            Some(Some(ref player)) => {
                on_found(player.borrow_mut()); // Call the closure on the found player
                Ok(())
            }
            _ => Err(format!(
                "Player with uid {} not found or not initialized",
                uid
            )),
        }
    }

    fn with_player<F>(&self, uid: i32, on_found: F) -> Result<(), String>
    where
        F: FnOnce(Ref<dyn ScriptPlayer>),
    {
        match self.players.get(uid as usize) {
            Some(Some(ref player)) => {
                on_found(player.borrow()); // Call the closure on the found player
                Ok(())
            }
            _ => Err(format!(
                "Player with uid {} not found or not initialized",
                uid
            )),
        }
    }
}

impl ScriptRunner for Engine {
    fn push_script<'script>(
        &'script self,
        state: &mut ScriptState<'script>,
        code: &ScriptOpcode,
    ) -> Result<(), String> {
        // println!("{:?}", code);
        match code {
            // Core language ops (0-99)
            ScriptOpcode::PushConstantInt
            | ScriptOpcode::PushVarp
            | ScriptOpcode::PopVarp
            | ScriptOpcode::PushConstantString
            | ScriptOpcode::PushVarn
            | ScriptOpcode::PopVarn
            | ScriptOpcode::Branch
            | ScriptOpcode::BranchNot
            | ScriptOpcode::BranchEquals
            | ScriptOpcode::BranchLessThan
            | ScriptOpcode::BranchGreaterThan
            | ScriptOpcode::PushVars
            | ScriptOpcode::PopVars
            | ScriptOpcode::Return
            | ScriptOpcode::GoSub
            | ScriptOpcode::Jump
            | ScriptOpcode::Switch
            | ScriptOpcode::PushVarbit
            | ScriptOpcode::PopVarbit
            | ScriptOpcode::BranchLessThanOrEquals
            | ScriptOpcode::BranchGreaterThanOrEquals
            | ScriptOpcode::PushIntLocal
            | ScriptOpcode::PopIntLocal
            | ScriptOpcode::PushStringLocal
            | ScriptOpcode::PopStringLocal
            | ScriptOpcode::JoinString
            | ScriptOpcode::PopIntDiscard
            | ScriptOpcode::PopStringDiscard
            | ScriptOpcode::GoSubWithParams
            | ScriptOpcode::JumpWithParams
            | ScriptOpcode::PushVarcInt
            | ScriptOpcode::PopVarcInt
            | ScriptOpcode::DefineArray
            | ScriptOpcode::PushArrayInt
            | ScriptOpcode::PopArrayInt
            | ScriptOpcode::EndCoreOps => self.ops.core.push(self, state, code),
            // Server ops (1000-1999)
            ScriptOpcode::CoordX
            | ScriptOpcode::CoordY
            | ScriptOpcode::CoordZ
            | ScriptOpcode::Distance
            | ScriptOpcode::HuntAll
            | ScriptOpcode::HuntNext
            | ScriptOpcode::InZone
            | ScriptOpcode::LineOfSight
            | ScriptOpcode::LineOfWalk
            | ScriptOpcode::MapBlocked
            | ScriptOpcode::MapIndoors
            | ScriptOpcode::MapClock
            | ScriptOpcode::MapLocAddUnsafe
            | ScriptOpcode::MapMembers
            | ScriptOpcode::MapPlayerCount
            | ScriptOpcode::MapFindSquare
            | ScriptOpcode::MoveCoord
            | ScriptOpcode::PlayerCount
            | ScriptOpcode::ProjAnimMap
            | ScriptOpcode::ProjAnimNpc
            | ScriptOpcode::ProjAnimPl
            | ScriptOpcode::SeqLength
            | ScriptOpcode::SplitGet
            | ScriptOpcode::SplitGetAnim
            | ScriptOpcode::SplitInit
            | ScriptOpcode::SplitLineCount
            | ScriptOpcode::SplitPageCount
            | ScriptOpcode::SpotAnimMap
            | ScriptOpcode::StatRandom
            | ScriptOpcode::StructParam
            | ScriptOpcode::WorldDelay
            | ScriptOpcode::NpcsCount
            | ScriptOpcode::ZonesCount
            | ScriptOpcode::LocsCount
            | ScriptOpcode::ObjsCount
            | ScriptOpcode::MapMulti => Err("Not implemented".to_string()),
            // Player ops (2000-2499)
            ScriptOpcode::AllowDesign
            | ScriptOpcode::Anim
            | ScriptOpcode::BasReadyAnim
            | ScriptOpcode::BasRunning
            | ScriptOpcode::BasTurnOnSpot
            | ScriptOpcode::BasWalkB
            | ScriptOpcode::BasWalkF
            | ScriptOpcode::BasWalkL
            | ScriptOpcode::BasWalkR
            | ScriptOpcode::BufferFull
            | ScriptOpcode::BuildAppearance
            | ScriptOpcode::Busy
            | ScriptOpcode::CamLookAt
            | ScriptOpcode::CamMoveTo
            | ScriptOpcode::CamReset
            | ScriptOpcode::CamShake
            | ScriptOpcode::ClearQueue
            | ScriptOpcode::ClearSoftTimer
            | ScriptOpcode::ClearTimer
            | ScriptOpcode::GetTimer
            | ScriptOpcode::Coord
            | ScriptOpcode::Damage
            | ScriptOpcode::Displayname
            | ScriptOpcode::FaceSquare
            | ScriptOpcode::FindUid
            | ScriptOpcode::Gender
            | ScriptOpcode::GetQueue
            | ScriptOpcode::StatAdvance
            | ScriptOpcode::HeadiconsGet
            | ScriptOpcode::HeadiconsSet
            | ScriptOpcode::HealEnergy
            | ScriptOpcode::HintCoord
            | ScriptOpcode::HintNpc
            | ScriptOpcode::HintPlayer
            | ScriptOpcode::HintStop
            | ScriptOpcode::IfClose
            | ScriptOpcode::TutClose
            | ScriptOpcode::IfMultiZone
            | ScriptOpcode::IfOpenChat
            | ScriptOpcode::TutOpen
            | ScriptOpcode::IfOpenMain
            | ScriptOpcode::IfOpenMainSide
            | ScriptOpcode::IfOpenSide
            | ScriptOpcode::IfSetAnim
            | ScriptOpcode::IfSetColour
            | ScriptOpcode::IfSetHide
            | ScriptOpcode::IfSetModel
            | ScriptOpcode::IfSetRecol
            | ScriptOpcode::IfSetNpcHead
            | ScriptOpcode::IfSetObject
            | ScriptOpcode::IfSetPlayerHead
            | ScriptOpcode::IfSetPosition
            | ScriptOpcode::IfSetResumeButtons
            | ScriptOpcode::IfSetTab
            | ScriptOpcode::IfSetTabActive
            | ScriptOpcode::TutFlash
            | ScriptOpcode::IfSetText
            | ScriptOpcode::LastLoginInfo
            | ScriptOpcode::LastCom
            | ScriptOpcode::LastInt
            | ScriptOpcode::LastItem
            | ScriptOpcode::LastSlot
            | ScriptOpcode::LastTargetSlot
            | ScriptOpcode::LastUseItem
            | ScriptOpcode::LastUseSlot
            | ScriptOpcode::LongQueue
            | ScriptOpcode::Mes
            | ScriptOpcode::MidiJingle
            | ScriptOpcode::MidiSong
            | ScriptOpcode::Name
            | ScriptOpcode::PApRange
            | ScriptOpcode::PArriveDelay
            | ScriptOpcode::PCountDialog
            | ScriptOpcode::PDelay
            | ScriptOpcode::PExactMove
            | ScriptOpcode::PFindUid
            | ScriptOpcode::PLocMerge
            | ScriptOpcode::PLogout
            | ScriptOpcode::POpHeld
            | ScriptOpcode::POpLoc
            | ScriptOpcode::POpNpc
            | ScriptOpcode::POpNpcT
            | ScriptOpcode::POpObj
            | ScriptOpcode::POpPlayer
            | ScriptOpcode::POpPlayerT
            | ScriptOpcode::PPauseButton
            | ScriptOpcode::PStopAction
            | ScriptOpcode::PTeleJump
            | ScriptOpcode::PTeleport
            | ScriptOpcode::PWalk
            | ScriptOpcode::PlayerFindAllZone
            | ScriptOpcode::PlayerFindNext
            | ScriptOpcode::Queue
            | ScriptOpcode::Say
            | ScriptOpcode::WalkTrigger
            | ScriptOpcode::SetTimer
            | ScriptOpcode::SoftTimer
            | ScriptOpcode::SoundSynth
            | ScriptOpcode::SpotAnimPl
            | ScriptOpcode::StaffModLevel
            | ScriptOpcode::Stat
            | ScriptOpcode::StatAdd
            | ScriptOpcode::StatBase
            | ScriptOpcode::StatHeal
            | ScriptOpcode::StatSub
            | ScriptOpcode::StrongQueue
            | ScriptOpcode::Uid
            | ScriptOpcode::WeakQueue
            | ScriptOpcode::IfOpenMainOverlay
            | ScriptOpcode::AfkEvent
            | ScriptOpcode::LowMemory
            | ScriptOpcode::SetIdkit
            | ScriptOpcode::PClearPendingAction
            | ScriptOpcode::GetWalkTrigger
            | ScriptOpcode::Busy2
            | ScriptOpcode::FindHero
            | ScriptOpcode::BothHeroPoints
            | ScriptOpcode::SetGender
            | ScriptOpcode::SetSkinColour
            | ScriptOpcode::PAnimProtect
            | ScriptOpcode::RunEnergy
            | ScriptOpcode::Weight
            | ScriptOpcode::LastCoord => self.ops.player.push(self, state, code),
            // Npc ops (2500-2999)
            ScriptOpcode::NpcAdd
            | ScriptOpcode::NpcAnim
            | ScriptOpcode::NpcBaseStat
            | ScriptOpcode::NpcCategory
            | ScriptOpcode::NpcChangeType
            | ScriptOpcode::NpcCoord
            | ScriptOpcode::NpcDamage
            | ScriptOpcode::NpcDel
            | ScriptOpcode::NpcDelay
            | ScriptOpcode::NpcFaceSquare
            | ScriptOpcode::NpcFind
            | ScriptOpcode::NpcFindAllAny
            | ScriptOpcode::NpcFindAll
            | ScriptOpcode::NpcFindExact
            | ScriptOpcode::NpcFindHero
            | ScriptOpcode::NpcFindAllZone
            | ScriptOpcode::NpcFindNext
            | ScriptOpcode::NpcFindUid
            | ScriptOpcode::NpcGetMode
            | ScriptOpcode::NpcHeroPoints
            | ScriptOpcode::NpcName
            | ScriptOpcode::NpcParam
            | ScriptOpcode::NpcQueue
            | ScriptOpcode::NpcRange
            | ScriptOpcode::NpcSay
            | ScriptOpcode::NpcHuntAll
            | ScriptOpcode::NpcHuntNext
            | ScriptOpcode::NpcSetHunt
            | ScriptOpcode::NpcSetHuntMode
            | ScriptOpcode::NpcSetMode
            | ScriptOpcode::NpcWalkTrigger
            | ScriptOpcode::NpcSetTimer
            | ScriptOpcode::NpcStat
            | ScriptOpcode::NpcStatAdd
            | ScriptOpcode::NpcStatHeal
            | ScriptOpcode::NpcStatSub
            | ScriptOpcode::NpcTele
            | ScriptOpcode::NpcType
            | ScriptOpcode::NpcUid
            | ScriptOpcode::SpotAnimNpc
            | ScriptOpcode::NpcWalk
            | ScriptOpcode::NpcAttackRange
            | ScriptOpcode::NpcHasOp
            | ScriptOpcode::NpcArriveDelay => Err("Not implemented".to_string()),
            // Loc ops (3000-3499)
            ScriptOpcode::LocAdd
            | ScriptOpcode::LocAngle
            | ScriptOpcode::LocAnim
            | ScriptOpcode::LocCategory
            | ScriptOpcode::LocChange
            | ScriptOpcode::LocCoord
            | ScriptOpcode::LocDel
            | ScriptOpcode::LocFind
            | ScriptOpcode::LocFindAllZone
            | ScriptOpcode::LocFindNext
            | ScriptOpcode::LocName
            | ScriptOpcode::LocParam
            | ScriptOpcode::LocShape
            | ScriptOpcode::LocType => Err("Not implemented".to_string()),
            // Obj ops (3500-4000)
            ScriptOpcode::ObjAdd
            | ScriptOpcode::ObjAddAll
            | ScriptOpcode::ObjCoord
            | ScriptOpcode::ObjCount
            | ScriptOpcode::ObjDel
            | ScriptOpcode::ObjName
            | ScriptOpcode::ObjParam
            | ScriptOpcode::ObjTakeItem
            | ScriptOpcode::ObjType
            | ScriptOpcode::ObjFind => Err("Not implemented".to_string()),
            // Npc config ops (4000-4099)
            ScriptOpcode::NcCategory
            | ScriptOpcode::NcDebugname
            | ScriptOpcode::NcDesc
            | ScriptOpcode::NcName
            | ScriptOpcode::NcOp
            | ScriptOpcode::NcParam => Err("Not implemented".to_string()),
            // Loc config ops (4100-4199)
            ScriptOpcode::LcCategory
            | ScriptOpcode::LcDebugname
            | ScriptOpcode::LcDesc
            | ScriptOpcode::LcName
            | ScriptOpcode::LcOp
            | ScriptOpcode::LcParam
            | ScriptOpcode::LcWidth
            | ScriptOpcode::LcLength => Err("Not implemented".to_string()),
            // Obj config ops (4200-4299)
            ScriptOpcode::OcCategory
            | ScriptOpcode::OcCert
            | ScriptOpcode::OcCost
            | ScriptOpcode::OcDebugname
            | ScriptOpcode::OcDesc
            | ScriptOpcode::OcIop
            | ScriptOpcode::OcMembers
            | ScriptOpcode::OcName
            | ScriptOpcode::OcOp
            | ScriptOpcode::OcParam
            | ScriptOpcode::OcStackable
            | ScriptOpcode::OcTradeable
            | ScriptOpcode::OcUncert
            | ScriptOpcode::OcWearPos2
            | ScriptOpcode::OcWearPos3
            | ScriptOpcode::OcWearPos
            | ScriptOpcode::OcWeight => self.ops.oc.push(self, state, code),
            // Inventory ops (4300-4399)
            ScriptOpcode::InvAllStock
            | ScriptOpcode::InvSize
            | ScriptOpcode::InvStockBase
            | ScriptOpcode::InvAdd
            | ScriptOpcode::InvChangeSlot
            | ScriptOpcode::InvClear
            | ScriptOpcode::InvDel
            | ScriptOpcode::InvDelSlot
            | ScriptOpcode::InvDropItem
            | ScriptOpcode::InvDropSlot
            | ScriptOpcode::InvFreespace
            | ScriptOpcode::InvGetNum
            | ScriptOpcode::InvGetObj
            | ScriptOpcode::InvItemSpace
            | ScriptOpcode::InvItemSpace2
            | ScriptOpcode::InvMoveFromSlot
            | ScriptOpcode::InvMoveToSlot
            | ScriptOpcode::BothMoveInv
            | ScriptOpcode::InvMoveItem
            | ScriptOpcode::InvMoveItemCert
            | ScriptOpcode::InvMoveItemUncert
            | ScriptOpcode::InvSetSlot
            | ScriptOpcode::InvTotal
            | ScriptOpcode::InvTotalCat
            | ScriptOpcode::InvTransmit
            | ScriptOpcode::InvOtherTransmit
            | ScriptOpcode::InvStopTransmit
            | ScriptOpcode::BothDropSlot
            | ScriptOpcode::InvDropAll
            | ScriptOpcode::InvTotalParam
            | ScriptOpcode::InvTotalParamStack => Err("Not implemented".to_string()),
            // Enum ops (4400-4499)
            ScriptOpcode::Enum | ScriptOpcode::EnumGetOutputCount => {
                Err("Not implemented".to_string())
            }
            // String ops (4500-4599)
            ScriptOpcode::AppendNum
            | ScriptOpcode::Append
            | ScriptOpcode::AppendSignNum
            | ScriptOpcode::Lowercase
            | ScriptOpcode::TextGender
            | ScriptOpcode::ToString
            | ScriptOpcode::Compare
            | ScriptOpcode::TextSwitch
            | ScriptOpcode::AppendChar
            | ScriptOpcode::StringLength
            | ScriptOpcode::SubString
            | ScriptOpcode::StringIndexOfChar
            | ScriptOpcode::StringIndexOfString => self.ops.string.push(self, state, code),
            // Number ops (4600-4699)
            ScriptOpcode::Add
            | ScriptOpcode::Sub
            | ScriptOpcode::Multiply
            | ScriptOpcode::Divide
            | ScriptOpcode::Random
            | ScriptOpcode::RandomInc
            | ScriptOpcode::Interpolate
            | ScriptOpcode::AddPercent
            | ScriptOpcode::SetBit
            | ScriptOpcode::ClearBit
            | ScriptOpcode::TestBit
            | ScriptOpcode::Modulo
            | ScriptOpcode::Pow
            | ScriptOpcode::InvPow
            | ScriptOpcode::And
            | ScriptOpcode::Or
            | ScriptOpcode::Min
            | ScriptOpcode::Max
            | ScriptOpcode::Scale
            | ScriptOpcode::BitCount
            | ScriptOpcode::ToggleBit
            | ScriptOpcode::SetBitRange
            | ScriptOpcode::ClearBitRange
            | ScriptOpcode::GetBitRange
            | ScriptOpcode::SetBitRangeToInt
            | ScriptOpcode::SinDeg
            | ScriptOpcode::CosDeg
            | ScriptOpcode::Atan2Deg
            | ScriptOpcode::Abs => self.ops.math.push(state, code),
            // DB ops (7500-7599)
            ScriptOpcode::DbFindWithCount
            | ScriptOpcode::DbFindNext
            | ScriptOpcode::DbGetField
            | ScriptOpcode::DbGetFieldCount
            | ScriptOpcode::DbListAllWithCount
            | ScriptOpcode::DbGetRowTable
            | ScriptOpcode::DbFindByIndex
            | ScriptOpcode::DbFindRefineWithCount
            | ScriptOpcode::DbFind
            | ScriptOpcode::DbFindRefine
            | ScriptOpcode::DbListAll => Err("Not implemented".to_string()),
            // Debug ops (10000-11000)
            ScriptOpcode::Error
            | ScriptOpcode::MapProduction
            | ScriptOpcode::MapLastClock
            | ScriptOpcode::MapLastWorld
            | ScriptOpcode::MapLastClientIn
            | ScriptOpcode::MapLastNpc
            | ScriptOpcode::MapLastPlayer
            | ScriptOpcode::MapLastLogout
            | ScriptOpcode::MapLastLogin
            | ScriptOpcode::MapLastZone
            | ScriptOpcode::MapLastClientOut
            | ScriptOpcode::MapLastCleanup
            | ScriptOpcode::MapLastBandwidthIn
            | ScriptOpcode::MapLastBandwidthOut => Err("Not implemented".to_string()),
        }
    }
}
