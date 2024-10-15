use std::collections::HashMap;
use std::time::Instant;

use io::Packet;

use crate::ObjType;

#[derive(PartialEq, PartialOrd, Clone, Debug)]
#[repr(u16)]
pub enum ScriptOpcode {
    // Core language ops (0-99)
    PushConstantInt = 0,    // official, see cs2
    PushVarp = 1,           // official, see cs2
    PopVarp = 2,            // official, see cs2
    PushConstantString = 3, // official, see cs2
    PushVarn = 4,
    PopVarn = 5,
    Branch = 6,             // official, see cs2
    BranchNot = 7,          // official, see cs2
    BranchEquals = 8,       // official, see cs2
    BranchLessThan = 9,     // official, see cs2
    BranchGreaterThan = 10, // official, see cs2
    PushVars = 11,
    PopVars = 12,
    Return = 21, // official, see cs2
    GoSub = 22,
    Jump = 23,
    Switch = 24,
    PushVarbit = 25,
    PopVarbit = 26,
    BranchLessThanOrEquals = 31,    // official, see cs2
    BranchGreaterThanOrEquals = 32, // official, see cs2
    PushIntLocal = 33,              // official, see cs2
    PopIntLocal = 34,               // official, see cs2
    PushStringLocal = 35,           // official, see cs2
    PopStringLocal = 36,            // official, see cs2
    JoinString = 37,                // official, see cs2
    PopIntDiscard = 38,             // official, see cs2
    PopStringDiscard = 39,          // official, see cs2
    GoSubWithParams = 40,           // official, see cs2
    JumpWithParams = 41,            // official, see cs2
    PushVarcInt = 42,
    PopVarcInt = 43,
    DefineArray = 44,  // official, see cs2
    PushArrayInt = 45, // official, see cs2
    PopArrayInt = 46,  // official, see cs2
    EndCoreOps = 100,
    // Server ops (1000-1999)
    CoordX = 1000, // official, see cs2
    CoordY = 1001, // official, see cs2
    CoordZ = 1002, // official, see cs2
    Distance = 1003,
    HuntAll = 1004,
    HuntNext = 1005, // official
    InZone = 1006,   // official
    LineOfSight = 1007,
    LineOfWalk = 1008,
    MapBlocked = 1009, // official
    MapIndoors = 1010,
    MapClock = 1011,        // official
    MapLocAddUnsafe = 1012, // official
    MapMembers = 1013,      // official
    MapPlayerCount = 1014,  // official, see giant dwarf cutscene
    MapFindSquare = 1015,   // official
    MoveCoord = 1016,       // official
    PlayerCount = 1017,
    ProjAnimMap = 1018,
    ProjAnimNpc = 1019,
    ProjAnimPl = 1020,
    SeqLength = 1021, // official
    SplitGet = 1022,
    SplitGetAnim = 1023,
    SplitInit = 1024, // official
    SplitLineCount = 1025,
    SplitPageCount = 1026, // official
    SpotAnimMap = 1027,
    StatRandom = 1028,
    StructParam = 1029,
    WorldDelay = 1030, // official
    NpcsCount = 1031,
    ZonesCount = 1032,
    LocsCount = 1033,
    ObjsCount = 1034,
    MapMulti = 1035,
    // Player ops (2000-2499)
    AllowDesign = 2000,
    Anim = 2001,
    BasReadyAnim = 2002,
    BasRunning = 2003,
    BasTurnOnSpot = 2004,
    BasWalkB = 2005,
    BasWalkF = 2006,
    BasWalkL = 2007,
    BasWalkR = 2008,
    BufferFull = 2009,      // official
    BuildAppearance = 2010, // official
    Busy = 2011,            // official
    CamLookAt = 2012,       // official
    CamMoveTo = 2013,       // official
    CamReset = 2014,        // official
    CamShake = 2015,        // official, see server packets
    ClearQueue = 2016,      // official
    ClearSoftTimer = 2017,
    ClearTimer = 2018,
    GetTimer = 2019,
    Coord = 2020, // official
    Damage = 2021,
    Displayname = 2022, // official, joke reply
    FaceSquare = 2023,  // official
    FindUid = 2024,     // official
    Gender = 2025,
    GetQueue = 2026, // official
    StatAdvance = 2027,
    HeadiconsGet = 2028,
    HeadiconsSet = 2029,
    HealEnergy = 2030, // official
    HintCoord = 2031,
    HintNpc = 2032,
    HintPlayer = 2033,
    HintStop = 2034,
    IfClose = 2035, // official
    TutClose = 2036,
    IfMultiZone = 2037,
    IfOpenChat = 2038,
    TutOpen = 2039,
    IfOpenMain = 2040,
    IfOpenMainSide = 2041,
    IfOpenSide = 2042,
    IfSetAnim = 2043,   // official
    IfSetColour = 2044, // official
    IfSetHide = 2045,   // official
    IfSetModel = 2046,  // official
    IfSetRecol = 2047,
    IfSetNpcHead = 2048,    // official
    IfSetObject = 2049,     // official
    IfSetPlayerHead = 2050, // official
    IfSetPosition = 2051,   // official
    IfSetResumeButtons = 2052,
    IfSetTab = 2053,
    IfSetTabActive = 2054,
    TutFlash = 2055,
    IfSetText = 2056, // official
    LastLoginInfo = 2057,
    LastCom = 2058,
    LastInt = 2059, // official
    LastItem = 2060,
    LastSlot = 2061, // official
    LastTargetSlot = 2062,
    LastUseItem = 2063,  // official
    LastUseSlot = 2064,  // official
    LongQueue = 2065,    // official
    Mes = 2066,          // official
    MidiJingle = 2067,   // official, see cs2
    MidiSong = 2068,     // official, see cs2
    Name = 2069,         // official, joke reply
    PApRange = 2070,     // official
    PArriveDelay = 2071, // official
    PCountDialog = 2072, // official
    PDelay = 2073,       // official
    PExactMove = 2074,   // official
    PFindUid = 2075,     // official
    PLocMerge = 2076,    // official
    PLogout = 2077,
    POpHeld = 2078, // official
    POpLoc = 2079,  // official
    POpNpc = 2080,  // official
    POpNpcT = 2081, // official
    POpObj = 2082,
    POpPlayer = 2083,
    POpPlayerT = 2084,   // official
    PPauseButton = 2085, // official
    PStopAction = 2086,  // official
    PTeleJump = 2087,    // official
    PTeleport = 2088,
    PWalk = 2089,             // official
    PlayerFindAllZone = 2090, // todo: replace with huntall
    PlayerFindNext = 2091,    // todo: replace with huntnext
    Queue = 2092,             // official
    Say = 2093,               // official
    WalkTrigger = 2094,       // official
    SetTimer = 2095,
    SoftTimer = 2096,  // official
    SoundSynth = 2097, // official, newspost
    SpotAnimPl = 2098,
    StaffModLevel = 2099, // official
    Stat = 2100,          // official
    StatAdd = 2101,
    StatBase = 2102, // official
    StatHeal = 2103, // official
    StatSub = 2104,
    StrongQueue = 2105,
    Uid = 2106,       // official
    WeakQueue = 2107, // official
    IfOpenMainOverlay = 2108,
    AfkEvent = 2109,
    LowMemory = 2110,
    SetIdkit = 2111,
    PClearPendingAction = 2112, // official
    GetWalkTrigger = 2113,      // official
    Busy2 = 2114,               // official
    FindHero = 2115,            // official
    BothHeroPoints = 2116,      // official
    SetGender = 2117,
    SetSkinColour = 2118,
    PAnimProtect = 2119,
    RunEnergy = 2120,
    Weight = 2121,
    LastCoord = 2122,
    // Npc ops (2500-2999)
    NpcAdd = 2500,      // official
    NpcAnim = 2501,     // official, newspost
    NpcBaseStat = 2502, // official
    NpcCategory = 2503, // official
    NpcChangeType = 2504,
    NpcCoord = 2505, // official
    NpcDamage = 2506,
    NpcDel = 2507,        // official
    NpcDelay = 2508,      // official
    NpcFaceSquare = 2509, // official
    NpcFind = 2510,       // official
    NpcFindAllAny = 2511, // official
    NpcFindAll = 2512,
    NpcFindExact = 2513, // official
    NpcFindHero = 2514,  // official
    NpcFindAllZone = 2515,
    NpcFindNext = 2516,
    NpcFindUid = 2517,
    NpcGetMode = 2518,
    NpcHeroPoints = 2519, // official
    NpcName = 2520,
    NpcParam = 2521,   // official
    NpcQueue = 2522,   // official
    NpcRange = 2523,   // official
    NpcSay = 2524,     // official
    NpcHuntAll = 2525, // official
    NpcHuntNext = 2526,
    NpcSetHunt = 2527,     // official
    NpcSetHuntMode = 2528, // official
    NpcSetMode = 2529,     // official
    NpcWalkTrigger = 2530, // official
    NpcSetTimer = 2531,
    NpcStat = 2532,
    NpcStatAdd = 2533,
    NpcStatHeal = 2534, // official
    NpcStatSub = 2535,
    NpcTele = 2536,
    NpcType = 2537, // official
    NpcUid = 2538,
    SpotAnimNpc = 2539,
    NpcWalk = 2540,
    NpcAttackRange = 2541, // official
    NpcHasOp = 2542,       // official
    NpcArriveDelay = 2543,
    // Loc ops (3000-3499)
    LocAdd = 3000,      // official
    LocAngle = 3001,    // official
    LocAnim = 3002,     // official
    LocCategory = 3003, // official
    LocChange = 3004,
    LocCoord = 3005,       // official
    LocDel = 3006,         // official
    LocFind = 3007,        // official
    LocFindAllZone = 3008, // official
    LocFindNext = 3009,    // official
    LocName = 3010,
    LocParam = 3011, // official
    LocShape = 3012,
    LocType = 3013, // official
    // Obj ops (3500-4000)
    ObjAdd = 3500,
    ObjAddAll = 3501,
    ObjCoord = 3502,
    ObjCount = 3503,
    ObjDel = 3504,
    ObjName = 3505,
    ObjParam = 3506,
    ObjTakeItem = 3507,
    ObjType = 3508,
    ObjFind = 3509,
    // Npc config ops (4000-4099)
    NcCategory = 4000,
    NcDebugname = 4001,
    NcDesc = 4002,
    NcName = 4003,
    NcOp = 4004,
    NcParam = 4005,
    // Loc config ops (4100-4199)
    LcCategory = 4100,
    LcDebugname = 4101,
    LcDesc = 4102,
    LcName = 4103,
    LcOp = 4104,
    LcParam = 4105,
    LcWidth = 4106,
    LcLength = 4107,
    // Obj config ops (4200-4299)
    OcCategory = 4200, // official
    OcCert = 4201,     // official, see cs2
    OcCost = 4202,     // official, see cs2
    OcDebugname = 4203,
    OcDesc = 4204,      // official
    OcIop = 4205,       // official, see cs2
    OcMembers = 4206,   // official
    OcName = 4207,      // official
    OcOp = 4208,        // official, see cs2
    OcParam = 4209,     // official
    OcStackable = 4210, // official, see cs2
    OcTradeable = 4211,
    OcUncert = 4212, // official, see cs2
    OcWearPos2 = 4213,
    OcWearPos3 = 4214,
    OcWearPos = 4215,
    OcWeight = 4216,
    // Inventory ops (4300-4399)
    InvAllStock = 4300,
    InvSize = 4301, // official
    InvStockBase = 4302,
    InvAdd = 4303,        // official
    InvChangeSlot = 4304, // official
    InvClear = 4305,
    InvDel = 4306, // official
    InvDelSlot = 4307,
    InvDropItem = 4308,
    InvDropSlot = 4309,
    InvFreespace = 4310,
    InvGetNum = 4311,
    InvGetObj = 4312, // official
    InvItemSpace = 4313,
    InvItemSpace2 = 4314, // official
    InvMoveFromSlot = 4315,
    InvMoveToSlot = 4316,     // official
    BothMoveInv = 4317,       // official
    InvMoveItem = 4318,       // official
    InvMoveItemCert = 4319,   // official
    InvMoveItemUncert = 4320, // official
    InvSetSlot = 4321,        // official
    InvTotal = 4322,          // official
    InvTotalCat = 4323,
    InvTransmit = 4324,
    InvOtherTransmit = 4325,
    InvStopTransmit = 4326,
    BothDropSlot = 4327,
    InvDropAll = 4328,
    InvTotalParam = 4329,      // official, see cs2
    InvTotalParamStack = 4330, // official, see cs2
    // Enum ops (4400-4499)
    Enum = 4400,               // official
    EnumGetOutputCount = 4401, // official
    // String ops (4500-4599)
    AppendNum = 4500,           // official, see cs2
    Append = 4501,              // official, see cs2
    AppendSignNum = 4502,       // official, see cs2
    Lowercase = 4503,           // official, see cs2
    TextGender = 4504,          // official, see cs2
    ToString = 4505,            // official, see cs2
    Compare = 4506,             // official, see cs2
    TextSwitch = 4507,          // official, see cs2
    AppendChar = 4508,          // official, see cs2
    StringLength = 4509,        // official, see cs2
    SubString = 4510,           // official, see cs2
    StringIndexOfChar = 4511,   // official, see cs2
    StringIndexOfString = 4512, // official, see cs2
    // Number ops (4600-4699)
    Add = 4600,              // official, see cs2
    Sub = 4601,              // official, see cs2
    Multiply = 4602,         // official, see cs2
    Divide = 4603,           // official, see cs2
    Random = 4604,           // official, see cs2
    RandomInc = 4605,        // official, see cs2
    Interpolate = 4606,      // official, see cs2
    AddPercent = 4607,       // official, see cs2
    SetBit = 4608,           // official, see cs2
    ClearBit = 4609,         // official, see cs2
    TestBit = 4610,          // official, see cs2
    Modulo = 4611,           // official, see cs2
    Pow = 4612,              // official, see cs2
    InvPow = 4613,           // official, see cs2
    And = 4614,              // official, see cs2
    Or = 4615,               // official, see cs2
    Min = 4616,              // official, see cs2
    Max = 4617,              // official, see cs2
    Scale = 4618,            // official, see cs2
    BitCount = 4619,         // custom
    ToggleBit = 4620,        // custom
    SetBitRange = 4621,      // custom
    ClearBitRange = 4622,    // custom
    GetBitRange = 4623,      // custom
    SetBitRangeToInt = 4624, // custom
    SinDeg = 4625,           // custom
    CosDeg = 4626,           // custom
    Atan2Deg = 4627,         // custom
    Abs = 4628,              // custom
    // DB ops (7500-7599)
    DbFindWithCount = 7500,
    DbFindNext = 7501,
    DbGetField = 7502,
    DbGetFieldCount = 7503,
    DbListAllWithCount = 7504,
    DbGetRowTable = 7505,
    DbFindByIndex = 7506,
    DbFindRefineWithCount = 7507,
    DbFind = 7508,
    DbFindRefine = 7509,
    DbListAll = 7510,
    // Debug ops (10000-11000)
    Error = 10000,
    MapProduction = 10001,
    MapLastClock = 10002,        // custom
    MapLastWorld = 10003,        // custom
    MapLastClientIn = 10004,     // custom
    MapLastNpc = 10005,          // custom
    MapLastPlayer = 10006,       // custom
    MapLastLogout = 10007,       // custom
    MapLastLogin = 10008,        // custom
    MapLastZone = 10009,         // custom
    MapLastClientOut = 10010,    // custom
    MapLastCleanup = 10011,      // custom
    MapLastBandwidthIn = 10012,  // custom
    MapLastBandwidthOut = 10013, // custom
}

