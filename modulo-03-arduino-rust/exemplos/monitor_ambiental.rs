// monitor_ambiental.rs
// Sistema de monitoramento ambiental com Arduino e Rust
// Projeto acadêmico para análise de qualidade do ar

#![no_std]
#![no_main]

use arduino_hal::prelude::*;
use panic_halt as _;

// Estruturas de dados para monitoramento
#[derive(Debug, Clone)]
pub struct EnvironmentalData {
    pub temperature: f32,
    pub humidity: f32,
    pub air_quality: f32,
    pub pressure: f32,
    pub timestamp: u32,
}

#[derive(Debug)]
pub enum SensorError {
    ReadError,
    CalibrationError,
    CommunicationError,
}

// Configurações do sistema
pub struct SystemConfig {
    pub reading_interval: u32,    // Intervalo entre leituras (ms)
    pub alert_threshold: f32,     // Limite para alertas
    pub calibration_factor: f32,  // Fator de calibração
}

impl Default for SystemConfig {
    fn default() -> Self {
        Self {
            reading_interval: 5000,  // 5 segundos
            alert_threshold: 100.0,  // 100 ppm
            calibration_factor: 1.0,
        }
    }
}

// Gerenciador de sensores
pub struct SensorManager {
    temperature_sensor: arduino_hal::adc::AdcChannel,
    humidity_sensor: arduino_hal::adc::AdcChannel,
    air_quality_sensor: arduino_hal::adc::AdcChannel,
    pressure_sensor: arduino_hal::adc::AdcChannel,
    config: SystemConfig,
}

impl SensorManager {
    pub fn new() -> Result<Self, SensorError> {
        let dp = arduino_hal::Peripherals::take().map_err(|_| SensorError::ReadError)?;
        let pins = arduino_hal::pins!(dp);
        
        let mut adc = arduino_hal::Adc::new(dp.ADC, arduino_hal::DefaultClock);
        
        let temperature_sensor = pins.a0.into_analog_input(&mut adc);
        let humidity_sensor = pins.a1.into_analog_input(&mut adc);
        let air_quality_sensor = pins.a2.into_analog_input(&mut adc);
        let pressure_sensor = pins.a3.into_analog_input(&mut adc);
        
        Ok(Self {
            temperature_sensor,
            humidity_sensor,
            air_quality_sensor,
            pressure_sensor,
            config: SystemConfig::default(),
        })
    }
    
    pub fn read_all_sensors(&mut self) -> Result<EnvironmentalData, SensorError> {
        let temp_raw = self.temperature_sensor.analog_read(&mut adc);
        let humidity_raw = self.humidity_sensor.analog_read(&mut adc);
        let air_quality_raw = self.air_quality_sensor.analog_read(&mut adc);
        let pressure_raw = self.pressure_sensor.analog_read(&mut adc);
        
        Ok(EnvironmentalData {
            temperature: self.convert_temperature(temp_raw)?,
            humidity: self.convert_humidity(humidity_raw)?,
            air_quality: self.convert_air_quality(air_quality_raw)?,
            pressure: self.convert_pressure(pressure_raw)?,
            timestamp: arduino_hal::time::millis(),
        })
    }
    
    fn convert_temperature(&self, raw: u16) -> Result<f32, SensorError> {
        // Conversão para sensor LM35 (10mV/°C)
        let voltage = (raw as f32 * 5.0) / 1024.0;
        let temperature = voltage * 100.0; // LM35: 10mV/°C
        
        if temperature < -40.0 || temperature > 125.0 {
            return Err(SensorError::ReadError);
        }
        
        Ok(temperature)
    }
    
    fn convert_humidity(&self, raw: u16) -> Result<f32, SensorError> {
        // Conversão para sensor DHT22
        let humidity = (raw as f32 * 100.0) / 1024.0;
        
        if humidity < 0.0 || humidity > 100.0 {
            return Err(SensorError::ReadError);
        }
        
        Ok(humidity)
    }
    
