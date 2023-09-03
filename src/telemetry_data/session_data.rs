use crate::telemetry_data::packet_header::PacketHeader;
use serde::{
    Deserialize,
    Serialize,
};
use serde_repr::Deserialize_repr;
#[derive(Deserialize, Debug, Serialize, Clone, Copy)]
pub struct MarshalZone {
    pub zone_start: f32,
    pub zone_flag: VehicleFiaFlags,
}

#[derive(Deserialize, Debug, Serialize, Default, Copy, Clone)]
pub struct WeatherForecastSample {
    pub session_type: SessionType,
    pub time_offset: u8, // Time in Minutes Forecast is for
    pub weather: Weather,

    pub track_temperature: i8,
    pub track_temperature_change: i8,
    pub air_temperature: i8,
    pub air_temperature_change: i8,

    pub rain_percentage: u8,
}

// #[derive(Deserialize, Debug, Serialize, Default, Copy, Clone)]
// pub struct WeatherForecastSampleStruct {
//     first_32: [WeatherForecastSample; 32],
//     last: [WeatherForecastSample; 24],
// }

#[derive(Serialize, Deserialize_repr, Debug, Clone, Copy, Default)]
#[repr(u8)]
pub enum SessionType {
    #[default]
    Unknown,
    #[serde(rename = "Practice One")]
    PracticeOne,
    #[serde(rename = "Practice Two")]
    PracticeTwo,
    #[serde(rename = "Practice Three")]
    PracticeThree,
    #[serde(rename = "Short Practice")]
    ShortPractice,
    #[serde(rename = "Qualifying One")]
    QualifyingOne,
    #[serde(rename = "Qualifying Two")]
    QualifyingTwo,
    #[serde(rename = "Qualifying Three")]
    QualifyingThree,
    #[serde(rename = "Short Qualifying")]
    ShortQualifying,
    #[serde(rename = "One Shot Qualifying")]
    OneShotQualifying,
    Race,
    RaceTwo,
    RaceThree,
    #[serde(rename = "Time Trial")]
    TimeTrial,
}

#[derive(Serialize, Deserialize_repr, Debug, Clone, Copy)]
#[repr(u8)]
pub enum Formula {
    #[serde(rename = "F1 Modern")]
    F1Modern,
    #[serde(rename = "F1 Classic")]
    F1Classic,
    #[serde(rename = "F2")]
    FormulaTwo,
    #[serde(rename = "F1 Generic")]
    F1Generic,
}

#[derive(Serialize, Deserialize_repr, Debug, Clone, Copy)]
#[repr(u8)]
pub enum SafetyCarStatus {
    #[serde(rename = "No Safety Car")]
    NoSafetyCar,
    #[serde(rename = "Full Safety Car")]
    FullSafetyCar,
    #[serde(rename = "Virtual Safety Car")]
    VirtualSafetyCar,
    #[serde(rename = "Formation Lap")]
    FormationLap,
}

#[derive(Serialize, Deserialize_repr, Debug, Clone, Copy)]
#[repr(u8)]
pub enum NetworkGame {
    Offline,
    Online,
}

#[derive(Serialize, Deserialize_repr, Debug, Clone, Copy)]
#[repr(u8)]
pub enum ForecastAccuracy {
    Approximate,
    Perfect,
}

#[derive(Serialize, Deserialize_repr, Debug, Clone, Copy)]
#[repr(u8)]
pub enum AssistToggle {
    Off,
    On,
}

#[derive(Serialize, Deserialize_repr, Debug, Clone, Copy)]
#[repr(u8)]
pub enum GearboxAssist {
    Manual,
    #[serde(rename = "Suggested Gear")]
    SuggestedGear,
    Automatic,
}

#[derive(Serialize, Deserialize_repr, Debug, Clone, Copy, Default)]
#[repr(u8)]
pub enum Weather {
    #[default]
    Clear = 0,
    LightCloud = 1,
    Overcast = 2,
    LightRain = 3,
    HeavyRain = 4,
    Storm = 5,
    Unknown = 6,
}

use super::car_status_data::VehicleFiaFlags;
use serde_big_array::BigArray;

#[derive(Deserialize, Debug, Serialize, Clone, Copy)]
pub struct PacketSessionData {
    pub header: PacketHeader, // Header

    pub weather: Weather, // Weather - 0 = clear, 1 = light cloud, 2 = overcast
    // 3 = light rain, 4 = heavy rain, 5 = storm
    pub track_temperature: i8,     // Track temp. in degrees celsius
    pub air_temperature: i8,       // Air temp. in degrees celsius
    pub total_laps: u8,            // Total number of laps in this race
    pub track_length: u16,         // Track length in metres
    pub session_type: SessionType, // 0 = unknown, 1 = P1, 2 = P2, 3 = P3, 4 = Short P
    // 5 = Q1, 6 = Q2, 7 = Q3, 8 = Short Q, 9 = OSQ
    // 10 = R, 11 = R2, 12 = R3, 13 = Time Trial
    pub track: Track,     // -1 for unknown, 0-21 for tracks, see appendix
    pub formula: Formula, // Formula, 0 = F1 Modern, 1 = F1 Classic, 2 = F2,
    // 3 = F1 Generic
    pub session_time_left: u16,       // Time left in session in seconds
    pub session_duration: u16,        // Session duration in seconds
    pub pit_speed_limit: u8,          // Pit speed limit in kilometres per hour
    pub game_paused: bool,            // Whether the game is paused
    pub is_spectating: bool,          // Whether the player is spectating
    pub spectator_car_index: u8,      // Index of the car being spectated
    pub sli_pro_native_support: bool, // SLI Pro support, 0 = inactive, 1 = active
    pub num_marshall_zones: u8,       // Number of marshal zones to follow
    pub marshall_zones: [MarshalZone; 21], // List of marshal zones – max 21
    pub safety_car_status: SafetyCarStatus, // 0 = no safety car, 1 = full
    // 2 = virtual, 3 = formation lap
    pub network_game: NetworkGame,        // 0 = offline, 1 = online
    pub num_weather_forecast_samples: u8, // Number of weather samples to follow