impl From<u16> for ScriptOpcode {
    fn from(code: u16) -> ScriptOpcode {
        match code {
            // Core language ops (0-99)
            0 => ScriptOpcode::PushConstantInt,
            1 => ScriptOpcode::PushVarp,
            2 => ScriptOpcode::PopVarp,
            3 => ScriptOpcode::PushConstantString,
            4 => ScriptOpcode::PushVarn,
            5 => ScriptOpcode::PopVarn,
            6 => ScriptOpcode::Branch,
            7 => ScriptOpcode::BranchNot,
            8 => ScriptOpcode::BranchEquals,
            9 => ScriptOpcode::BranchLessThan,
            10 => ScriptOpcode::BranchGreaterThan,
            11 => ScriptOpcode::PushVars,
            12 => ScriptOpcode::PopVars,
            21 => ScriptOpcode::Return,
            22 => ScriptOpcode::GoSub,
            23 => ScriptOpcode::Jump,
            24 => ScriptOpcode::Switch,
            25 => ScriptOpcode::PushVarbit,
            26 => ScriptOpcode::PopVarbit,
            31 => ScriptOpcode::BranchLessThanOrEquals,
            32 => ScriptOpcode::BranchGreaterThanOrEquals,
            33 => ScriptOpcode::PushIntLocal,
            34 => ScriptOpcode::PopIntLocal,
            35 => ScriptOpcode::PushStringLocal,
            36 => ScriptOpcode::PopStringLocal,
            37 => ScriptOpcode::JoinString,
            38 => ScriptOpcode::PopIntDiscard,
            39 => ScriptOpcode::PopStringDiscard,
            40 => ScriptOpcode::GoSubWithParams,
            41 => ScriptOpcode::JumpWithParams,
            42 => ScriptOpcode::PushVarcInt,
            43 => ScriptOpcode::PopVarcInt,
            44 => ScriptOpcode::DefineArray,
            45 => ScriptOpcode::PushArrayInt,
            46 => ScriptOpcode::PopArrayInt,
            100 => ScriptOpcode::EndCoreOps,
            // Server ops (1000-1999)
            1000 => ScriptOpcode::CoordX,
            1001 => ScriptOpcode::CoordY,
            1002 => ScriptOpcode::CoordZ,
            1003 => ScriptOpcode::Distance,
            1004 => ScriptOpcode::HuntAll,
            1005 => ScriptOpcode::HuntNext,
            1006 => ScriptOpcode::InZone,
            1007 => ScriptOpcode::LineOfSight,
            1008 => ScriptOpcode::LineOfWalk,
            1009 => ScriptOpcode::MapBlocked,
            1010 => ScriptOpcode::MapIndoors,
            1011 => ScriptOpcode::MapClock,
            1012 => ScriptOpcode::MapLocAddUnsafe,
            1013 => ScriptOpcode::MapMembers,
            1014 => ScriptOpcode::MapPlayerCount,
            1015 => ScriptOpcode::MapFindSquare,
            1016 => ScriptOpcode::MoveCoord,
            1017 => ScriptOpcode::PlayerCount,
            1018 => ScriptOpcode::ProjAnimMap,
            1019 => ScriptOpcode::ProjAnimNpc,
            1020 => ScriptOpcode::ProjAnimPl,
            1021 => ScriptOpcode::SeqLength,
            1022 => ScriptOpcode::SplitGet,
            1023 => ScriptOpcode::SplitGetAnim,
            1024 => ScriptOpcode::SplitInit,
            1025 => ScriptOpcode::SplitLineCount,
            1026 => ScriptOpcode::SplitPageCount,
            1027 => ScriptOpcode::SpotAnimMap,
            1028 => ScriptOpcode::StatRandom,
            1029 => ScriptOpcode::StructParam,
            1030 => ScriptOpcode::WorldDelay,
            1031 => ScriptOpcode::NpcsCount,
            1032 => ScriptOpcode::ZonesCount,
            1033 => ScriptOpcode::LocsCount,
            1034 => ScriptOpcode::ObjsCount,
            1035 => ScriptOpcode::MapMulti,
            // Player ops (2000-2499)
            2000 => ScriptOpcode::AllowDesign,
            2001 => ScriptOpcode::Anim,
            2002 => ScriptOpcode::BasReadyAnim,
            2003 => ScriptOpcode::BasRunning,
            2004 => ScriptOpcode::BasTurnOnSpot,
            2005 => ScriptOpcode::BasWalkB,
            2006 => ScriptOpcode::BasWalkF,
            2007 => ScriptOpcode::BasWalkL,
            2008 => ScriptOpcode::BasWalkR,
            2009 => ScriptOpcode::BufferFull,
            2010 => ScriptOpcode::BuildAppearance,
            2011 => ScriptOpcode::Busy,
            2012 => ScriptOpcode::CamLookAt,
            2013 => ScriptOpcode::CamMoveTo,
            2014 => ScriptOpcode::CamReset,
            2015 => ScriptOpcode::CamShake,
            2016 => ScriptOpcode::ClearQueue,
            2017 => ScriptOpcode::ClearSoftTimer,
            2018 => ScriptOpcode::ClearTimer,
            2019 => ScriptOpcode::GetTimer,
            2020 => ScriptOpcode::Coord,
            2021 => ScriptOpcode::Damage,
            2022 => ScriptOpcode::Displayname,
            2023 => ScriptOpcode::FaceSquare,
            2024 => ScriptOpcode::FindUid,
            2025 => ScriptOpcode::Gender,
            2026 => ScriptOpcode::GetQueue,
            2027 => ScriptOpcode::StatAdvance,
            2028 => ScriptOpcode::HeadiconsGet,
            2029 => ScriptOpcode::HeadiconsSet,
            2030 => ScriptOpcode::HealEnergy,
            2031 => ScriptOpcode::HintCoord,
            2032 => ScriptOpcode::HintNpc,
            2033 => ScriptOpcode::HintPlayer,
            2034 => ScriptOpcode::HintStop,
            2035 => ScriptOpcode::IfClose,
            2036 => ScriptOpcode::TutClose,
            2037 => ScriptOpcode::IfMultiZone,
            2038 => ScriptOpcode::IfOpenChat,
            2039 => ScriptOpcode::TutOpen,
            2040 => ScriptOpcode::IfOpenMain,
            2041 => ScriptOpcode::IfOpenMainSide,
            2042 => ScriptOpcode::IfOpenSide,
            2043 => ScriptOpcode::IfSetAnim,
            2044 => ScriptOpcode::IfSetColour,
            2045 => ScriptOpcode::IfSetHide,
            2046 => ScriptOpcode::IfSetModel,
            2047 => ScriptOpcode::IfSetRecol,
            2048 => ScriptOpcode::IfSetNpcHead,
            2049 => ScriptOpcode::IfSetObject,
            2050 => ScriptOpcode::IfSetPlayerHead,
            2051 => ScriptOpcode::IfSetPosition,
            2052 => ScriptOpcode::IfSetResumeButtons,
            2053 => ScriptOpcode::IfSetTab,
            2054 => ScriptOpcode::IfSetTabActive,
            2055 => ScriptOpcode::TutFlash,
            2056 => ScriptOpcode::IfSetText,
            2057 => ScriptOpcode::LastLoginInfo,
            2058 => ScriptOpcode::LastCom,
            2059 => ScriptOpcode::LastInt,
            2060 => ScriptOpcode::LastItem,
            2061 => ScriptOpcode::LastSlot,
            2062 => ScriptOpcode::LastTargetSlot,
            2063 => ScriptOpcode::LastUseItem,
            2064 => ScriptOpcode::LastUseSlot,
            2065 => ScriptOpcode::LongQueue,
            2066 => ScriptOpcode::Mes,
            2067 => ScriptOpcode::MidiJingle,
            2068 => ScriptOpcode::MidiSong,
            2069 => ScriptOpcode::Name,
            2070 => ScriptOpcode::PApRange,
            2071 => ScriptOpcode::PArriveDelay,
            2072 => ScriptOpcode::PCountDialog,
            2073 => ScriptOpcode::PDelay,
            2074 => ScriptOpcode::PExactMove,
            2075 => ScriptOpcode::PFindUid,
            2076 => ScriptOpcode::PLocMerge,
            2077 => ScriptOpcode::PLogout,
            2078 => ScriptOpcode::POpHeld,
            2079 => ScriptOpcode::POpLoc,
            2080 => ScriptOpcode::POpNpc,
            2081 => ScriptOpcode::POpNpcT,
            2082 => ScriptOpcode::POpObj,
            2083 => ScriptOpcode::POpPlayer,
            2084 => ScriptOpcode::POpPlayerT,
            2085 => ScriptOpcode::PPauseButton,
            2086 => ScriptOpcode::PStopAction,
            2087 => ScriptOpcode::PTeleJump,
            2088 => ScriptOpcode::PTeleport,
            2089 => ScriptOpcode::PWalk,
            2090 => ScriptOpcode::PlayerFindAllZone,
            2091 => ScriptOpcode::PlayerFindNext,
            2092 => ScriptOpcode::Queue,
            2093 => ScriptOpcode::Say,
            2094 => ScriptOpcode::WalkTrigger,
            2095 => ScriptOpcode::SetTimer,
            2096 => ScriptOpcode::SoftTimer,
            2097 => ScriptOpcode::SoundSynth,
            2098 => ScriptOpcode::SpotAnimPl,
            2099 => ScriptOpcode::StaffModLevel,
            2100 => ScriptOpcode::Stat,
            2101 => ScriptOpcode::StatAdd,
            2102 => ScriptOpcode::StatBase,
            2103 => ScriptOpcode::StatHeal,
            2104 => ScriptOpcode::StatSub,
            2105 => ScriptOpcode::StrongQueue,
            2106 => ScriptOpcode::Uid,
            2107 => ScriptOpcode::WeakQueue,
            2108 => ScriptOpcode::IfOpenMainOverlay,
            2109 => ScriptOpcode::AfkEvent,
            2110 => ScriptOpcode::LowMemory,
            2111 => ScriptOpcode::SetIdkit,
            2112 => ScriptOpcode::PClearPendingAction,
            2113 => ScriptOpcode::GetWalkTrigger,
            2114 => ScriptOpcode::Busy2,
            2115 => ScriptOpcode::FindHero,
            2116 => ScriptOpcode::BothHeroPoints,
            2117 => ScriptOpcode::SetGender,
            2118 => ScriptOpcode::SetSkinColour,
            2119 => ScriptOpcode::PAnimProtect,
            2120 => ScriptOpcode::RunEnergy,
            2121 => ScriptOpcode::Weight,
            2122 => ScriptOpcode::LastCoord,
            // Npc ops (2500-2999)
            2500 => ScriptOpcode::NpcAdd,
            2501 => ScriptOpcode::NpcAnim,
            2502 => ScriptOpcode::NpcBaseStat,
            2503 => ScriptOpcode::NpcCategory,
            2504 => ScriptOpcode::NpcChangeType,
            2505 => ScriptOpcode::NpcCoord,
            2506 => ScriptOpcode::NpcDamage,
            2507 => ScriptOpcode::NpcDel,
            2508 => ScriptOpcode::NpcDelay,
            2509 => ScriptOpcode::NpcFaceSquare,
            2510 => ScriptOpcode::NpcFind,
            2511 => ScriptOpcode::NpcFindAllAny,
            2512 => ScriptOpcode::NpcFindAll,
            2513 => ScriptOpcode::NpcFindExact,
            2514 => ScriptOpcode::NpcFindHero,
            2515 => ScriptOpcode::NpcFindAllZone,
            2516 => ScriptOpcode::NpcFindNext,
            2517 => ScriptOpcode::NpcFindUid,
            2518 => ScriptOpcode::NpcGetMode,
            2519 => ScriptOpcode::NpcHeroPoints,
            2520 => ScriptOpcode::NpcName,
            2521 => ScriptOpcode::NpcParam,
            2522 => ScriptOpcode::NpcQueue,
            2523 => ScriptOpcode::NpcRange,
            2524 => ScriptOpcode::NpcSay,
            2525 => ScriptOpcode::NpcHuntAll,
            2526 => ScriptOpcode::NpcHuntNext,
            2527 => ScriptOpcode::NpcSetHunt,
            2528 => ScriptOpcode::NpcSetHuntMode,
            2529 => ScriptOpcode::NpcSetMode,
            2530 => ScriptOpcode::NpcWalkTrigger,
            2531 => ScriptOpcode::NpcSetTimer,
            2532 => ScriptOpcode::NpcStat,
            2533 => ScriptOpcode::NpcStatAdd,
            2534 => ScriptOpcode::NpcStatHeal,
            2535 => ScriptOpcode::NpcStatSub,
            2536 => ScriptOpcode::NpcTele,
            2537 => ScriptOpcode::NpcType,
            2538 => ScriptOpcode::NpcUid,
            2539 => ScriptOpcode::SpotAnimNpc,
            2540 => ScriptOpcode::NpcWalk,
            2541 => ScriptOpcode::NpcAttackRange,
            2542 => ScriptOpcode::NpcHasOp,
            2543 => ScriptOpcode::NpcArriveDelay,
            // Loc ops (3000-3499)
            3000 => ScriptOpcode::LocAdd,
            3001 => ScriptOpcode::LocAngle,
            3002 => ScriptOpcode::LocAnim,
            3003 => ScriptOpcode::LocCategory,
            3004 => ScriptOpcode::LocChange,
            3005 => ScriptOpcode::LocCoord,
            3006 => ScriptOpcode::LocDel,
            3007 => ScriptOpcode::LocFind,
            3008 => ScriptOpcode::LocFindAllZone,
            3009 => ScriptOpcode::LocFindNext,
            3010 => ScriptOpcode::LocName,
            3011 => ScriptOpcode::LocParam,
            3012 => ScriptOpcode::LocShape,
            3013 => ScriptOpcode::LocType,
            // Obj ops (3500-4000)
            3500 => ScriptOpcode::ObjAdd,
            3501 => ScriptOpcode::ObjAddAll,
            3502 => ScriptOpcode::ObjCoord,
            3503 => ScriptOpcode::ObjCount,
            3504 => ScriptOpcode::ObjDel,
            3505 => ScriptOpcode::ObjName,
            3506 => ScriptOpcode::ObjParam,
            3507 => ScriptOpcode::ObjTakeItem,
            3508 => ScriptOpcode::ObjType,
            3509 => ScriptOpcode::ObjFind,
            // Npc config ops (4000-4099)
            4000 => ScriptOpcode::NcCategory,
            4001 => ScriptOpcode::NcDebugname,
            4002 => ScriptOpcode::NcDesc,
            4003 => ScriptOpcode::NcName,
            4004 => ScriptOpcode::NcOp,
            4005 => ScriptOpcode::NcParam,
            // Loc config ops (4100-4199)
            4100 => ScriptOpcode::LcCategory,
            4101 => ScriptOpcode::LcDebugname,
            4102 => ScriptOpcode::LcDesc,
            4103 => ScriptOpcode::LcName,
            4104 => ScriptOpcode::LcOp,
            4105 => ScriptOpcode::LcParam,
            4106 => ScriptOpcode::LcWidth,
            4107 => ScriptOpcode::LcLength,
            // Obj config ops (4200-4299)
            4200 => ScriptOpcode::OcCategory,
            4201 => ScriptOpcode::OcCert,
            4202 => ScriptOpcode::OcCost,
            4203 => ScriptOpcode::OcDebugname,
            4204 => ScriptOpcode::OcDesc,
            4205 => ScriptOpcode::OcIop,
            4206 => ScriptOpcode::OcMembers,
            4207 => ScriptOpcode::OcName,
            4208 => ScriptOpcode::OcOp,
            4209 => ScriptOpcode::OcParam,
            4210 => ScriptOpcode::OcStackable,
            4211 => ScriptOpcode::OcTradeable,
            4212 => ScriptOpcode::OcUncert,
            4213 => ScriptOpcode::OcWearPos2,
            4214 => ScriptOpcode::OcWearPos3,
            4215 => ScriptOpcode::OcWearPos,
            4216 => ScriptOpcode::OcWeight,
            // Inventory ops (4300-4399)
            4300 => ScriptOpcode::InvAllStock,
            4301 => ScriptOpcode::InvSize,
            4302 => ScriptOpcode::InvStockBase,
            4303 => ScriptOpcode::InvAdd,
            4304 => ScriptOpcode::InvChangeSlot,
            4305 => ScriptOpcode::InvClear,
            4306 => ScriptOpcode::InvDel,
            4307 => ScriptOpcode::InvDelSlot,
            4308 => ScriptOpcode::InvDropItem,
            4309 => ScriptOpcode::InvDropSlot,
            4310 => ScriptOpcode::InvFreespace,
            4311 => ScriptOpcode::InvGetNum,
            4312 => ScriptOpcode::InvGetObj,
            4313 => ScriptOpcode::InvItemSpace,
            4314 => ScriptOpcode::InvItemSpace2,
            4315 => ScriptOpcode::InvMoveFromSlot,
            4316 => ScriptOpcode::InvMoveToSlot,
            4317 => ScriptOpcode::BothMoveInv,
            4318 => ScriptOpcode::InvMoveItem,
            4319 => ScriptOpcode::InvMoveItemCert,
            4320 => ScriptOpcode::InvMoveItemUncert,
            4321 => ScriptOpcode::InvSetSlot,
            4322 => ScriptOpcode::InvTotal,
            4323 => ScriptOpcode::InvTotalCat,
            4324 => ScriptOpcode::InvTransmit,
            4325 => ScriptOpcode::InvOtherTransmit,
            4326 => ScriptOpcode::InvStopTransmit,
            4327 => ScriptOpcode::BothDropSlot,
            4328 => ScriptOpcode::InvDropAll,
            4329 => ScriptOpcode::InvTotalParam,
            4330 => ScriptOpcode::InvTotalParamStack,
            // Enum ops (4400-4499)
            4400 => ScriptOpcode::Enum,
            4401 => ScriptOpcode::EnumGetOutputCount,
            // String ops (4500-4599)
            4500 => ScriptOpcode::AppendNum,
            4501 => ScriptOpcode::Append,
            4502 => ScriptOpcode::AppendSignNum,
            4503 => ScriptOpcode::Lowercase,
            4504 => ScriptOpcode::TextGender,
            4505 => ScriptOpcode::ToString,
            4506 => ScriptOpcode::Compare,
            4507 => ScriptOpcode::TextSwitch,
            4508 => ScriptOpcode::AppendChar,
            4509 => ScriptOpcode::StringLength,
            4510 => ScriptOpcode::SubString,
            4511 => ScriptOpcode::StringIndexOfChar,
            4512 => ScriptOpcode::StringIndexOfString,
            // Number ops (4600-4699)
            4600 => ScriptOpcode::Add,
            4601 => ScriptOpcode::Sub,
            4602 => ScriptOpcode::Multiply,
            4603 => ScriptOpcode::Divide,
            4604 => ScriptOpcode::Random,
            4605 => ScriptOpcode::RandomInc,
            4606 => ScriptOpcode::Interpolate,
            4607 => ScriptOpcode::AddPercent,
            4608 => ScriptOpcode::SetBit,
            4609 => ScriptOpcode::ClearBit,
            4610 => ScriptOpcode::TestBit,
            4611 => ScriptOpcode::Modulo,
            4612 => ScriptOpcode::Pow,
            4613 => ScriptOpcode::InvPow,
            4614 => ScriptOpcode::And,
            4615 => ScriptOpcode::Or,
            4616 => ScriptOpcode::Min,
            4617 => ScriptOpcode::Max,
            4618 => ScriptOpcode::Scale,
            4619 => ScriptOpcode::BitCount,
            4620 => ScriptOpcode::ToggleBit,
            4621 => ScriptOpcode::SetBitRange,
            4622 => ScriptOpcode::ClearBitRange,
            4623 => ScriptOpcode::GetBitRange,
            4624 => ScriptOpcode::SetBitRangeToInt,
            4625 => ScriptOpcode::SinDeg,
            4626 => ScriptOpcode::CosDeg,
            4627 => ScriptOpcode::Atan2Deg,
            4628 => ScriptOpcode::Abs,
            // DB ops (7500-7599)
            7500 => ScriptOpcode::DbFindWithCount,
            7501 => ScriptOpcode::DbFindNext,
            7502 => ScriptOpcode::DbGetField,
            7503 => ScriptOpcode::DbGetFieldCount,
            7504 => ScriptOpcode::DbListAllWithCount,
            7505 => ScriptOpcode::DbGetRowTable,
            7506 => ScriptOpcode::DbFindByIndex,
            7507 => ScriptOpcode::DbFindRefineWithCount,
            7508 => ScriptOpcode::DbFind,
            7509 => ScriptOpcode::DbFindRefine,
            7510 => ScriptOpcode::DbListAll,
            // Debug ops (10000-11000)
            10000 => ScriptOpcode::Error,
            10001 => ScriptOpcode::MapProduction,
            10002 => ScriptOpcode::MapLastClock,
            10003 => ScriptOpcode::MapLastWorld,
            10004 => ScriptOpcode::MapLastClientIn,
            10005 => ScriptOpcode::MapLastNpc,
            10006 => ScriptOpcode::MapLastPlayer,
            10007 => ScriptOpcode::MapLastLogout,
            10008 => ScriptOpcode::MapLastLogin,
            10009 => ScriptOpcode::MapLastZone,
            10010 => ScriptOpcode::MapLastClientOut,
            10011 => ScriptOpcode::MapLastCleanup,
            10012 => ScriptOpcode::MapLastBandwidthIn,
            10013 => ScriptOpcode::MapLastBandwidthOut,
            _ => panic!("Invalid script opcode value: {}", code),
        }
    }
}

