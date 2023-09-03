
#[cfg(feature = "async")]
pub mod event_loop;

#[cfg(feature = "sync")]
pub mod event_loop_sync;



use crate::{
    telemetry_data,
    telemetry_data::{
        car_damage_data::PacketCarDamageData,
        event_data::PacketEventFinal,
        lap_data,
        lobby_info::PacketLobbyInfoData,
        motion_extended_data::PacketMotionExData,
        participant_data::PacketParticipantData,
        tyre_set_data::PacketTyreSetsData,
        F1Data::{self,},
        PacketCarStatusData,
        PacketClassificationData,
        PacketMotionData,
        PacketSessionData,
        PacketSessionHistoryData,
    },
};
use async_trait;

#[cfg(feature = "async")]
pub use event_loop::event_loop_generator;

#[cfg(feature = "sync")]
pub use event_loop_sync::sync_event_loop_generator;


#[cfg(feature = "async")]
use event_loop::DataHandlerError;
#[cfg(feature = "async")]
use tokio_stream::StreamExt;


#[async_trait::async_trait]
#[cfg(feature = "async")]
#[allow(unused_variables)]
pub trait AsyncDataHandler: Send + Sync {

    async fn on_boot(&self) {}

    async fn on_packet(&self, packet: F1Data) {}

    async fn on_lap_data(&self, lap_data: lap_data::PacketLapData) {}

    async fn on_car_damage_data(&self, damage_data: PacketCarDamageData) {}

    async fn on_car_setup_data(
        &self,
        setup_data: telemetry_data::car_setup_data::PacketCarSetupData,
    ) {
    }

    async fn on_car_telemetry_data(
        &self,
        telemetry_data: telemetry_data::car_telemetry_data::PacketCarTelemetryData,
    ) {
    }

    async fn on_event_data(&self, event_data: PacketEventFinal) {}

    async fn on_participant_data(&self, participant_data: PacketParticipantData) {}

    async fn on_car_status_data(&self, status_data: PacketCarStatusData) {}

    async fn on_session_data(&self, session_data: PacketSessionData) {}

    async fn on_session_history_data(&self, session_history_data: PacketSessionHistoryData) {}

    async fn on_classification_data(&self, classification_data: PacketClassificationData) {}

    async fn on_motion_data(&self, motion_data: PacketMotionData) {}

    async fn on_lobby_info(&self, lobby_info_data: PacketLobbyInfoData) {}

    async fn on_extended_motion_data(&self, extended_motion_data: PacketMotionExData) {}

    async fn on_tyre_set_data(&self, tyre_set_data: PacketTyreSetsData) {}

    async fn listen(&self, ip: &str, port: &str) -> Result<(), DataHandlerError> {
        let mut stream = event_loop_generator(ip.to_string(), port.to_string())?;

        while let Some(packet) = stream.next().await {
            self.on_packet(packet).await;
            match packet {
                F1Data::Damage(pack) => self.on_car_damage_data(pack).await,
                F1Data::Event(pack) => self.on_event_data(pack).await,
                F1Data::Participant(pack) => self.on_participant_data(pack).await,
                F1Data::Lap(pack) => self.on_lap_data(pack).await,
                F1Data::Telemetry(pack) => self.on_car_telemetry_data(pack).await,
                F1Data::Setup(pack) => self.on_car_setup_data(pack).await,
                F1Data::Status(pack) => self.on_car_status_data(pack).await,
                F1Data::Session(pack) => self.on_session_data(pack).await,
                F1Data::SessionHistory(pack) => self.on_session_history_data(pack).await,
                F1Data::Motion(pack) => self.on_motion_data(pack).await,
                F1Data::Classification(pack) => self.on_classification_data(pack).await,
                F1Data::Lobby(pack) => self.on_lobby_info(pack).await,
                F1Data::ExtendedMotion(pack) => self.on_extended_motion_data(pack).await,
                F1Data::TyreSetData(pack) => self.on_tyre_set_data(pack).await,
            }
        }

        Ok(())
    }
}


#[cfg(feature = "sync")]
#[allow(unused_variables)]
pub trait SyncDataHandler {
    

     fn on_boot(&self) {}

     fn on_packet(&self, packet: F1Data) {}

     fn on_lap_data(&self, lap_data: lap_data::PacketLapData) {}

     fn on_car_damage_data(&self, damage_data: PacketCarDamageData) {}

     fn on_car_setup_data(
        &self,
        setup_data: telemetry_data::car_setup_data::PacketCarSetupData,
    ) {
    }

     fn on_car_telemetry_data(
        &self,
        telemetry_data: telemetry_data::car_telemetry_data::PacketCarTelemetryData,
    ) {
    }

     fn on_event_data(&self, event_data: PacketEventFinal) {}

     fn on_participant_data(&self, participant_data: PacketParticipantData) {}

     fn on_car_status_data(&self, status_data: PacketCarStatusData) {}

     fn on_session_data(&self, session_data: PacketSessionData) {}

     fn on_session_history_data(&self, session_history_data: PacketSessionHistoryData) {}

     fn on_classification_data(&self, classification_data: PacketClassificationData) {}

     fn on_motion_data(&self, motion_data: PacketMotionData) {}

     fn on_lobby_info(&self, lobby_info_data: PacketLobbyInfoData) {}

     fn on_extended_motion_data(&self, extended_motion_data: PacketMotionExData) {}

     fn on_tyre_set_data(&self, tyre_set_data: PacketTyreSetsData) {}

     fn listen(&self, ip: &str, port: &str) {
        let generator = event_loop_sync::sync_event_loop_generator(ip, port);

        let iter = event_loop_sync::GeneratorIteratorAdapter::new(generator);

        for packet in iter {
            self.on_packet(packet);
            match packet {
                F1Data::Damage(pack) => self.on_car_damage_data(pack),
                F1Data::Event(pack) => self.on_event_data(pack),
                F1Data::Participant(pack) => self.on_participant_data(pack),
                F1Data::Lap(pack) => self.on_lap_data(pack),
                F1Data::Telemetry(pack) => self.on_car_telemetry_data(pack),
                F1Data::Setup(pack) => self.on_car_setup_data(pack),
                F1Data::Status(pack) => self.on_car_status_data(pack),
                F1Data::Session(pack) => self.on_session_data(pack),
                F1Data::SessionHistory(pack) => self.on_session_history_data(pack),
                F1Data::Motion(pack) => self.on_motion_data(pack),
                F1Data::Classification(pack) => self.on_classification_data(pack),
                F1Data::Lobby(pack) => self.on_lobby_info(pack),
                F1Data::ExtendedMotion(pack) => self.on_extended_motion_data(pack),
                F1Data::TyreSetData(pack) => self.on_tyre_set_data(pack),
            }
        }
    }
}