    fn convert_air_quality(&self, raw: u16) -> Result<f32, SensorError> {
        // Conversão para sensor MQ-135 (CO2)
        let voltage = (raw as f32 * 5.0) / 1024.0;
        let resistance = (5.0 - voltage) / voltage;
        let ppm = 116.6020682 * resistance.powf(-2.769034857);
        
        if ppm < 0.0 || ppm > 10000.0 {
            return Err(SensorError::ReadError);
        }
        
        Ok(ppm)
    }
    
    fn convert_pressure(&self, raw: u16) -> Result<f32, SensorError> {
        // Conversão para sensor BMP280
        let voltage = (raw as f32 * 5.0) / 1024.0;
        let pressure = (voltage - 0.5) * 400.0; // kPa
        
        if pressure < 30.0 || pressure > 110.0 {
            return Err(SensorError::ReadError);
        }
        
        Ok(pressure)
    }
    
    pub fn calibrate_sensor(&mut self, sensor_type: SensorType) -> Result<(), SensorError> {
        match sensor_type {
            SensorType::Temperature => {
                // Implementar calibração de temperatura
                self.config.calibration_factor = 1.0;
            }
            SensorType::Humidity => {
                // Implementar calibração de umidade
                self.config.calibration_factor = 1.0;
            }
            SensorType::AirQuality => {
                // Implementar calibração de qualidade do ar
                self.config.calibration_factor = 1.0;
            }
            SensorType::Pressure => {
                // Implementar calibração de pressão
                self.config.calibration_factor = 1.0;
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum SensorType {
    Temperature,
    Humidity,
    AirQuality,
    Pressure,
}

// Sistema de alertas
pub struct AlertSystem {
    config: SystemConfig,
    alert_history: [bool; 10],
    alert_count: usize,
}

impl AlertSystem {
    pub fn new(config: SystemConfig) -> Self {
        Self {
            config,
            alert_history: [false; 10],
            alert_count: 0,
        }
    }
    
    pub fn check_alerts(&mut self, data: &EnvironmentalData) -> Vec<Alert> {
        let mut alerts = Vec::new();
        
        // Verificar qualidade do ar
        if data.air_quality > self.config.alert_threshold {
            alerts.push(Alert {
                level: AlertLevel::Warning,
                message: "Qualidade do ar crítica",
                value: data.air_quality,
                timestamp: data.timestamp,
            });
        }
        
        // Verificar temperatura
        if data.temperature > 35.0 || data.temperature < 5.0 {
            alerts.push(Alert {
                level: AlertLevel::Critical,
                message: "Temperatura fora da faixa normal",
                value: data.temperature,
                timestamp: data.timestamp,
            });
        }
        
        // Verificar umidade
        if data.humidity > 90.0 || data.humidity < 10.0 {
            alerts.push(Alert {
                level: AlertLevel::Warning,
                message: "Umidade fora da faixa normal",
                value: data.humidity,
                timestamp: data.timestamp,
            });
        }
        
        self.update_alert_history(alerts.len() > 0);
        alerts
    }
    
    fn update_alert_history(&mut self, has_alert: bool) {
        self.alert_history[self.alert_count % 10] = has_alert;
        self.alert_count += 1;
    }
    
    pub fn get_alert_frequency(&self) -> f32 {
        let alert_count = self.alert_history.iter().filter(|&&x| x).count();
        (alert_count as f32) / 10.0 * 100.0
    }
}

#[derive(Debug, Clone)]
pub struct Alert {
    pub level: AlertLevel,
    pub message: &'static str,
    pub value: f32,
    pub timestamp: u32,
}

#[derive(Debug, Clone)]
pub enum AlertLevel {
    Info,
    Warning,
    Critical,
}

// Sistema de comunicação
pub struct CommunicationSystem {
    serial: arduino_hal::Usart<arduino_hal::pac::USART0>,
    led_status: arduino_hal::port::Pin<arduino_hal::port::mode::Output>,
    led_alert: arduino_hal::port::Pin<arduino_hal::port::mode::Output>,
}

impl CommunicationSystem {
    pub fn new() -> Result<Self, SensorError> {
        let dp = arduino_hal::Peripherals::take().map_err(|_| SensorError::CommunicationError)?;
        let pins = arduino_hal::pins!(dp);
        
        let serial = arduino_hal::Usart::new(
            dp.USART0,
            pins.d0,
            pins.d1.into_output(),
            9600.into_baudrate(),
        );
        
        let led_status = pins.d13.into_output();
        let led_alert = pins.d12.into_output();
        
        Ok(Self {
            serial,
            led_status,
            led_alert,
        })
    }
    
    pub fn send_data(&mut self, data: &EnvironmentalData) -> Result<(), SensorError> {
        let message = format!(
            "T:{:.1}C,H:{:.1}%,AQ:{:.1}ppm,P:{:.1}kPa,T:{}\n",
            data.temperature,
            data.humidity,
            data.air_quality,
            data.pressure,
            data.timestamp
        );
        
        for byte in message.bytes() {
            nb::block!(self.serial.write(byte))
                .map_err(|_| SensorError::CommunicationError)?;
        }
        
        Ok(())
    }
    
    pub fn send_alert(&mut self, alert: &Alert) -> Result<(), SensorError> {
        let level_str = match alert.level {
            AlertLevel::Info => "INFO",
            AlertLevel::Warning => "WARNING",
            AlertLevel::Critical => "CRITICAL",
        };
        
        let message = format!(
            "ALERT[{}]: {} - Value: {:.1} at {}\n",
            level_str, alert.message, alert.value, alert.timestamp
        );
        
        for byte in message.bytes() {
            nb::block!(self.serial.write(byte))
                .map_err(|_| SensorError::CommunicationError)?;
        }
        
        Ok(())
    }
    
    pub fn update_status_leds(&mut self, status: bool, alert: bool) {
        if status {
            self.led_status.set_high();
        } else {
            self.led_status.set_low();
        }
        
        if alert {
            self.led_alert.set_high();
        } else {
            self.led_alert.set_low();
        }
    }
}

// Sistema de armazenamento de dados
pub struct DataStorage {
    data_buffer: [EnvironmentalData; 50],
    write_index: usize,
    is_full: bool,
}

impl DataStorage {
    pub fn new() -> Self {
        Self {
            data_buffer: unsafe { core::mem::zeroed() },
            write_index: 0,
            is_full: false,
        }
    }
    
    pub fn store_data(&mut self, data: EnvironmentalData) {
        self.data_buffer[self.write_index] = data;
        self.write_index = (self.write_index + 1) % 50;
        
        if self.write_index == 0 {
            self.is_full = true;
        }
    }
    
    pub fn get_latest_data(&self) -> Option<&EnvironmentalData> {
        if self.write_index == 0 && !self.is_full {
            return None;
        }
        
        let index = if self.write_index == 0 { 49 } else { self.write_index - 1 };
        Some(&self.data_buffer[index])
    }
    
    pub fn get_average_data(&self, count: usize) -> Option<EnvironmentalData> {
        if count == 0 || count > 50 {
            return None;
        }
        
        let mut sum_temp = 0.0;
        let mut sum_humidity = 0.0;
        let mut sum_air_quality = 0.0;
        let mut sum_pressure = 0.0;
        
        let start_index = if self.is_full {
            (self.write_index + 50 - count) % 50
        } else {
            0
        };
        
        for i in 0..count {
            let index = (start_index + i) % 50;
            let data = &self.data_buffer[index];
            
            sum_temp += data.temperature;
            sum_humidity += data.humidity;
            sum_air_quality += data.air_quality;
            sum_pressure += data.pressure;
        }
        
        Some(EnvironmentalData {
            temperature: sum_temp / count as f32,
            humidity: sum_humidity / count as f32,
            air_quality: sum_air_quality / count as f32,
            pressure: sum_pressure / count as f32,
            timestamp: arduino_hal::time::millis(),
        })
    }
}

// Sistema principal de monitoramento
pub struct EnvironmentalMonitoringSystem {
    sensor_manager: SensorManager,
    alert_system: AlertSystem,
    communication: CommunicationSystem,
    data_storage: DataStorage,
    last_reading_time: u32,
    system_status: SystemStatus,
}

#[derive(Debug)]
pub enum SystemStatus {
    Running,
    Calibrating,
    Error,
}

impl EnvironmentalMonitoringSystem {
    pub fn new() -> Result<Self, SensorError> {
        let config = SystemConfig::default();
        let sensor_manager = SensorManager::new()?;
        let alert_system = AlertSystem::new(config.clone());
        let communication = CommunicationSystem::new()?;
        let data_storage = DataStorage::new();
        
        Ok(Self {
            sensor_manager,
            alert_system,
            communication,
            data_storage,
            last_reading_time: 0,
            system_status: SystemStatus::Running,
        })
    }
    
    pub fn run_monitoring_cycle(&mut self) -> Result<(), SensorError> {
        let current_time = arduino_hal::time::millis();
        
        // Verificar se é hora de fazer nova leitura
        if current_time - self.last_reading_time >= self.sensor_manager.config.reading_interval {
            match self.sensor_manager.read_all_sensors() {
                Ok(data) => {
                    // Armazenar dados
                    self.data_storage.store_data(data.clone());
                    
                    // Enviar dados
                    self.communication.send_data(&data)?;
                    
                    // Verificar alertas
                    let alerts = self.alert_system.check_alerts(&data);
                    for alert in alerts {
                        self.communication.send_alert(&alert)?;
                    }
                    
                    // Atualizar LEDs de status
                    let has_alerts = !alerts.is_empty();
                    self.communication.update_status_leds(true, has_alerts);
                    
                    self.last_reading_time = current_time;
                }
                Err(e) => {
                    self.system_status = SystemStatus::Error;
                    return Err(e);
                }
            }
        }
        
        Ok(())
    }
    
    pub fn calibrate_all_sensors(&mut self) -> Result<(), SensorError> {
        self.system_status = SystemStatus::Calibrating;
        
        let sensors = [
            SensorType::Temperature,
            SensorType::Humidity,
            SensorType::AirQuality,
            SensorType::Pressure,
        ];
        
        for sensor in &sensors {
            self.sensor_manager.calibrate_sensor(sensor.clone())?;
        }
        
        self.system_status = SystemStatus::Running;
        Ok(())
    }
    
    pub fn get_system_status(&self) -> &SystemStatus {
        &self.system_status
    }
    
    pub fn get_alert_frequency(&self) -> f32 {
        self.alert_system.get_alert_frequency()
    }
}

// Função principal
#[arduino_hal::entry]
fn main() -> ! {
    let mut monitoring_system = EnvironmentalMonitoringSystem::new()
        .expect("Falha ao inicializar sistema de monitoramento");
    
    // Calibrar sensores na inicialização
    monitoring_system.calibrate_all_sensors()
        .expect("Falha na calibração dos sensores");
    
    loop {
        match monitoring_system.run_monitoring_cycle() {
            Ok(_) => {
                // Sistema funcionando normalmente
            }
            Err(e) => {
                // Tratar erro
                match e {
                    SensorError::ReadError => {
                        // Tentar recalibrar
                        let _ = monitoring_system.calibrate_all_sensors();
                    }
                    SensorError::CommunicationError => {
                        // Tentar reenviar dados
                    }
                    _ => {
                        // Outros erros
                    }
                }
            }
        }
        
        arduino_hal::delay_ms(100);
    }
}