#[derive(Clone)]
pub struct ScriptProvider {
    names: HashMap<String, usize>,
    scripts: Vec<Option<ScriptFile>>,
    lookups: HashMap<i32, usize>,
}

impl ScriptProvider {
    pub fn io(dir: &str) -> ScriptProvider {
        let start: Instant = Instant::now();
        let mut dat: Packet = Packet::io(format!("{}/server/script.dat", dir));
        let mut idx: Packet = Packet::io(format!("{}/server/script.idx", dir));

        let count: usize = dat.g2() as usize;
        idx.pos += 2;

        let version: i32 = dat.g4s();
        let compiler: &String = &std::env::var("COMPILER_VERSION").unwrap();
        if !version.to_string().eq(compiler) {
            panic!("RuneScript compiler is out of date!");
        }

        let mut names: HashMap<String, usize> = HashMap::new();
        let mut scripts: Vec<Option<ScriptFile>> = vec![None; count];
        let mut lookups: HashMap<i32, usize> = HashMap::new();

        for (index, option) in scripts.iter_mut().enumerate() {
            let length: usize = idx.g2() as usize;
            if length == 0 {
                continue;
            }

            let start: usize = dat.pos;
            let end: usize = start + length;

            let script: ScriptFile = ScriptFile::new(&mut dat, index, length);

            let info: &ScriptInfo = &script.info;
            names.insert(info.name.clone(), index);
            // add the script to lookup table if the value isn't -1
            if info.lookup != -1 {
                lookups.insert(info.lookup, index);
            }

            *option = Some(script);

            if dat.pos > end {
                panic!("Script {} has read past end!", index);
            }

            dat.pos = end;
        }

        println!("Loaded scripts in: {:?}", start.elapsed());
        return ScriptProvider {
            names,
            scripts,
            lookups,
        };
    }

