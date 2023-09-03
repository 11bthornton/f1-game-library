use super::lobby_info;
use crate::telemetry_data::packet_header::PacketHeader;
use serde::{
    Deserialize,
    Serialize,
};
use serde_big_array::BigArray;
use serde_repr::Deserialize_repr;
use std::str;

#[derive(Deserialize, Debug, Serialize, Copy, Clone)]
pub struct ParticipantData {
    pub ai_controlled: u8,
    pub driver_id: Driver,
    pub network_id: u8,
    pub team_id: Team,
    pub my_team: bool,
    pub race_number: u8,
    pub nationality: Nationality,

    #[serde(with = "BigArray")]
    pub name: [u8; 48],

    pub telemetry: bool,
    pub online_name: bool,
    pub platform: lobby_info::Platform,
}

#[derive(Deserialize_repr, Debug, Serialize, Copy, Clone)]
#[repr(u8)]
pub enum Driver {
    Unknown = 255,

    #[serde(rename = "Carlos Sainz")]
    CarlosSainz = 0,
    #[serde(rename = "Daniil Kvyat")]
    DaniilKvyat = 1,
    #[serde(rename = "Daniel Ricciardo")]
    DanielRicciardo = 2,

    #[serde(rename = "Fernando Alonso")]
    FernandoAlonso = 3,

