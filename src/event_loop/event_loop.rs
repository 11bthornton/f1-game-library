use crate::telemetry_data::{
    car_damage_data::PacketCarDamageData,
    car_setup_data::PacketCarSetupData,
    car_status_data::PacketCarStatusData,
    car_telemetry_data::PacketCarTelemetryData,
    event_data::{
        PacketEventData,
        PacketEventFinal,
    },
    final_classification::PacketClassificationData,
    lap_data::PacketLapData,
    lobby_info::PacketLobbyInfoData,
    motion_data::PacketMotionData,
    motion_extended_data::PacketMotionExData,
    participant_data::PacketParticipantData,
    session_data::PacketSessionData,
    session_history::PacketSessionHistoryData,
    tyre_set_data::PacketTyreSetsData,
    F1Data,
};
use bincode::deserialize;
use tokio::{
    net::UdpSocket,
    sync::mpsc::{
        self,
        UnboundedReceiver,
        UnboundedSender,
    },
};
use tokio_stream::wrappers::UnboundedReceiverStream;
use F1Data::{
    Classification,
    Damage,
    Event,
    ExtendedMotion,
    Lap,
    Lobby,
    Motion,
    Participant,
    Session,
    SessionHistory,
    Setup,
    Status,
    Telemetry,
    TyreSetData,
};

macro_rules! match_and_deserialize {

    ($tx:expr, $ip:expr, $port:expr, $($size:expr => ($ty:ty, $enum_variant:ident $(, $alt:ty)?)),+) => {
        tokio::spawn(async move {

            let mut buf: Vec<u8> = vec![0; 2048];
            let socket = match UdpSocket::bind(format!("{}:{}", $ip, $port)).await {

                Ok(socket) => socket,
                Err(_) => {
                    // eprintln!("Network error: {}", e);
                    return DataHandlerError::NetworkError;
                }
            };

            loop {
                match socket.recv_from(&mut buf).await {
                    Ok((n, _)) => {
                        match n {
                            $(
                                $size => {

                                    let decoded: Result<$ty, _> = deserialize(&buf);

                                    match decoded {
                                        Ok(decoded) => {
                                            $(
                                                let decoded: $alt = decoded.decode().unwrap();
                                            )?

                                            $tx.send($enum_variant(decoded)).unwrap();
                                        },
                                        Err(_) => {
                                            eprintln!(
                                                "Error parsing packet of (presumed) type {} (size {} bytes)\n{:?}",
                                                stringify!($ty),
                                                n,
                                                decoded
                                            );
                                            return DataHandlerError::DeserialisationError;
                                        }
                                    }
                                }
                            ),*
                            other => {
                                eprintln!("Found packet of size {}", other);
                            }
                        }
                    },
                    Err(_) => {
                        return DataHandlerError::NetworkError
                    }
                }
            }

        });
    };
}

pub fn event_loop_generator(
    ip: String,
    port: String,
) -> Result<impl tokio_stream::Stream<Item = F1Data>, DataHandlerError> {
    let (tx, rx): (UnboundedSender<F1Data>, UnboundedReceiver<F1Data>) = mpsc::unbounded_channel();

    match_and_deserialize!(
        tx,
        ip,
        port,
        // Packets of size _ bytes parsed into type X and shoved in Enum Variant Y
        1349 => (PacketMotionData, Motion),
        1352 => (PacketCarTelemetryData, Telemetry),
        1020 => (PacketClassificationData, Classification),
        953  => (PacketCarDamageData, Damage),
        1460 => (PacketSessionHistoryData, SessionHistory),
        1131  => (PacketLapData, Lap),
        45   => (PacketEventData, Event, PacketEventFinal),
        644  => (PacketSessionData, Session),
        1306 => (PacketParticipantData, Participant),
        1107 => (PacketCarSetupData, Setup),
        1239  => (PacketCarStatusData, Status),
        1218 => (PacketLobbyInfoData, Lobby),
        217 => (PacketMotionExData, ExtendedMotion),
        231 => (PacketTyreSetsData, TyreSetData)
    );

    Ok(UnboundedReceiverStream::new(rx))
}

#[derive(Debug)]
pub enum DataHandlerError {
    NetworkError,
    DeserialisationError,
}