    /// Retrieves a script by its ID, invoking the provided callback functions
    /// based on whether the script is found or not.
    ///
    /// This method looks up the script in the internal `scripts` collection using
    /// the provided `id`. If the script is found, the `on_found` callback is invoked
    /// with a reference to the script. If the script is not found, the `on_not_found`
    /// callback is invoked.
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the script to retrieve from the `scripts`.
    /// * `on_found` - A closure that is called with a reference to the found `ScriptFile`
    ///                if the script exists in the `scripts` collection.
    /// * `on_not_found` - A closure that is called if the script does not exist in
    ///                    the `scripts` collection.
    ///
    /// # Behavior
    ///
    /// - If the script with the given `id` is found in `scripts`, the `on_found` closure
    ///   will be executed with the found `ScriptFile`.
    /// - If the script is not found, the `on_not_found` closure will be executed.
    ///
    /// # Panics
    ///
    /// This function does not panic under normal operation.
    /// However, the callbacks may panic if they are implemented to do so.
    ///
    /// # Safety
    ///
    /// There are no specific safety concerns for this function. It is safe as long as the internal data structures of `ScriptProvider` are properly initialized.
    ///
    /// # Side Effects
    ///
    /// - Calls the appropriate callback (`on_found` or `on_not_found`) based on the existence of the script with the given ID.
    ///
    /// # Performance
    ///
    /// The time complexity of this function is O(1), as it performs a direct lookup
    /// on the `scripts` vector using the index.
    pub fn on_by_id<F, E>(&self, id: usize, on_found: F, on_not_found: E)
    where
        F: FnOnce(&ScriptFile),
        E: FnOnce(),
    {
        return self
            .scripts
            .get(id)
            .and_then(|option| option.as_ref())
            .map(|obj| on_found(obj))
            .unwrap_or(on_not_found());
    }