    #[serde(rename = "Felipe Massa")]
    FelipeMassa = 4,
    #[serde(rename = "Kimi Raikkonen")]
    KimiRaikkonen = 6,
    #[serde(rename = "Lewis Hamilton")]
    LewisHamilton = 7,
    #[serde(rename = "Max Verstappen")]
    MaxVerstappen = 9,
    #[serde(rename = "Nico Hulkenburg")]
    NicoHulkenburg = 10,
    #[serde(rename = "Kevin Magnussen")]
    KevinMagnussen = 11,
    #[serde(rename = "Romain Grosjean")]
    RomainGrosjean = 12,
    #[serde(rename = "Sebastian Vettel")]
    SebastianVettel = 13,
    #[serde(rename = "Sergio Perez")]
    SergioPerez = 14,
    #[serde(rename = "Valtteri Bottas")]
    ValtteriBottas = 15,
    #[serde(rename = "Esteban Ocon")]
    EstebanOcon = 17,
    #[serde(rename = "Lance Stroll")]
    LanceStroll = 19,
    #[serde(rename = "Arron Barnes")]
    ArronBarnes = 20,
    #[serde(rename = "Martin Giles")]
    MartinGiles = 21,
    #[serde(rename = "Alex Murray")]
    AlexMurray = 22,
    #[serde(rename = "Lucas Roth")]
    LucasRoth = 23,
    #[serde(rename = "Igor Correia")]
    IgorCorreia = 24,
    #[serde(rename = "Sophie Levasseur")]
    SophieLevasseur = 25,
    #[serde(rename = "Jonas Schiffer")]
    JonasSchiffer = 26,
    #[serde(rename = "Alain Forest")]
    AlainForest = 27,
    #[serde(rename = "Jay Letourneau")]
    JayLetourneau = 28,
    #[serde(rename = "Esto Saari")]
    EstoSaari = 29,
    #[serde(rename = "Yasar Atiyeh")]
    YasarAtiyeh = 30,
    #[serde(rename = "Callisto Calabresi")]
    CallistoCalabresi = 31,
    #[serde(rename = "Naota Izum")]
    NaotaIzum = 32,
    #[serde(rename = "Howard Clarke")]
    HowardClarke = 33,
    #[serde(rename = "Wilheim Kaufmann")]
    WilheimKaufmann = 34,
    #[serde(rename = "Marie Laursen")]
    MarieLaursen = 35,
    #[serde(rename = "Flavio Nieves")]
    FlavioNieves = 36,
    #[serde(rename = "Peter Belousov")]
    PeterBelousov = 37,
    #[serde(rename = "Klimek Michalski")]
    KlimekMichalski = 38,
    #[serde(rename = "Santiago Moreno")]
    SantiagoMoreno = 39,
    #[serde(rename = "Benjamin Coppens")]
    BenjaminCoppens = 40,
    #[serde(rename = "Noah Visser")]
    NoahVisser = 41,
    #[serde(rename = "Gert Waldmuller")]
    GertWaldmuller = 42,
    #[serde(rename = "Julian Quesada")]
    JulianQuesada = 43,
    #[serde(rename = "Daniel Jones")]
    DanielJones = 44,
    #[serde(rename = "Artem Markelov")]
    ArtemMarkelov = 45,
    #[serde(rename = "Tadasuke Makino")]
    TadasukeMakino = 46,
    #[serde(rename = "Sean Gelael")]
    SeanGelael = 47,
    #[serde(rename = "Nyck De Vries")]
    NyckDeVries = 48,
    #[serde(rename = "Jack Aitken")]
    JackAitken = 49,
    #[serde(rename = "George Russell")]
    GeorgeRussell = 50,
    #[serde(rename = "Maximilian Gunther")]
    MaximilianGunther = 51,
    #[serde(rename = "Nirei Fukuzumi")]
    NireiFukuzumi = 52,
    #[serde(rename = "Luca Ghiotto")]
    LucaGhiotto = 53,
    #[serde(rename = "Lando Norris")]
    LandoNorris = 54,
    #[serde(rename = "Sergio Sette Camara")]
    SergioSetteCamara = 55,
    #[serde(rename = "Louis Deletraz")]
    LouisDeletraz = 56,
    #[serde(rename = "Antonio Fuoco")]
    AntonioFuoco = 57,
    #[serde(rename = "Charles Leclerc")]
    CharlesLeclerc = 58,
    #[serde(rename = "Pierre Gasly")]
    PierreGasly = 59,
    #[serde(rename = "Alexander Albon")]
    AlexanderAlbon = 62,
    #[serde(rename = "Nicholas Latifi")]
    NicholasLatifi = 63,
    #[serde(rename = "Dorian Boccolacci")]
    DorianBoccolacci = 64,
    #[serde(rename = "Niko Kari")]
    NikoKari = 65,
    #[serde(rename = "Roberto Merhi")]
    RobertoMerhi = 66,
    #[serde(rename = "Arjun Maini")]
    ArjunMaini = 67,
    #[serde(rename = "Alessio Lorandi")]
    AlessioLorandi = 68,
    #[serde(rename = "Ruben Meijer")]
    RubenMeijer = 69,
    #[serde(rename = "Rashid Nair")]
    RashidNair = 70,
    #[serde(rename = "Jack Tremblay")]
    JackTremblay = 71,
    #[serde(rename = "Devon Butler")]
    DevonButler = 72,
    #[serde(rename = "Lukas Weber")]
    LukasWeber = 73,
    #[serde(rename = "Antonio Giovinazzi")]
    AntonioGiovinazzi = 74,
    #[serde(rename = "Robert Kubica")]
    RobertKubica = 75,
    #[serde(rename = "Alain Prost")]
    AlainProst = 76,
    #[serde(rename = "Ayrton Senna")]
    AyrtonSenna = 77,
    #[serde(rename = "Nobuharu Matsushita")]
    NobuharuMatsushita = 78,
    #[serde(rename = "Nikita Mazepin")]
    NikitaMazepin = 79,
    #[serde(rename = "Guanya Zhou")]
    GuanyaZhou = 80,
    #[serde(rename = "Mick Schumacher")]
    MickSchumacher = 81,
    #[serde(rename = "Callum Ilott")]
    CallumIlott = 82,
    #[serde(rename = "Juan Manuel Correa")]
    JuanManuelCorrea = 83,
    #[serde(rename = "Jordan King")]
    JordanKing = 84,
    #[serde(rename = "Mahaveer Raghunathan")]
    MahaveerRaghunathan = 85,
    #[serde(rename = "Tatiana Calderon")]
    TatianaCalderon = 86,
    #[serde(rename = "Anthoine Hubert")]
    AnthoineHubert = 87,
    #[serde(rename = "Guiliano Alesi")]
    GuilianoAlesi = 88,
    #[serde(rename = "Ralph Boschung")]
    RalphBoschung = 89,
    #[serde(rename = "Michael Schumacher")]
    MichaelSchumacher = 90,
    #[serde(rename = "Dan Ticktum")]
    DanTicktum = 91,
    #[serde(rename = "Marcus Armstrong")]
    MarcusArmstrong = 92,
    #[serde(rename = "Christian Lundgaard")]
    ChristianLundgaard = 93,
    #[serde(rename = "Yuki Tsunoda")]
    YukiTsunoda = 94,
    #[serde(rename = "Jehan Daruvala")]
    JehanDaruvala = 95,
    #[serde(rename = "Gulherme Samaia")]
    GulhermeSamaia = 96,
    #[serde(rename = "Pedro Piquet")]
    PedroPiquet = 97,
    #[serde(rename = "Felipe Drugovich")]
    FelipeDrugovich = 98,
    #[serde(rename = "Robert Schwartzman")]
    RobertSchwartzman = 99,
    #[serde(rename = "Roy Nissany")]
    RoyNissany = 100,
    #[serde(rename = "Marino Sato")]
    MarinoSato = 101,
    #[serde(rename = "Aidan Jackson")]
    AidanJackson = 102,
    #[serde(rename = "Casper Akkerman")]
    CasperAkkerman = 103,
    #[serde(rename = "Jenson Button")]
    JensonButton = 109,
    #[serde(rename = "David Coulthard")]
    DavidCoulthard = 110,
    #[serde(rename = "Nico Rosberg")]
    NicoRosberg = 111,
    #[serde(rename = "Oscar Piastri")]
    OscarPiastri = 112,
    #[serde(rename = "Liam Lawson")]
    LiamLawson = 113,
    #[serde(rename = "Juri Vips")]
    JuriVips = 114,
    #[serde(rename = "Theo Pourchaire")]
    TheoPourchaire = 115,
    #[serde(rename = "Richard Verschoor")]
    RichardVerschoor = 116,
    #[serde(rename = "Lirim Zendeli")]
    LirimZendeli = 117,
    #[serde(rename = "David Beckmann")]
    DavidBeckmann = 118,
    #[serde(rename = "Alessio Deledda")]
    AlessioDeledda = 121,
    #[serde(rename = "Bent Viscaal")]
    BentViscaal = 122,
    #[serde(rename = "Enzo Fittipaldi")]
    EnzoFittipaldi = 123,
    #[serde(rename = "Mark Webber")]
    MarkWebber = 125,
    #[serde(rename = "Jacques Villeneuve")]
    JacquesVilleneuve = 126,
    #[serde(rename = "Callie Mayer")]
    CallieMayer = 127,
    #[serde(rename = "Noah Bell")]
    NoahBell = 128,
    #[serde(rename = "Jake Hughes")]
    JakeHughes = 129,
    #[serde(rename = "Frederik Vesti")]
    FrederikVesti = 130,
    #[serde(rename = "Olli Caldwell")]
    OlliCaldwell = 131,
    #[serde(rename = "Logan Sargeant")]
    LoganSargeant = 132,
    #[serde(rename = "Cem Bolukbasi")]
    CemBolukbasi = 133,
    #[serde(rename = "Ayumu Iwasa")]
    AyumuIwasa = 134,
    #[serde(rename = "Clement Novalak")]
    ClementNovalak = 135,
    #[serde(rename = "Jack Doohan")]
    JackDoohan = 136,
    #[serde(rename = "Amaury Cordeel")]
    AmauryCordeel = 137,
    #[serde(rename = "Dennis Hauger")]
    DennisHauger = 138,
    #[serde(rename = "Calan Williams")]
    CalanWilliams = 139,
    #[serde(rename = "Jamie Chadwick")]
    JamieChadwick = 140,
    #[serde(rename = "Kamui Kobayashi")]
    KamuiKobayashi = 141,
    #[serde(rename = "Pastor Maldonado")]
    PastorMaldonado = 142,
    #[serde(rename = "Mika Hakkinen")]
    MikaHakkinen = 143,
    #[serde(rename = "Nigel Mansell")]
    NigelMansell = 144,
}

