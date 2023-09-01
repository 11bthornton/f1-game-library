use crate::telemetry_data::packet_header::PacketHeader;
use bincode::deserialize;
use serde::{
    Deserialize,
    Serialize,
};
use std::{
    str,
    str::Utf8Error,
};

#[derive(Deserialize, Debug, Serialize, Default, Clone, Copy)]
#[repr(C)]

pub struct PacketEventData {
    pub m_header: PacketHeader,
    pub event_string_code: [u8; 4],
    pub remaining_data: [u8; 16],
}

macro_rules! decode {
    ($($name:expr => ($ty:ty,  $var:ident)),+) => {
            impl PacketEventData {
                // #[ignore(unreachable_code)]
                pub fn decode(self) -> Result<PacketEventFinal, Utf8Error> {

                        let button_code = str::from_utf8(&self.event_string_code)?;

                        match button_code {
                            $(
                                $name => {

                                        let decoded: $ty = deserialize(&self.remaining_data).unwrap();

                                         return Ok(
                                            PacketEventFinal {
                                                m_header: self.m_header,
                                                event_string_code: self.event_string_code,
                                                r#type: EventType::$var(decoded)
                                            }
                                        )


                                },
                            )+

                            _ => panic!(
                                "{}", "Unrecognised button code! {button_code}"
                            )
                        }

                }
            }
    };
}

decode!(

    "BUTN" => (Buttons, Buttons),
    "RCWN" => (RaceWinner, RaceWinner),
    "FLBK" => (FlashBack, FlashBack),
    "TMPT" => (TeamMateInPits, TeamMateInPits),
    "RCWM" => (RaceWinner, RaceWinner),
    "RTMT" => (Retirement, Retirement),
    "FTLP" => (FastestLap, FastestLap),
    "STLG" => (StartLights, StartLights),
    "SPTP" => (SpeedTrap, SpeedTrap),
    "PENA" => (Penalty, Penalty),
    "DTSV" => (DriveThroughPenaltyServed, DriveThroughPenaltyServed),
    "SGSV" => (StopGoPenaltyServed, StopGoPenaltyServed),
    "LGOT" => (LightsOut, LightsOut),
    "SSTA" => (SessionStart, SessionStart),
    "SEND" => (SessionEnd, SessionEnd),
    "CHQF" => (ChequeredFlag, ChequeredFlag),
    "DRSE" => (DrsEnabled, DrsEnabled),
    "DRSD" => (DrsDisabled, DrsDisabled),
    "OVTK" => (Overtake, Overtake),
    "RDFL" => (RedFlag, RedFlag)

);

#[derive(Deserialize, Debug, Serialize, Default, Clone, Copy)]
#[repr(C)]
pub struct PacketEventFinal {
    pub m_header: PacketHeader,
    pub event_string_code: [u8; 4],
    pub r#type: EventType,
}
#[derive(Deserialize, Debug, Serialize, Default, Clone, Copy)]
#[repr(C)]

pub struct Overtake {
    pub overtaking_vehicle_idx: u8,
    pub overtaken_vehicle_idx: u8,
}

#[derive(Deserialize, Debug, Serialize, Default, Clone, Copy)]
#[repr(C)]

pub struct RedFlag;

#[derive(Deserialize, Debug, Serialize, Clone, Copy)]
#[repr(C)]

pub struct Buttons {
    button_status: u32,
}
#[derive(Deserialize, Debug, Serialize, Clone, Copy)]
#[repr(C)]

pub struct FastestLap {
    pub vehicle_index: u8,
    lap_time: f32,
}
#[derive(Deserialize, Debug, Serialize, Clone, Copy)]
#[repr(C)]

pub struct Retirement {
    vehicle_index: u8,
}
#[derive(Deserialize, Debug, Serialize, Clone, Copy)]
#[repr(C)]

pub struct TeamMateInPits {
    vehicle_index: u8,
}
#[derive(Deserialize, Debug, Serialize, Clone, Copy)]
#[repr(C)]

pub struct RaceWinner {
    vehicle_index: u8,
}
#[derive(Deserialize, Debug, Serialize, Clone, Copy)]
#[repr(C)]

pub struct Penalty {
    pub penalty_type: PenaltyType,
    pub infringement_type: InfringementType,
    pub vehicle_index: u8,
    pub time: u8,
    pub lap_number: u8,
    pub places_gained: u8,
}

use serde_repr::{
    Deserialize_repr,
    Serialize_repr,
};