    /// Retrieves a script by its ID, returning a `Result` that indicates
    /// success or failure of the operation.
    ///
    /// This method looks up the script in the internal `scripts` vector using
    /// the provided `id`. If the script is found, a reference to the `ScriptFile`
    /// is returned wrapped in `Ok`. If the script is not found, an error message
    /// is returned wrapped in `Err`.
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the script to retrieve from the `scripts`
    ///          vector.
    ///
    /// # Behavior
    ///
    /// - If the script with the given `id` is found in `scripts`, a reference to the
    ///   script is returned as `Ok(&ScriptFile)`.
    /// - If the script is not found, an error message indicating the absence of
    ///   the script is returned as `Err(String)`.
    ///
    /// # Panics
    ///
    /// This function does not panic under normal operation. However, it will
    /// return an error if the script is not found.
    ///
    /// # Safety
    ///
    /// There are no specific safety concerns for this function. It is safe as long
    /// as the internal data structures of the `ScriptProvider` are properly initialized.
    ///
    /// # Performance
    ///
    /// The time complexity of this function is O(1) for accessing elements in the
    /// `scripts` vector by index. If the `id` is out of bounds, the method will return
    /// an error without panic.
    pub fn get_by_id(&self, id: usize) -> Result<&ScriptFile, String> {
        return self
            .scripts
            .get(id)
            .and_then(|option| option.as_ref())
            .ok_or(format!("Script not found for id: {}", id));
    }

    /// Retrieves a script by its name and executes a callback based on whether the script is found or not.
    ///
    /// This method searches for the script name in the internal `names` map. If found, it retrieves the script
    /// from the `scripts` collection and invokes the `on_found` callback. If the script is not found, it invokes the
    /// `on_not_found` callback.
    ///
    /// # Parameters
    ///
    /// - `name`: The name of the script to search for. This is a string slice (`&str`) and is used to look up the script ID in the internal `names` map.
    /// - `on_found`: A callback function that takes a reference to the `ScriptFile` and is executed when the script is found.
    /// - `on_not_found`: A callback function that takes no arguments and is executed when the script is not found.
    ///
    /// # Return
    ///
    /// This function does not return any value. Instead, it calls the provided callbacks based on whether the script is found or not.
    ///
    /// # Panics
    ///
    /// This function does not panic under normal operation.
    /// However, the callbacks may panic if they are implemented to do so.
    ///
    /// # Safety
    ///
    /// There are no specific safety concerns for this function. It is safe as long as the internal data structures of `ScriptProvider` are properly initialized.
    ///
    /// # Side Effects
    ///
    /// - Calls the appropriate callback (`on_found` or `on_not_found`) based on the existence of the script with the given name.
    ///
    /// # Performance
    ///
    /// This function performs two lookups: one in the `names` map and another in the `scripts` collection. If the name does not exist,
    /// it performs minimal work. The time complexity is O(1) for both lookups.
    pub fn on_by_name<F, E>(&self, name: &str, on_found: F, on_not_found: E)
    where
        F: FnOnce(&ScriptFile),
        E: FnOnce(),
    {
        if let Some(&id) = self.names.get(name) {
            self.on_by_id(id, on_found, on_not_found);
        } else {
            on_not_found();
        }
    }