#[derive(Serialize, Deserialize_repr, Debug, Copy, Clone)]
#[repr(u8)]
pub enum Nationality {
    Default,
    American,
    Argentinean,
    Australian,
    Austrian,
    Azerbaijani,
    Bahraini,
    Belgian,
    Bolivian,
    Brazilian,
    British,
    Bulgarian,
    Cameroonian,
    Canadian,
    Chilean,
    Chinese,
    Colombian,
    CostaRican,
    Croatian,
    Cypriot,
    Czech,
    Danish,
    Dutch,
    Ecuadorian,
    English,
    Emirian,
    Estonian,
    Finnish,
    French,
    German,
    Ghanaian,
    Greek,
    Guatemalan,
    Honduran,
    HongKonger,
    Hungarian,
    Icelander,
    Indian,
    Indonesian,
    Irish,
    Israeli,
    Italian,
    Jamaican,
    Japanese,
    Jordanian,
    Kuwaiti,
    Latvian,
    Lebanese,
    Lithuanian,
    Luxembourger,
    Malaysian,
    Maltese,
    Mexican,
    Monegasque,
    NewZealander,
    Nicaraguan,
    NorthernIrish,
    Norwegian,
    Omani,
    Pakistani,
    Panamanian,
    Paraguayan,
    Peruvian,
    Polish,
    Portuguese,
    Qatari,
    Romanian,
    Russian,
    Salvadoran,
    Saudi,
    Scottish,
    Serbian,
    Singaporean,
    Slovakian,
    Slovenian,
    SouthKorean,
    SouthAfrican,
    Spanish,
    Swedish,
    Swiss,
    Thai,
    Turkish,
    Uruguayan,
    Ukrainian,
    Venezuelan,
    Barbadian,
    Welsh,
    Vietnamese,
    None = 255,
}