#[derive(Serialize, Deserialize_repr, Debug, Clone, Copy)]
#[repr(u8)]
pub enum PenaltyType {
    #[serde(rename = "Drive Through")]
    DriveThrough = 0,
    #[serde(rename = "Stop Go")]
    StopGo = 1,
    #[serde(rename = "Grid Penalty")]
    GridPenalty = 2,
    #[serde(rename = "Penalty Reminder")]
    PenaltyReminder = 3,
    #[serde(rename = "Time Penalty")]
    TimePenalty = 4,
    #[serde(rename = "Warning")]
    Warning = 5,
    #[serde(rename = "Disqualified")]
    Disqualified = 6,
    #[serde(rename = "Removed From Formation Lap")]
    RemovedFromFormationLap = 7,
    #[serde(rename = "Parked Too Long Timer")]
    ParkedTooLongTimer = 8,
    #[serde(rename = "Tyre Regulations")]
    TyreRegulations = 9,
    #[serde(rename = "This Lap Invalidated")]
    ThisLapInvalidated = 10,
    #[serde(rename = "This And Next Lap Invalidated")]
    ThisAndNextLapInvalidated = 11,
    #[serde(rename = "This Lap Invalidated Without Reason")]
    ThisLapInvalidatedWithoutReason = 12,
    #[serde(rename = "This And Next Lap Invalidated Without Reason")]
    ThisAndNextLapInvalidatedWithoutReason = 13,
    #[serde(rename = "This And Previous Lap Invalidated")]
    ThisAndPreviousLapInvalidated = 14,
    #[serde(rename = "This And Previous Lap Invalidated Without Reason")]
    ThisAndPreviousLapInvalidatedWithoutReason = 15,
    #[serde(rename = "Retired")]
    Retired = 16,
    #[serde(rename = "Black Flag Timer")]
    BlackFlagTimer = 17,
}
#[derive(Serialize, Deserialize_repr, Debug, Clone, Copy)]
#[repr(u8)]
pub enum InfringementType {
    #[serde(rename = "Blocking by Slow Driving")]
    SlowDrivingBlock = 0,

    #[serde(rename = "Blocking by Wrong Way Driving")]
    WrongWayBlock = 1,

    #[serde(rename = "Reversing Off the Start Line")]
    ReverseStart = 2,

    #[serde(rename = "Big Collision")]
    MajorCollision = 3,

    #[serde(rename = "Small Collision")]
    MinorCollision = 4,

    #[serde(rename = "Collision: Failed to Hand Back Position (Single)")]
    FailedReturnSingle = 5,

    #[serde(rename = "Collision: Failed to Hand Back Position (Multiple)")]
    FailedReturnMultiple = 6,

    #[serde(rename = "Corner Cutting: Gained Time")]
    TimeGainCornerCut = 7,

    #[serde(rename = "Corner Cutting: Overtake (Single)")]
    OvertakeCutSingle = 8,

    #[serde(rename = "Corner Cutting: Overtake (Multiple)")]
    OvertakeCutMultiple = 9,

    #[serde(rename = "Crossed Pit Exit Lane")]
    PitExitViolation = 10,

    #[serde(rename = "Ignoring Blue Flags")]
    IgnoredBlueFlags = 11,

    #[serde(rename = "Ignoring Yellow Flags")]
    IgnoredYellowFlags = 12,

    #[serde(rename = "Ignoring Drive Through")]
    IgnoredDriveThrough = 13,

    #[serde(rename = "Too Many Drive Throughs")]
    ExcessiveDriveThroughs = 14,

    #[serde(rename = "Drive Through Reminder: Serve Within N Laps")]
    DriveThroughReminderNLaps = 15,

    #[serde(rename = "Drive Through Reminder: Serve This Lap")]
    DriveThroughReminderThisLap = 16,

    #[serde(rename = "Pit Lane Speeding")]
    PitSpeeding = 17,

    #[serde(rename = "Parked for Too Long")]
    ExcessiveParking = 18,

    #[serde(rename = "Ignoring Tyre Regulations")]
    TyreRulesIgnored = 19,

    #[serde(rename = "Too Many Penalties")]
    ExcessivePenalties = 20,

    #[serde(rename = "Multiple Warnings")]
    MultipleWarnings = 21,

    #[serde(rename = "Approaching Disqualification")]
    NearDisqualification = 22,

    #[serde(rename = "Tyre Regulations: Select (Single)")]
    SingleTyreViolation = 23,

    #[serde(rename = "Tyre Regulations: Select (Multiple)")]
    MultipleTyreViolation = 24,

    #[serde(rename = "Lap Invalidated: Corner Cutting")]
    InvalidLapCornerCut = 25,

    #[serde(rename = "Lap Invalidated: Running Wide")]
    InvalidLapRanWide = 26,

    #[serde(rename = "Corner Cutting: Ran Wide, Gained Time (Minor)")]
    MinorTimeGainRanWide = 27,

    #[serde(rename = "Corner Cutting: Ran Wide, Gained Time (Significant)")]
    SignificantTimeGainRanWide = 28,

    #[serde(rename = "Corner Cutting: Ran Wide, Gained Time (Extreme)")]
    ExtremeTimeGainRanWide = 29,

    #[serde(rename = "Lap Invalidated: Wall Riding")]
    InvalidLapWallRide = 30,

    #[serde(rename = "Lap Invalidated: Flashback Used")]
    InvalidLapFlashback = 31,

    #[serde(rename = "Lap Invalidated: Reset to Track")]
    InvalidLapReset = 32,

    #[serde(rename = "Blocking the Pitlane")]
    PitlaneBlock = 33,