    /// Retrieves a script by its name.
    ///
    /// This method searches for the script name in the internal `names` map. If found, it retrieves the script
    /// from the `scripts` vector using its ID. If the script is not found, an error message is returned.
    ///
    /// # Parameters
    ///
    /// - `name`: The name of the script to search for. This is a string slice (`&str`) and is used to look up the script ID in the internal `names` map.
    ///
    /// # Return
    ///
    /// This function returns a `Result`:
    /// - `Ok(&ScriptFile)` if the script is found, containing a reference to the script.
    /// - `Err(String)` if the script is not found, containing an error message.
    ///
    /// # Panics
    ///
    /// This function does not panic under normal operation. However, it may return an error if the script is not found.
    ///
    /// # Safety
    ///
    /// There are no specific safety concerns for this function. It is safe as long as the internal data structures of `ScriptProvider` are properly initialized.
    ///
    /// # Performance
    ///
    /// This function performs a lookup in the `names` map to find the script ID and then retrieves the script
    /// from the `scripts` vector using that ID. The time complexity for both lookups is O(1).
    pub fn get_by_name(&self, name: &str) -> Result<&ScriptFile, String> {
        return self
            .names
            .get(name)
            .map(|&id| self.get_by_id(id))
            .unwrap_or_else(|| Err(format!("Script not found for name: {}", name)))
            .and_then(|result| Ok(result));
    }

    /// Retrieves a script based on a specified trigger, ID, or category, executing a callback if found.
    ///
    /// This method attempts to locate a `ScriptFile` using a combination of the provided `trigger`, `id`,
    /// and `category` values. It constructs keys based on these parameters and searches the internal
    /// `lookups` mapping for the corresponding script index. If a script is found, the `on_found`
    /// callback is invoked with the retrieved `ScriptFile`. If no script is found after checking all
    /// possible keys, the `on_not_found` callback is executed.
    ///
    /// # Parameters
    ///
    /// - `trigger`: An `i32` used as a base for constructing the lookup keys.
    /// - `id`: An `i32` that contributes to a key for finding scripts associated with a specific ID.
    /// - `category`: An `i32` that is part of the key used to find scripts related to a category.
    /// - `on_found`: A callback function that is executed with a reference to the found `ScriptFile`.
    /// - `on_not_found`: A callback function that is executed when no script is found.
    ///
    /// # Return
    ///
    /// This function does not return any value. It executes the provided callbacks based on whether
    /// the script is found or not.
    ///
    /// # Panics
    ///
    /// This function does not panic under normal operation. However, the provided callbacks may panic
    /// if they are implemented to do so.
    ///
    /// # Safety
    ///
    /// There are no specific safety concerns for this function. It is safe as long as the internal data
    /// structures of the `ScriptProvider` are properly initialized.
    ///
    /// # Side Effects
    ///
    /// - Calls either the `on_found` or `on_not_found` callback depending on whether the script was
    /// found.
    ///
    /// # Performance
    ///
    /// This function performs up to three lookups in the `lookups` map and may perform an additional
    /// lookup in the `scripts` array. The time complexity is O(1) for each lookup, resulting in a
    /// worst-case scenario of O(3).
    pub fn get_by_trigger<'script, 'state: 'script, F, E>(
        &'state self,
        trigger: i32,
        id: i32,
        category: i32,
        on_found: F,
        on_not_found: E,
    ) where
        F: FnOnce(&'script ScriptFile),
        E: FnOnce(),
    {
        let keys: [i32; 3] = [
            trigger | (0x2 << 8) | (id << 10),
            trigger | (0x1 << 8) | (category << 10),
            trigger,
        ];

        for key in &keys {
            if let Some(index) = self.lookups.get(key) {
                if let Some(option) = self.scripts.get(*index) {
                    if let Some(script) = option {
                        on_found(script);
                        return;
                    }
                }
            }
        }
        on_not_found();
    }

    /// Retrieves a script based on a specified trigger, ID, or category, executing a callback if found.
    ///
    /// This method attempts to locate a `ScriptFile` using a combination of the provided `trigger`, `id`,
    /// and `category` values. It constructs keys based on these parameters and searches the internal
    /// `lookups` mapping for the corresponding script index. If a script is found using the `id`,
    /// the `on_found` callback is invoked with the retrieved `ScriptFile`. If no script is found with the
    /// `id`, it checks for the `category` and does the same. If neither is found, it checks just the `trigger`.
    /// If no script is found after all checks, the `on_not_found` callback is executed.
    ///
    /// # Parameters
    ///
    /// - `trigger`: An `i32` used as a base for constructing the lookup keys.
    /// - `id`: An `i32` that contributes to a key for finding scripts associated with a specific ID.
    ///          If set to `-1`, the `id` check is skipped.
    /// - `category`: An `i32` that is part of the key used to find scripts related to a category.
    ///               If set to `-1`, the `category` check is skipped.
    /// - `on_found`: A callback function that is executed with a reference to the found `ScriptFile`.
    /// - `on_not_found`: A callback function that is executed when no script is found.
    ///
    /// # Return
    ///
    /// This function does not return any value. It executes the provided callbacks based on whether
    /// the script is found or not.
    ///
    /// # Panics
    ///
    /// This function does not panic under normal operation. However, the provided callbacks may panic
    /// if they are implemented to do so.
    ///
    /// # Safety
    ///
    /// There are no specific safety concerns for this function. It is safe as long as the internal data
    /// structures of the `ScriptProvider` are properly initialized.
    ///
    /// # Side Effects
    ///
    /// - Calls either the `on_found` or `on_not_found` callback depending on whether the script was
    /// found.
    ///
    /// # Performance
    ///
    /// This function performs up to three lookups in the `lookups` map and may perform an additional
    /// lookup in the `scripts` array. The time complexity is O(1) for each lookup, resulting in a
    /// worst-case scenario of O(3).
    pub fn get_by_trigger_specific<'script, 'state: 'script, F, E>(
        &'state self,
        trigger: i32,
        id: i32,
        category: i32,
        on_found: F,
        on_not_found: E,
    ) where
        F: FnOnce(&'script ScriptFile),
        E: FnOnce(),
    {
        let get_script = |trigger_key| {
            self.lookups
                .get(&trigger_key)
                .and_then(|index| self.scripts.get(*index).and_then(|option| option.as_ref()))
        };

        let trigger_key = if id != -1 {
            trigger | (0x2 << 8) | (id << 10)
        } else if category != -1 {
            trigger | (0x1 << 8) | (category << 10)
        } else {
            trigger
        };

        match get_script(trigger_key) {
            Some(script) => on_found(script),
            None => on_not_found(),
        }
    }
}

#[derive(Clone)]
pub struct ScriptInfo {
    pub name: String,
    pub path: String,
    pub lookup: i32,
    pub params: Vec<u8>,
    pub pcs: Vec<i32>,
    pub lines: Vec<i32>,
}

#[derive(Clone)]
pub struct ScriptFile {
    id: usize,
    int_local_count: u16,
    string_local_count: u16,
    pub int_arg_count: u16,
    pub string_arg_count: u16,
    pub switch_table: Option<HashMap<i32, i32>>,
    pub info: ScriptInfo,
    pub codes: Option<Vec<Option<ScriptOpcode>>>,
    pub int_operands: Vec<i32>,
    pub string_operands: Vec<String>,
}

impl ScriptFile {
    fn is_large_operand(code: &ScriptOpcode) -> bool {
        if code > &ScriptOpcode::EndCoreOps {
            return false;
        }
        return match code {
            ScriptOpcode::Return => false,
            ScriptOpcode::PopIntDiscard => false,
            ScriptOpcode::PopStringDiscard => false,
            ScriptOpcode::GoSub => false,
            ScriptOpcode::Jump => false,
            _ => true,
        };
    }