#[derive(Serialize, Deserialize_repr, Debug, Copy, Clone)]
#[repr(u8)]
pub enum Team {
    #[serde(rename = "Mercedes")]
    Mercedes = 0,
    #[serde(rename = "Ferrari")]
    Ferrari = 1,
    #[serde(rename = "Red Bull Racing")]
    RedBullRacing = 2,
    #[serde(rename = "Williams")]
    Williams = 3,
    #[serde(rename = "Aston Martin")]
    AstonMartin = 4,
    #[serde(rename = "Alpine")]
    Alpine = 5,
    #[serde(rename = "Alpha Tauri")]
    AlphaTauri = 6,
    #[serde(rename = "Haas")]
    Haas = 7,
    #[serde(rename = "Mc Laren")]
    McLaren = 8,
    #[serde(rename = "Alfa Romeo")]
    AlfaRomeo = 9,
    #[serde(rename = "Mercedes 2020")]
    Mercedes2020 = 85,
    #[serde(rename = "Ferrari 2020")]
    Ferrari2020 = 86,
    #[serde(rename = "Red Bull 2020")]
    RedBull2020 = 87,
    #[serde(rename = "Williams 2020")]
    Williams2020 = 88,
    #[serde(rename = "Racing Point 2020")]
    RacingPoint2020 = 89,
    #[serde(rename = "Renault 2020")]
    Renault2020 = 90,
    #[serde(rename = "Alpha Tauri 2020")]
    AlphaTauri2020 = 91,
    #[serde(rename = "Haas 2020")]
    Haas2020 = 92,
    #[serde(rename = "Mc Laren 2020")]
    McLaren2020 = 93,
    #[serde(rename = "Alfa Romeo 2020")]
    AlfaRomeo2020 = 94,
    #[serde(rename = "Aston Martin DB11 V12")]
    AstonMartinDB11V12 = 95,
    #[serde(rename = "Aston Martin Vantage F1 Edition")]
    AstonMartinVantageF1Edition = 96,
    #[serde(rename = "Aston Martin Vantage Safety Car")]
    AstonMartinVantageSafetyCar = 97,
    #[serde(rename = "Ferrari F8 Tributo")]
    FerrariF8Tributo = 98,
    #[serde(rename = "Ferrari Roma")]
    FerrariRoma = 99,
    #[serde(rename = "Mc Laren 720 S")]
    McLaren720S = 100,
    #[serde(rename = "Mc Laren Artura")]
    McLarenArtura = 101,
    #[serde(rename = "Mercedes AMG GT Black Series Safety Car")]
    MercedesAMGGTBlackSeriesSafetyCar = 102,
    #[serde(rename = "Mercedes AMG GTR Pro")]
    MercedesAMGGTRPro = 103,
    #[serde(rename = "F1 Custom Team")]
    F1CustomTeam = 104,
    #[serde(rename = "Prema 21")]
    Prema21 = 106,
    #[serde(rename = "Uni Virtuosi 21")]
    UniVirtuosi21 = 107,
    #[serde(rename = "Carlin 21")]
    Carlin21 = 108,
    #[serde(rename = "Hitech 21")]
    Hitech21 = 109,
    #[serde(rename = "Art GP 21")]
    ArtGP21 = 110,
    #[serde(rename = "MPMotorsport 21")]
    MPMotorsport21 = 111,
    #[serde(rename = "Charouz 21")]
    Charouz21 = 112,
    #[serde(rename = "Dams 21")]
    Dams21 = 113,
    #[serde(rename = "Campos 21")]
    Campos21 = 114,
    #[serde(rename = "BWT 21")]
    BWT21 = 115,
    #[serde(rename = "Trident 21")]
    Trident21 = 116,
    #[serde(rename = "Mercedes AMG GT Black Series")]
    MercedesAMGGTBlackSeries = 117,
    #[serde(rename = "Mercedes 22")]
    Mercedes22 = 118,
    #[serde(rename = "Ferrari 22")]
    Ferrari22 = 119,
    #[serde(rename = "Red Bull Racing 22")]
    RedBullRacing22 = 120,
    #[serde(rename = "Williams 22")]
    Williams22 = 121,
    #[serde(rename = "Aston Martin 22")]
    AstonMartin22 = 122,
    #[serde(rename = "Alpine 22")]
    Alpine22 = 123,
    #[serde(rename = "Alpha Tauri 22")]
    AlphaTauri22 = 124,
    #[serde(rename = "Haas 22")]
    Haas22 = 125,
    #[serde(rename = "Mc Laren 22")]
    McLaren22 = 126,
    #[serde(rename = "Alfa Romeo 22")]
    AlfaRomeo22 = 127,
    #[serde(rename = "Konnersport 22")]
    Konnersport22 = 128,
    #[serde(rename = "Konnersport")]
    Konnersport = 129,
    #[serde(rename = "Prema 22")]
    Prema22 = 130,
    #[serde(rename = "Virtuosi 22")]
    Virtuosi22 = 131,
    #[serde(rename = "Carlin 22")]
    Carlin22 = 132,
    #[serde(rename = "MP Motorsport 22")]
    MPMotorsport22 = 133,
    #[serde(rename = "Charouz 22")]
    Charouz22 = 134,
    #[serde(rename = "Dams 22")]
    Dams22 = 135,
    #[serde(rename = "Campos 22")]
    Campos22 = 136,
    #[serde(rename = "Van Amersfoort Racing 22")]
    VanAmersfoortRacing22 = 137,
    #[serde(rename = "Trident 22")]
    Trident22 = 138,
    #[serde(rename = "Hitech 22")]
    Hitech22 = 139,
    #[serde(rename = "Art GP 22")]
    ArtGP22 = 140,
    #[serde(rename = "None")]
    None = 255,
}

