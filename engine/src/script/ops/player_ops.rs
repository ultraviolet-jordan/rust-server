use cache::{ScriptEngine, ScriptOpcode, ScriptState};

pub struct PlayerOps;

impl PlayerOps {
    pub fn new() -> PlayerOps {
        return PlayerOps;
    }

    pub fn push(
        &self,
        engine: &impl ScriptEngine,
        state: &mut ScriptState,
        code: &ScriptOpcode,
    ) -> Result<(), String> {
        match code {
            ScriptOpcode::AllowDesign => Err("Not implemented".to_string()),
            ScriptOpcode::Anim => state.protect(&ScriptState::ACTIVE_PLAYER, |state| {
                self.anim(engine, state)
            }),
            ScriptOpcode::BasReadyAnim => self.bas_readyanim(engine, state),
            ScriptOpcode::BasRunning => Err("Not implemented".to_string()),
            ScriptOpcode::BasTurnOnSpot => Err("Not implemented".to_string()),
            ScriptOpcode::BasWalkB => Err("Not implemented".to_string()),
            ScriptOpcode::BasWalkF => Err("Not implemented".to_string()),
            ScriptOpcode::BasWalkL => Err("Not implemented".to_string()),
            ScriptOpcode::BasWalkR => Err("Not implemented".to_string()),
            ScriptOpcode::BufferFull => Err("Not implemented".to_string()),
            ScriptOpcode::BuildAppearance => Err("Not implemented".to_string()),
            ScriptOpcode::Busy => Err("Not implemented".to_string()),
            ScriptOpcode::CamLookAt => Err("Not implemented".to_string()),
            ScriptOpcode::CamMoveTo => Err("Not implemented".to_string()),
            ScriptOpcode::CamReset => Err("Not implemented".to_string()),
            ScriptOpcode::CamShake => Err("Not implemented".to_string()),
            ScriptOpcode::ClearQueue => Err("Not implemented".to_string()),
            ScriptOpcode::ClearSoftTimer => Err("Not implemented".to_string()),
            ScriptOpcode::ClearTimer => Err("Not implemented".to_string()),
            ScriptOpcode::GetTimer => Err("Not implemented".to_string()),
            ScriptOpcode::Coord => Err("Not implemented".to_string()),
            ScriptOpcode::Damage => Err("Not implemented".to_string()),
            ScriptOpcode::Displayname => Err("Not implemented".to_string()),
            ScriptOpcode::FaceSquare => Err("Not implemented".to_string()),
            ScriptOpcode::FindUid => self.find_uid(engine, state),
            ScriptOpcode::Gender => Err("Not implemented".to_string()),
            ScriptOpcode::GetQueue => Err("Not implemented".to_string()),
            ScriptOpcode::StatAdvance => Err("Not implemented".to_string()),
            ScriptOpcode::HeadiconsGet => Err("Not implemented".to_string()),
            ScriptOpcode::HeadiconsSet => Err("Not implemented".to_string()),
            ScriptOpcode::HealEnergy => Err("Not implemented".to_string()),
            ScriptOpcode::HintCoord => Err("Not implemented".to_string()),
            ScriptOpcode::HintNpc => Err("Not implemented".to_string()),
            ScriptOpcode::HintPlayer => Err("Not implemented".to_string()),
            ScriptOpcode::HintStop => Err("Not implemented".to_string()),
            ScriptOpcode::IfClose => Err("Not implemented".to_string()),
            ScriptOpcode::TutClose => Err("Not implemented".to_string()),
            ScriptOpcode::IfMultiZone => Err("Not implemented".to_string()),
            ScriptOpcode::IfOpenChat => Err("Not implemented".to_string()),
            ScriptOpcode::TutOpen => Err("Not implemented".to_string()),
            ScriptOpcode::IfOpenMain => Err("Not implemented".to_string()),
            ScriptOpcode::IfOpenMainSide => Err("Not implemented".to_string()),
            ScriptOpcode::IfOpenSide => Err("Not implemented".to_string()),
            ScriptOpcode::IfSetAnim => Err("Not implemented".to_string()),
            ScriptOpcode::IfSetColour => Err("Not implemented".to_string()),
            ScriptOpcode::IfSetHide => Err("Not implemented".to_string()),
            ScriptOpcode::IfSetModel => Err("Not implemented".to_string()),
            ScriptOpcode::IfSetRecol => Err("Not implemented".to_string()),
            ScriptOpcode::IfSetNpcHead => Err("Not implemented".to_string()),
            ScriptOpcode::IfSetObject => Err("Not implemented".to_string()),
            ScriptOpcode::IfSetPlayerHead => Err("Not implemented".to_string()),
            ScriptOpcode::IfSetPosition => Err("Not implemented".to_string()),
            ScriptOpcode::IfSetResumeButtons => Err("Not implemented".to_string()),
            ScriptOpcode::IfSetTab => Err("Not implemented".to_string()),
            ScriptOpcode::IfSetTabActive => Err("Not implemented".to_string()),
            ScriptOpcode::TutFlash => Err("Not implemented".to_string()),
            ScriptOpcode::IfSetText => Err("Not implemented".to_string()),
            ScriptOpcode::LastLoginInfo => Err("Not implemented".to_string()),
            ScriptOpcode::LastCom => Err("Not implemented".to_string()),
            ScriptOpcode::LastInt => Err("Not implemented".to_string()),
            ScriptOpcode::LastItem => Err("Not implemented".to_string()),
            ScriptOpcode::LastSlot => Err("Not implemented".to_string()),
            ScriptOpcode::LastTargetSlot => Err("Not implemented".to_string()),
            ScriptOpcode::LastUseItem => Err("Not implemented".to_string()),
            ScriptOpcode::LastUseSlot => Err("Not implemented".to_string()),
            ScriptOpcode::LongQueue => Err("Not implemented".to_string()),
            ScriptOpcode::Mes => {
                state.pop_string();
                //println!("{}", state.pop_string());
                return Ok(());
            }
            ScriptOpcode::MidiJingle => Err("Not implemented".to_string()),
            ScriptOpcode::MidiSong => Err("Not implemented".to_string()),
            ScriptOpcode::Name => Err("Not implemented".to_string()),
            ScriptOpcode::PApRange => Err("Not implemented".to_string()),
            ScriptOpcode::PArriveDelay => Err("Not implemented".to_string()),
            ScriptOpcode::PCountDialog => Err("Not implemented".to_string()),
            ScriptOpcode::PDelay => Err("Not implemented".to_string()),
            ScriptOpcode::PExactMove => Err("Not implemented".to_string()),
            ScriptOpcode::PFindUid => Err("Not implemented".to_string()),
            ScriptOpcode::PLocMerge => Err("Not implemented".to_string()),
            ScriptOpcode::PLogout => Err("Not implemented".to_string()),
            ScriptOpcode::POpHeld => Err("Not implemented".to_string()),
            ScriptOpcode::POpLoc => Err("Not implemented".to_string()),
            ScriptOpcode::POpNpc => Err("Not implemented".to_string()),
            ScriptOpcode::POpNpcT => Err("Not implemented".to_string()),
            ScriptOpcode::POpObj => Err("Not implemented".to_string()),
            ScriptOpcode::POpPlayer => Err("Not implemented".to_string()),
            ScriptOpcode::POpPlayerT => Err("Not implemented".to_string()),
            ScriptOpcode::PPauseButton => Err("Not implemented".to_string()),
            ScriptOpcode::PStopAction => Err("Not implemented".to_string()),
            ScriptOpcode::PTeleJump => Err("Not implemented".to_string()),
            ScriptOpcode::PTeleport => Err("Not implemented".to_string()),
            ScriptOpcode::PWalk => Err("Not implemented".to_string()),
            ScriptOpcode::PlayerFindAllZone => Err("Not implemented".to_string()),
            ScriptOpcode::PlayerFindNext => Err("Not implemented".to_string()),
            ScriptOpcode::Queue => Err("Not implemented".to_string()),
            ScriptOpcode::Say => Err("Not implemented".to_string()),
            ScriptOpcode::WalkTrigger => Err("Not implemented".to_string()),
            ScriptOpcode::SetTimer => Err("Not implemented".to_string()),
            ScriptOpcode::SoftTimer => Err("Not implemented".to_string()),
            ScriptOpcode::SoundSynth => Err("Not implemented".to_string()),
            ScriptOpcode::SpotAnimPl => Err("Not implemented".to_string()),
            ScriptOpcode::StaffModLevel => Err("Not implemented".to_string()),
            ScriptOpcode::Stat => Err("Not implemented".to_string()),
            ScriptOpcode::StatAdd => Err("Not implemented".to_string()),
            ScriptOpcode::StatBase => Err("Not implemented".to_string()),
            ScriptOpcode::StatHeal => Err("Not implemented".to_string()),
            ScriptOpcode::StatSub => Err("Not implemented".to_string()),
            ScriptOpcode::StrongQueue => Err("Not implemented".to_string()),
            ScriptOpcode::Uid => Err("Not implemented".to_string()),
            ScriptOpcode::WeakQueue => Err("Not implemented".to_string()),
            ScriptOpcode::IfOpenMainOverlay => Err("Not implemented".to_string()),
            ScriptOpcode::AfkEvent => Err("Not implemented".to_string()),
            ScriptOpcode::LowMemory => Err("Not implemented".to_string()),
            ScriptOpcode::SetIdkit => Err("Not implemented".to_string()),
            ScriptOpcode::PClearPendingAction => Err("Not implemented".to_string()),
            ScriptOpcode::GetWalkTrigger => Err("Not implemented".to_string()),
            ScriptOpcode::Busy2 => Err("Not implemented".to_string()),
            ScriptOpcode::FindHero => Err("Not implemented".to_string()),
            ScriptOpcode::BothHeroPoints => Err("Not implemented".to_string()),
            ScriptOpcode::SetGender => Err("Not implemented".to_string()),
            ScriptOpcode::SetSkinColour => Err("Not implemented".to_string()),
            ScriptOpcode::PAnimProtect => Err("Not implemented".to_string()),
            ScriptOpcode::RunEnergy => Err("Not implemented".to_string()),
            ScriptOpcode::Weight => Err("Not implemented".to_string()),
            ScriptOpcode::LastCoord => Err("Not implemented".to_string()),
            _ => Err(format!("Unrecognised player ops code: {:?}", code)),
        }
    }

    // https://x.com/JagexAsh/status/1806246992797921391
    #[inline(always)]
    fn anim(&self, engine: &impl ScriptEngine, state: &mut ScriptState) -> Result<(), String> {
        let delay: i32 = state.pop_int();
        let seq: i32 = state.pop_int();
        return engine.with_player_mut(state.get_active_player(), |mut player| {
            player.play_animation(seq, delay);
        });
    }

    #[rustfmt::skip]
    #[inline(always)]
    fn bas_readyanim(&self, engine: &impl ScriptEngine, state: &mut ScriptState) -> Result<(), String> {
        let seq: i32 = state.pop_int();
        return engine.with_player_mut(state.get_active_player(), |mut player| {
            player.set_bas_readyanim(seq);
        });
    }

    #[rustfmt::skip]
    #[inline(always)]
    fn find_uid(&self, engine: &impl ScriptEngine, state: &mut ScriptState) -> Result<(), String> {
        let uid: i32 = state.pop_int();
        return engine.with_player(uid, |player| {
            state.set_active_player(uid);
            state.pointer_add(ScriptState::ACTIVE_PLAYER[state.int_operand() as usize]);
            state.push_int(1);
        }).or_else(|_| {
            state.push_int(0);
            Ok(())
        });
    }
}