    fn new(dat: &mut Packet, id: usize, length: usize) -> ScriptFile {
        let start: usize = dat.pos;
        let end: usize = start + length;

        if end < 16 {
            panic!("Invalid script file (minimum length) must be 16 bytes.");
        }

        dat.pos = end - 2;

        let trailer_len: usize = dat.g2() as usize;
        let trailer_pos: usize = end - trailer_len - 12 - 2;

        if trailer_pos >= end {
            panic!("Invalid script file (bad trailer pos).");
        }

        dat.pos = trailer_pos;

        let instructions: usize = dat.g4s() as usize;
        let int_local_count: u16 = dat.g2();
        let string_local_count: u16 = dat.g2();
        let int_arg_count: u16 = dat.g2();
        let string_arg_count: u16 = dat.g2();
        let mut switch_table: Option<HashMap<i32, i32>> = None;

        let switches: usize = dat.g1() as usize;
        for _ in 0..switches {
            let count: usize = dat.g2() as usize;
            let mut table: HashMap<i32, i32> = HashMap::new();

            for _ in 0..count {
                table.insert(dat.g4s(), dat.g4s());
            }

            switch_table = Some(table);
        }

        dat.pos = start;
        let name: String = dat.gjstr(0);
        let path: String = dat.gjstr(0);
        let lookup: i32 = dat.g4s();

        let params_count: usize = dat.g1() as usize;
        let mut params: Vec<u8> = vec![0; params_count];
        for param in params.iter_mut() {
            *param = dat.g1();
        }

        let lines_count: usize = dat.g2() as usize;
        let mut pcs: Vec<i32> = vec![0; lines_count];
        let mut lines: Vec<i32> = vec![0; lines_count];
        for index in 0..lines_count {
            pcs[index] = dat.g4s();
            lines[index] = dat.g4s();
        }

        let info: ScriptInfo = ScriptInfo {
            name,
            path,
            lookup,
            params,
            pcs,
            lines,
        };

        let mut opcodes: Vec<Option<ScriptOpcode>> = vec![None; instructions];
        let mut int_operands: Vec<i32> = vec![0; instructions];
        let mut string_operands: Vec<String> = vec![String::new(); instructions];

        let mut pc: usize = 0;
        while trailer_pos > dat.pos {
            let code: ScriptOpcode = ScriptOpcode::from(dat.g2());

            if code == ScriptOpcode::PushConstantString {
                string_operands[pc] = dat.gjstr(0);
            } else if ScriptFile::is_large_operand(&code) {
                int_operands[pc] = dat.g4s();
            } else {
                int_operands[pc] = dat.g1() as i32;
            }

            opcodes[pc] = Some(code);
            pc += 1;
        }

        return ScriptFile {
            id,
            int_local_count,
            string_local_count,
            int_arg_count,
            string_arg_count,
            switch_table,
            info,
            codes: Some(opcodes),
            int_operands,
            string_operands,
        };
    }

    pub fn mock() -> ScriptFile {
        return ScriptFile {
            id: 0,
            int_local_count: 0,
            string_local_count: 0,
            int_arg_count: 0,
            string_arg_count: 0,
            switch_table: None,
            info: ScriptInfo {
                name: String::new(),
                path: String::new(),
                lookup: 0,
                params: Vec::new(),
                pcs: Vec::new(),
                lines: Vec::new(),
            },
            codes: None,
            int_operands: Vec::new(),
            string_operands: Vec::new(),
        };
    }
}

struct GoSubFrame<'script> {
    script: &'script ScriptFile,
    pc: i32, // program counter
    int_locals: Vec<i32>,
    string_locals: Vec<String>,
}

impl<'script> GoSubFrame<'script> {}

struct JumpFrame<'script> {
    script: &'script ScriptFile,
    pc: i32,
}

impl<'script> JumpFrame<'script> {}

#[derive(PartialEq)]
#[repr(i8)]
pub enum ScriptExecutionState {
    Aborted = -1,
    Running = 0,
    Finished = 1,
    Suspended = 2, // suspended to move to player
    PauseButton = 3,
    CountDialog = 4,
    NpcSuspended = 5,   // suspended to move to npc
    WorldSuspended = 6, // suspended to move to world
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum ScriptPointer {
    ActivePlayer = 0,
    ActivePlayer2 = 1,
    ProtectedActivePlayer = 2,
    ProtectedActivePlayer2 = 3,
    ActiveNpc = 4,
    ActiveNpc2 = 5,
    ActiveLoc = 6,
    ActiveLoc2 = 7,
    ActiveObj = 8,
    ActiveObj2 = 9,
    Last = 10,
}

impl From<i32> for ScriptPointer {
    fn from(value: i32) -> Self {
        match value {
            0 => ScriptPointer::ActivePlayer,
            1 => ScriptPointer::ActivePlayer2,
            2 => ScriptPointer::ProtectedActivePlayer,
            3 => ScriptPointer::ProtectedActivePlayer2,
            4 => ScriptPointer::ActiveNpc,
            5 => ScriptPointer::ActiveNpc2,
            6 => ScriptPointer::ActiveLoc,
            7 => ScriptPointer::ActiveLoc2,
            8 => ScriptPointer::ActiveObj,
            9 => ScriptPointer::ActiveObj2,
            10 => ScriptPointer::Last,
            _ => panic!("Pointer not found for from value of: {}", value),
        }
    }
}

pub struct ScriptState<'script> {
    pub script: &'script ScriptFile,
    pub execution_state: ScriptExecutionState,
    pub pc: i32,      // program counter
    pub opcount: i64, // number of opcodes executed
    frame_stack: Vec<GoSubFrame<'script>>,
    pub fp: usize, // frame pointer
    pub int_stack: Vec<i32>,
    pub isp: usize, // integer stack pointer
    pub string_stack: Vec<String>,
    pub ssp: usize, // string stack pointer
    pub int_locals: Vec<i32>,
    pub string_locals: Vec<String>,
    pointers: i32, // state pointers
}

impl<'script> ScriptState<'script> {
    /// Creates a new `ScriptState` instance with specified `i32` and `String` arguments.
    ///
    /// This method initializes a `ScriptState` for a given `ScriptFile` and sets up local
    /// variables based on the provided `i32` and `String` arguments. It allocates space for
    /// local variables and populates them with the arguments supplied.
    ///
    /// # Parameters
    ///
    /// - `script`: A reference to the `ScriptFile` to execute.
    /// - `int_args`: A vector of `i32` representing integer arguments for the script.
    /// - `string_args`: A vector of `String` representing string arguments for the script.
    ///
    /// # Return
    ///
    /// Returns a new instance of `ScriptState` initialized with the provided arguments.
    ///
    /// # Safety
    ///
    /// This function assumes that the `ScriptFile` provided has valid local counts for
    /// integer and string variables. If the `script` is improperly initialized, it may cause
    /// undefined behavior.
    #[rustfmt::skip]
    pub fn new_with_args(
        script: &ScriptFile,
        int_args: Vec<i32>,
        string_args: Vec<String>,
    ) -> ScriptState {
        let mut int_locals: Vec<i32> = vec![0; script.int_local_count as usize];
        let mut string_locals: Vec<String> = vec![String::new(); script.string_local_count as usize];

        int_locals[..int_args.len()].copy_from_slice(&int_args);
        string_locals[..string_args.len()].clone_from_slice(&string_args);

        return ScriptState {
            script,
            execution_state: ScriptExecutionState::Running,
            pc: -1,
            opcount: 0,
            frame_stack: Vec::with_capacity(50),
            fp: 0,
            int_stack: vec![0; 1000],
            isp: 0,
            string_stack: vec![String::new(); 1000],
            ssp: 0,
            int_locals,
            string_locals,
            pointers: 0,
        }
    }

    pub fn mock(file: &ScriptFile) -> ScriptState {
        return ScriptState {
            script: file,
            execution_state: ScriptExecutionState::Running,
            pc: -1,
            opcount: 0,
            frame_stack: Vec::with_capacity(5),
            fp: 0,
            int_stack: vec![0; 100],
            isp: 0,
            string_stack: vec![String::new(); 100],
            ssp: 0,
            int_locals: Vec::new(),
            string_locals: Vec::new(),
            pointers: 0,
        };
    }

    pub fn execute(&mut self, runner: &'script impl ScriptRunner, benchmark: bool) {
        self.execution_state = ScriptExecutionState::Running;

        let start: Instant = Instant::now();
        while self.execution_state == ScriptExecutionState::Running {
            match &self.script.codes {
                None => panic!("Invalid script did not contain any codes to run!"),
                Some(codes) => {
                    let pc: i32 = self.pc;
                    let length: i32 = codes.len() as i32;
                    if pc >= length || pc < -1 {
                        panic!("Invalid program counter: {}, max expected: {}", pc, length);
                    }

                    if self.opcount > 500_000 {
                        panic!("Too many instructions! {}", self.script.info.name);
                    }

                    self.opcount += 1;
                    self.pc += 1;

                    if let Some(codes) = &self.script.codes {
                        if let Some(code) = &codes[self.pc as usize] {
                            runner.push_script(code, self);
                            // runner(code, self, provider);
                        }
                    }
                }
            }
        }
        if !benchmark {
            println!("Executed script in: {:?}", start.elapsed());
        }
    }

    // ---- ints

    #[inline(always)]
    pub fn int_operand(&self) -> i32 {
        return self.script.int_operands[self.pc as usize];
    }

    /// Pushes an `i32` value onto the integer stack.
    ///
    /// This method places the provided integer value at the current integer stack pointer
    /// and then increments the pointer. It assumes that there is sufficient space on the stack.
    /// When a `ScriptState` is created, it allocates the string stack with a capacity of 1000.
    ///
    /// # Parameters
    ///
    /// - `value`: The `i32` value to push onto the integer stack.
    #[inline(always)]
    pub fn push_int(&mut self, value: i32) {
        self.int_stack[self.isp] = value;
        self.isp += 1;
    }

    /// Pops the top integer value from the integer stack.
    ///
    /// This method decreases the integer stack pointer and retrieves the integer at the new top
    /// of the stack. The caller should ensure there are values on the stack.
    ///
    /// # Return
    ///
    /// Returns the top integer value as an `i32`.
    #[inline(always)]
    pub fn pop_int(&mut self) -> i32 {
        self.isp -= 1;
        return self.int_stack[self.isp];
    }