#[derive(Deserialize, Debug, Serialize, Copy, Clone)]
pub struct PacketParticipantData {
    pub header: PacketHeader,
    pub num_active_cars: u8,
    pub participants: [ParticipantData; 22],
}

impl ParticipantData {
    #[allow(dead_code)]
    pub fn name(&self) -> String {
        let name = str::from_utf8(&self.name).unwrap().to_string();
        let res = name.trim_matches(char::from(0));

        res.to_string()
    }

    pub fn team_colour(&self) -> (u8, u8, u8) {
        match &self.team_id {
            Team::Mercedes => (108, 211, 191),
            Team::Ferrari => (249, 21, 54),
            Team::McLaren => (245, 128, 32),
            Team::RedBullRacing => (54, 113, 198),
            Team::Alpine => (34, 147, 209),
            Team::AlphaTauri => (94, 143, 170),
            Team::AstonMartin => (53, 140, 117),
            Team::Williams => (55, 190, 221),
            Team::AlfaRomeo => (201, 45, 75),
            Team::Haas => (182, 186, 189),
            // 1 => (220, 0, 0),
            // 2 => (6, 0, 239),
            // 3 => (0, 90, 255),
            // 4 => (0, 111, 98),
            // 5 => (0, 144, 255),
            // 6 => (43, 69, 98),
            // 7 => (255, 255, 255),
            // 8 => (255, 235, 0),
            // 9 => (144, 0, 0),
            _ => (0, 0, 0),
        }
    }
}