    #[serde(with = "BigArray")]
    pub weather_forecast_samples: [WeatherForecastSample; 56], // Array of weather forecast samples

    pub forecast_accuracy: ForecastAccuracy, // 0 = Perfect, 1 = Approximate
    pub ai_difficulty: u8,                   // AI Difficulty rating – 0-110
    pub season_link_identifier: u32,         // Identifier for season - persists across saves
    pub weekend_link_identifier: u32,        // Identifier for weekend - persists across saves
    pub session_link_identifier: u32,        // Identifier for session - persists across saves
    pub pit_stop_window_ideal_lap: u8,       // Ideal lap to pit on for current strategy (player)
    pub pit_stop_window_latest_lap: u8,      // Latest lap to pit on for current strategy (player)
    pub pit_stop_rejoin_position: u8,        // Predicted position to rejoin at (player)
    pub steering_assist: AssistToggle,       // 0 = off, 1 = on
    pub braking_assist: AssistToggle,        // 0 = off, 1 = low, 2 = medium, 3 = high
    pub gearbox_assist: GearboxAssist,       // 1 = manual, 2 = manual & suggested gear, 3 = auto
    pub pit_assist: AssistToggle,            // 0 = off, 1 = on
    pub pit_release_assist: AssistToggle,    // 0 = off, 1 = on
    pub ers_assist: AssistToggle,            // 0 = off, 1 = on
    pub drs_assist: AssistToggle,            // 0 = off, 1 = on
    pub dynamic_racing_line: DynamicRacingLine, // 0 = off, 1 = corners only, 2 = full
    pub dynamic_racing_line_type: DynamicRacingLineType, // 0 = 2D, 1 = 3D

    // added stuff
    pub speed_units_lead_player: u8,
    pub temp_units_lead_player: u8,
    pub speed_units_secondary_player: u8,
    pub temp_units_secondary_player: u8,

    pub num_safety_car_periods: u8,
    pub num_vsc_periods: u8,
    pub num_red_flag_periods: u8,
}

#[derive(Serialize, Deserialize_repr, Debug, Clone, Copy)]
#[repr(u8)]
pub enum DynamicRacingLine {
    #[serde(rename = "2d")]
    TwoD,
    #[serde(rename = "3d")]
    ThreeD,
}

#[derive(Serialize, Deserialize_repr, Debug, Clone, Copy)]
#[repr(u8)]
pub enum DynamicRacingLineType {
    Off,
    #[serde(rename = "Corners Only")]
    CornersOnly,
    Full,
}

#[derive(Serialize, Deserialize_repr, Debug, Clone, Copy)]
#[repr(i8)]
pub enum Track {
    #[serde(rename = "Melbourne, Australia")]
    Melbourne = 0,

    #[serde(rename = "Paul Ricard, France")]
    PaulRicard = 1,

    #[serde(rename = "Shanghai, China")]
    Shanghai = 2,

    #[serde(rename = "Sakhir Bahrain")]
    SakhirBahrain = 3,

    #[serde(rename = "Catalunya, Spain")]
    Catalunya = 4,

    #[serde(rename = "Monaco")]
    Monaco = 5,

    #[serde(rename = "Montreal, Canada")]
    Montreal = 6,

    #[serde(rename = "Silverstone, UK")]
    Silverstone = 7,

    #[serde(rename = "Hockenheim, Germany")]
    Hockenheim = 8,

    #[serde(rename = "Hungaroring, Hungary")]
    Hungaroring = 9,

    #[serde(rename = "Spa, Belgium")]
    Spa = 10,

    #[serde(rename = "Monza, Italy")]
    Monza = 11,

    #[serde(rename = "Singapore")]
    Singapore = 12,

    #[serde(rename = "Suzuka, Japan")]
    Suzuka = 13,

    #[serde(rename = "Abu Dhabi, UAE")]
    AbuDhabi = 14,

    #[serde(rename = "Texas, USA")]
    Texas = 15,

    #[serde(rename = "Brazil")]
    Brazil = 16,

    #[serde(rename = "Austria")]
    Austria = 17,

    #[serde(rename = "Sochi, Russia")]
    Sochi = 18,

    #[serde(rename = "Mexico City, Mexico")]
    Mexico = 19,

    #[serde(rename = "Baku Azerbaijan")]
    BakuAzerbaijan = 20,

    #[serde(rename = "Sakhir Short, Bahrain")]
    SakhirShort = 21,

    #[serde(rename = "Silverstone Short, UK")]
    SilverstoneShort = 22,

    #[serde(rename = "Texas Short, USA")]
    TexasShort = 23,

    #[serde(rename = "Suzuka Short, Japan")]
    SuzukaShort = 24,

    #[serde(rename = "Hanoi, Vietnam")]
    Hanoi = 25,

    #[serde(rename = "Zandvoort, Netherlands")]
    Zandvoort = 26,

    #[serde(rename = "Imola, Italy")]
    Imola = 27,

    #[serde(rename = "Portimao, Portugal")]
    Portimao = 28,

    #[serde(rename = "Jeddah, Saudi Arabia")]
    Jeddah = 29,

    #[serde(rename = "Miami, USA")]
    Miami = 30,

    #[serde(rename = "Las Vegas, USA")]
    LasVegas = 31,

    #[serde(rename = "Losail, Qatar")]
    Losail = 32,
}