    // ---- strings

    #[inline(always)]
    pub fn string_operand(&self) -> String {
        return self.script.string_operands[self.pc as usize].clone();
    }

    /// Pushes a `String` value onto the string stack.
    ///
    /// This method places the provided string value at the current string stack pointer
    /// and then increments the pointer. It assumes that there is sufficient space on the stack.
    /// When a `ScriptState` is created, it allocates the string stack with a capacity of 1000.
    ///
    /// # Parameters
    ///
    /// - `value`: The `String` value to push onto the string stack.
    #[inline(always)]
    pub fn push_string(&mut self, value: String) {
        self.string_stack[self.ssp] = value;
        self.ssp += 1;
    }

    /// Pops the top string value from the string stack.
    ///
    /// This method decreases the string stack pointer and retrieves the string at the new top
    /// of the stack. IThe caller should ensure there are values on the stack.
    ///
    /// # Return
    ///
    /// Returns the top string value as a `String`.
    #[inline(always)]
    pub fn pop_string(&mut self) -> String {
        self.ssp -= 1;
        return self.string_stack[self.ssp].clone();
    }

    // ---- frames

    /// Pops the most recent frame from the frame stack and restores its state.
    ///
    /// This method retrieves the last `GoSubFrame` from the stack, updates the script execution
    /// context (including the program counter and local variables) back to that frame's state,
    /// and decreases the frame pointer accordingly. It assumes that there is at least one frame
    /// to pop.
    #[inline(always)]
    pub fn pop_frame(&mut self) {
        let frame: GoSubFrame = self.frame_stack.pop().unwrap();
        self.fp -= 1;
        self.script = frame.script;
        self.pc = frame.pc;
        self.int_locals = frame.int_locals;
        self.string_locals = frame.string_locals;
    }

    /// Pushes a new frame onto the frame stack for the given script.
    ///
    /// This method saves the current execution context (including the script, program counter,
    /// and local variables) into a new `GoSubFrame`, pushes it onto the frame stack, and then
    /// sets the program counter to -1 to prepare for the new frame's execution. It also populates
    /// local variables with values from the integer and string stacks based on the argument counts
    /// specified in the `ScriptFile`.
    ///
    /// # Parameters
    ///
    /// - `script`: A reference to the `ScriptFile` to execute in the new frame.
    #[inline(always)]
    pub fn push_frame(&mut self, script: &'script ScriptFile) {
        self.frame_stack.push(GoSubFrame {
            script: self.script,
            pc: self.pc,
            int_locals: self.int_locals.clone(),
            string_locals: self.string_locals.clone(),
        });

        self.fp += 1;
        self.pc = -1;

        for i in (0..script.int_arg_count as usize).rev() {
            self.int_locals[i] = self.pop_int();
        }

        for i in (0..script.string_arg_count as usize).rev() {
            self.string_locals[i] = self.pop_string();
        }

        self.script = script;
    }

    // ---- pointers

    /// Adds a specified pointer to the internal pointer state.
    ///
    /// This method modifies the internal state by setting the bit corresponding
    /// to the provided `pointer`. It effectively adds the pointer to the current
    /// set of pointers tracked by the instance.
    ///
    /// # Parameters
    ///
    /// - `pointer`: A `ScriptPointer` representing the pointer to be added.
    ///
    /// # Return
    ///
    /// This function does not return any value. It modifies the internal state in place.
    ///
    /// # Panics
    ///
    /// This function does not panic under normal operation.
    ///
    /// # Safety
    ///
    /// There are no specific safety concerns for this function. It is safe as long as
    /// the provided pointer is a valid variant of `ScriptPointer`.
    ///
    /// # Side Effects
    ///
    /// - Updates the internal `pointers` state to include the new pointer.
    #[inline(always)]
    pub fn pointer_add(&mut self, pointer: ScriptPointer) {
        self.pointers |= 1 << pointer as i32;
    }

    /// Removes a specified pointer from the internal pointer state.
    ///
    /// This method modifies the internal state by clearing the bit corresponding
    /// to the provided `pointer`. It effectively removes the pointer from the current
    /// set of pointers tracked by the instance.
    ///
    /// # Parameters
    ///
    /// - `pointer`: A `ScriptPointer` representing the pointer to be removed.
    ///
    /// # Return
    ///
    /// This function does not return any value. It modifies the internal state in place.
    ///
    /// # Panics
    ///
    /// This function does not panic under normal operation.
    ///
    /// # Safety
    ///
    /// There are no specific safety concerns for this function. It is safe as long as
    /// the provided pointer is a valid variant of `ScriptPointer`.
    ///
    /// # Side Effects
    ///
    /// - Updates the internal `pointers` state to exclude the specified pointer.
    #[inline(always)]
    pub fn pointer_remove(&mut self, pointer: ScriptPointer) {
        self.pointers &= !(1 << pointer as i32);
    }

    /// Checks whether a specified pointer is currently set in the internal state.
    ///
    /// This method examines the internal pointer state and returns a boolean indicating
    /// whether the specified `pointer` is currently active (set).
    ///
    /// # Parameters
    ///
    /// - `pointer`: A `ScriptPointer` representing the pointer to check.
    ///
    /// # Return
    ///
    /// Returns `true` if the pointer is currently set, and `false` otherwise.
    ///
    /// # Panics
    ///
    /// This function does not panic under normal operation.
    ///
    /// # Safety
    ///
    /// There are no specific safety concerns for this function. It is safe as long as
    /// the provided pointer is a valid variant of `ScriptPointer`.
    ///
    /// # Side Effects
    ///
    /// - This function does not modify the internal state.
    #[inline(always)]
    pub fn pointer_get(&self, pointer: ScriptPointer) -> bool {
        return (self.pointers & (1 << pointer as i32)) != 0;
    }

    /// Validates the presence of required pointers in the internal state.
    ///
    /// This method checks if all specified pointers are currently set in the internal
    /// state. If any required pointer is missing, the function will panic with a detailed
    /// error message that includes both the required and current pointer states.
    ///
    /// # Parameters
    ///
    /// - `pointers`: A slice of `ScriptPointer` representing the pointers to check for presence.
    ///
    /// # Return
    ///
    /// This function does not return any value. It modifies control flow by panicking if
    /// any required pointer is missing.
    ///
    /// # Panics
    ///
    /// Panics if any of the specified pointers are not set in the internal state.
    ///
    /// # Safety
    ///
    /// There are no specific safety concerns for this function. It is safe as long as the
    /// provided pointers are valid variants of `ScriptPointer`.
    ///
    /// # Side Effects
    ///
    /// - This function may panic and terminate the execution if required pointers are not found.
    #[inline(always)]
    pub fn pointer_check(&self, pointers: &[ScriptPointer]) -> bool {
        return pointers.iter().all(|&pointer| {
            let flag: i32 = 1 << pointer as i32;
            self.pointers & flag == flag
        });
    }

    /// Converts the specified flags into a readable string representation of active pointers.
    ///
    /// This method takes a set of pointer flags and generates a string listing the
    /// corresponding `ScriptPointer` variants that are active in the given flags.
    ///
    /// # Parameters
    ///
    /// - `flags`: An `i32` representing the bit flags of pointers to convert to a string.
    ///
    /// # Return
    ///
    /// Returns a `String` that lists the active `ScriptPointer` variants corresponding
    /// to the provided flags. The string is formatted as a comma-separated list.
    ///
    /// # Panics
    ///
    /// This function does not panic under normal operation.
    ///
    /// # Safety
    ///
    /// There are no specific safety concerns for this function. It is safe as long as
    /// the provided flags represent valid states for the `ScriptPointer` variants.
    ///
    /// # Side Effects
    ///
    /// - This function does not modify any state. It only reads the internal state and generates
    ///   a string representation of the provided flags.
    fn pointer_print(&self, flags: i32) -> String {
        let mut text = String::new();
        for i in 0..(ScriptPointer::Last as usize) {
            let flag: i32 = 1 << i;
            if flags & flag != 0 {
                text.push_str(&format!("{:?}, ", ScriptPointer::from(i as i32)));
            }
        }
        if !text.is_empty() {
            text.truncate(text.len() - 2); // Remove last ", "
        }
        return text;
    }

    // testing purposes.
    pub fn pointer_debug(&self) -> String {
        return self.pointer_print(self.pointers);
    }
}

pub trait ScriptRunner: ScriptEngine {
    fn push_script<'script>(&'script self, code: &ScriptOpcode, state: &mut ScriptState<'script>);
}

/// It is important to note that these are not commands.
/// This is specifically for interfacing commands<->engine.
pub trait ScriptEngine {
    fn pop_obj(&self, id: i32) -> Result<&ObjType, String>;
    fn pop_script(&self, id: i32) -> Result<&ScriptFile, String>;
    fn line_of_sight(&self, from: i32, to: i32) -> bool;
    fn add_obj(&self, coord: i32, id: i32, count: i32, duration: i32) -> bool;
}