    #[serde(rename = "Jump Start")]
    JumpStart = 34,

    #[serde(rename = "Safety Car to Car Collision")]
    SafetyCarCollision = 35,

    #[serde(rename = "Safety Car: Illegal Overtake")]
    SafetyCarOvertake = 36,

    #[serde(rename = "Safety Car: Exceeding Allowed Pace")]
    SafetyCarPaceExceed = 37,

    #[serde(rename = "Virtual Safety Car: Exceeding Allowed Pace")]
    VSCarPaceExceed = 38,

    #[serde(rename = "Formation Lap: Below Allowed Speed")]
    SlowFormationLap = 39,

    #[serde(rename = "Formation Lap: Parking")]
    FormationLapParking = 40,

    #[serde(rename = "Retired: Mechanical Failure")]
    MechanicalRetirement = 41,

    #[serde(rename = "Retired: Terminally Damaged")]
    DamageRetirement = 42,

    #[serde(rename = "Safety Car: Falling Too Far Back")]
    SafetyCarGapLarge = 43,

    #[serde(rename = "Black Flag Timer")]
    BlackFlag = 44,

    #[serde(rename = "Unserved Stop Go Penalty")]
    UnservedStopGo = 45,

    #[serde(rename = "Unserved Drive Through Penalty")]
    UnservedDriveThrough = 46,

    #[serde(rename = "Engine Component Change")]
    EngineChange = 47,

    #[serde(rename = "Gearbox Change")]
    GearboxChange = 48,

    #[serde(rename = "Parc Fermé Change")]
    ParcFerméEdit = 49,

    #[serde(rename = "League Grid Penalty")]
    LeaguePenalty = 50,

    #[serde(rename = "Retry Penalty")]
    RetryPenalty = 51,

    #[serde(rename = "Illegal Time Gain")]
    IllegalTime = 52,

    #[serde(rename = "Mandatory Pitstop")]
    MandatoryPit = 53,

    #[serde(rename = "Attribute Assigned")]
    Attribute = 54,
}

#[derive(Deserialize, Debug, Serialize, Clone, Copy)]
#[repr(C)]

pub struct SpeedTrap {
    pub vehicle_index: u8,
    pub speed: f32,
    pub overall_fastest_in_session: u8,
    pub driver_fastest_in_session: u8,
}
#[derive(Deserialize, Debug, Serialize, Clone, Copy)]
#[repr(C)]

pub struct StartLights {
    pub num_lights: u8,
}
#[derive(Deserialize, Debug, Serialize, Clone, Copy)]
#[repr(C)]

pub struct DriveThroughPenaltyServed {
    pub vehicle_index: u8,
}
#[derive(Deserialize, Debug, Serialize, Clone, Copy)]
#[repr(C)]

pub struct StopGoPenaltyServed {
    vehicle_index: u8,
}
#[derive(Deserialize, Debug, Serialize, Clone, Copy)]
#[repr(C)]

pub struct FlashBack {
    pub flashback_frame_identifier: u32,
    pub flashback_session_time: f32,
}

#[derive(Deserialize, Debug, Serialize, Default, Clone, Copy)]
#[repr(C)]

pub enum EventType {
    Buttons(Buttons),
    FastestLap(FastestLap),
    Retirement(Retirement),
    TeamMateInPits(TeamMateInPits),
    RaceWinner(RaceWinner),
    Penalty(Penalty),
    SpeedTrap(SpeedTrap),
    StartLights(StartLights),
    DriveThroughPenaltyServed(DriveThroughPenaltyServed),
    StopGoPenaltyServed(StopGoPenaltyServed),
    FlashBack(FlashBack),
    LightsOut(LightsOut),
    SessionStart(SessionStart),
    SessionEnd(SessionEnd),
    DrsEnabled(DrsEnabled),
    DrsDisabled(DrsDisabled),
    ChequeredFlag(ChequeredFlag),
    AlternateSpeedTrap(AlternateSpeedTrap),
    Overtake(Overtake),
    RedFlag(RedFlag),
    #[default]
    NoEvent,
}
#[derive(Deserialize, Debug, Serialize, Default, Clone, Copy)]
#[repr(C)]

pub struct LightsOut;
#[derive(Deserialize, Debug, Serialize, Default, Clone, Copy)]
#[repr(C)]

pub struct SessionStart;
#[derive(Deserialize, Debug, Serialize, Default, Clone, Copy)]
#[repr(C)]

pub struct SessionEnd;

#[derive(Deserialize, Debug, Serialize, Default, Clone, Copy)]
#[repr(C)]

pub struct DrsEnabled;
#[derive(Deserialize, Debug, Serialize, Default, Clone, Copy)]
#[repr(C)]

pub struct DrsDisabled;
#[derive(Deserialize, Debug, Serialize, Default, Clone, Copy)]
#[repr(C)]

pub struct ChequeredFlag;
#[derive(Deserialize, Debug, Serialize, Default, Clone, Copy)]
#[repr(C)]

pub struct AlternateSpeedTrap;
