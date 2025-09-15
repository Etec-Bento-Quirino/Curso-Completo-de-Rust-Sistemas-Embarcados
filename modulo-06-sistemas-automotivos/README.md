# 🚗 Módulo 6: Sistemas Automotivos

## 🎯 **Objetivos de Aprendizagem Acadêmica**

Ao final deste módulo, você será capaz de:
- ✅ Implementar comunicação CAN bus
- ✅ Desenvolver sistemas de telemetria veicular
- ✅ Trabalhar com protocolos automotivos
- ✅ Implementar sistemas de segurança (SIL)
- ✅ Desenvolver diagnósticos automotivos
- ✅ Criar sistemas de controle automotivo

## 📋 **Pré-requisitos**

### **Obrigatórios**
- ✅ Conhecimento básico de Rust
- ✅ Conceitos de sistemas embarcados
- ✅ Familiaridade com protocolos de comunicação
- ✅ Conhecimento de sistemas automotivos

### **Recomendados**
- ✅ Experiência com CAN bus
- ✅ Conhecimento de OBD-II
- ✅ Familiaridade com sistemas de segurança
- ✅ Experiência com projetos automotivos

### **Recursos de Aprendizado**
- 📚 [The Rust Programming Language](https://doc.rust-lang.org/book/)
- 🔧 [Rust Embedded Book](https://docs.rust-embedded.org/book/)
- ⚡ [Embedded Rust Discovery](https://docs.rust-embedded.org/discovery/)
- 🚗 [CAN Bus Documentation](https://en.wikipedia.org/wiki/CAN_bus)

## 📚 **Conteúdo Teórico Acadêmico**

### **📋 Índice do Módulo**
- [6.1 Comunicação CAN Bus](#61-comunicação-can-bus)
- [6.2 Sistema de Telemetria](#62-sistema-de-telemetria)
- [Exemplos Práticos](#exemplos-práticos-acadêmicos)
- [Projeto Acadêmico](#projeto-acadêmico-sistema-de-diagnóstico-automotivo)
- [Atividades Acadêmicas](#atividades-acadêmicas)

---

### **6.1 Comunicação CAN Bus**

#### **Implementação CAN**
```rust
// can_bus.rs
use embedded_can::{Can, Frame, StandardId};
use nb::block;

pub struct CANController {
    can: Can<impl Can>,
    node_id: u8,
}

impl CANController {
    pub fn new(can: Can<impl Can>, node_id: u8) -> Self {
        Self { can, node_id }
    }
    
    pub fn send_data(&mut self, target_id: u8, data: &[u8]) -> Result<(), CANError> {
        if data.len() > 8 {
            return Err(CANError::DataTooLong);
        }
        
        let frame = Frame::new_data(
            StandardId::new(target_id as u16).ok_or(CANError::InvalidId)?,
            data
        );
        
        block!(self.can.transmit(&frame)).map_err(|_| CANError::TransmitError)?;
        Ok(())
    }
    
    pub fn receive_data(&mut self) -> Result<CANMessage, CANError> {
        let frame = block!(self.can.receive()).map_err(|_| CANError::ReceiveError)?;
        
        Ok(CANMessage {
            id: frame.id().as_raw() as u8,
            data: frame.data().to_vec(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() as u32,
        })
    }
}

#[derive(Debug)]
pub struct CANMessage {
    pub id: u8,
    pub data: Vec<u8>,
    pub timestamp: u32,
}

#[derive(Debug)]
pub enum CANError {
    DataTooLong,
    InvalidId,
    TransmitError,
    ReceiveError,
}
```

### **6.2 Sistema de Telemetria**

```rust
// telemetria_veicular.rs
use std::collections::HashMap;

pub struct TelemetrySystem {
    sensors: HashMap<SensorType, Box<dyn Sensor>>,
    can_controller: CANController,
    data_logger: DataLogger,
}

pub trait Sensor {
    fn read_value(&mut self) -> Result<f32, SensorError>;
    fn get_id(&self) -> SensorType;
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum SensorType {
    EngineRPM,
    VehicleSpeed,
    ThrottlePosition,
    BrakePressure,
    FuelLevel,
    EngineTemperature,
    OilPressure,
}

impl TelemetrySystem {
    pub fn new(can_controller: CANController) -> Self {
        Self {
            sensors: HashMap::new(),
            can_controller,
            data_logger: DataLogger::new(),
        }
    }
    
    pub fn add_sensor(&mut self, sensor: Box<dyn Sensor>) {
        let sensor_id = sensor.get_id();
        self.sensors.insert(sensor_id, sensor);
    }
    
    pub fn collect_data(&mut self) -> Result<TelemetryData, TelemetryError> {
        let mut data = TelemetryData::new();
        
        for (sensor_type, sensor) in &mut self.sensors {
            let value = sensor.read_value()?;
            data.add_reading(*sensor_type, value);
        }
        
        // Enviar dados via CAN
        self.send_can_data(&data)?;
        
        // Log dos dados
        self.data_logger.log_data(&data);
        
        Ok(data)
    }
    
    fn send_can_data(&mut self, data: &TelemetryData) -> Result<(), CANError> {
        let can_data = data.to_can_format();
        self.can_controller.send_data(0x100, &can_data)?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct TelemetryData {
    readings: HashMap<SensorType, f32>,
    timestamp: u32,
}

impl TelemetryData {
    pub fn new() -> Self {
        Self {
            readings: HashMap::new(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs() as u32,
        }
    }
    
    pub fn add_reading(&mut self, sensor: SensorType, value: f32) {
        self.readings.insert(sensor, value);
    }
    
    pub fn get_reading(&self, sensor: &SensorType) -> Option<f32> {
        self.readings.get(sensor).copied()
    }
    
    pub fn to_can_format(&self) -> Vec<u8> {
        let mut data = Vec::new();
        
        // Formato: [sensor_id, value_bytes...]
        for (sensor_type, value) in &self.readings {
            data.push(*sensor_type as u8);
            let value_bytes = value.to_le_bytes();
            data.extend_from_slice(&value_bytes);
        }
        
        data
    }
}
```

## 💻 **Exemplos Práticos Acadêmicos**

### **Exemplo 1: Sistema de Diagnóstico Automotivo**

**Objetivo de Pesquisa**: Sistema de diagnóstico em tempo real para veículos

```rust
// diagnostico_automotivo.rs
use std::collections::HashMap;

pub struct DiagnosticSystem {
    dtc_codes: HashMap<u16, DTCInfo>,
    active_codes: Vec<u16>,
    can_controller: CANController,
}

#[derive(Debug, Clone)]
pub struct DTCInfo {
    pub code: u16,
    pub description: String,
    pub severity: Severity,
    pub category: Category,
}

#[derive(Debug, Clone)]
pub enum Severity {
    Info,
    Warning,
    Critical,
}

#[derive(Debug, Clone)]
pub enum Category {
    Engine,
    Transmission,
    Brakes,
    Electrical,
    Emissions,
}

impl DiagnosticSystem {
    pub fn new(can_controller: CANController) -> Self {
        let mut system = Self {
            dtc_codes: HashMap::new(),
            active_codes: Vec::new(),
            can_controller,
        };
        
        system.initialize_dtc_codes();
        system
    }
    
    fn initialize_dtc_codes(&mut self) {
        // Códigos DTC comuns
        self.dtc_codes.insert(0x0001, DTCInfo {
            code: 0x0001,
            description: "Engine Misfire Detected".to_string(),
            severity: Severity::Critical,
            category: Category::Engine,
        });
        
        self.dtc_codes.insert(0x0002, DTCInfo {
            code: 0x0002,
            description: "Oxygen Sensor Malfunction".to_string(),
            severity: Severity::Warning,
            category: Category::Emissions,
        });
        
        // Adicionar mais códigos...
    }
    
    pub fn scan_for_codes(&mut self) -> Result<Vec<DTCInfo>, DiagnosticError> {
        // Solicitar códigos DTC via CAN
        let request = [0x03, 0x00, 0x00]; // OBD-II request
        self.can_controller.send_data(0x7E0, &request)?;
        
        // Receber resposta
        let response = self.can_controller.receive_data()?;
        
        // Processar códigos
        let mut found_codes = Vec::new();
        let mut data = response.data.as_slice();
        
        while data.len() >= 2 {
            let code = u16::from_be_bytes([data[0], data[1]]);
            if let Some(dtc_info) = self.dtc_codes.get(&code) {
                found_codes.push(dtc_info.clone());
            }
            data = &data[2..];
        }
        
        self.active_codes = found_codes.iter().map(|dtc| dtc.code).collect();
        Ok(found_codes)
    }
    
    pub fn clear_codes(&mut self) -> Result<(), DiagnosticError> {
        let clear_request = [0x04]; // Clear DTCs
        self.can_controller.send_data(0x7E0, &clear_request)?;
        
        self.active_codes.clear();
        Ok(())
    }
    
    pub fn get_active_codes(&self) -> &Vec<u16> {
        &self.active_codes
    }
}

#[derive(Debug)]
pub enum DiagnosticError {
    CANError(CANError),
    InvalidResponse,
    Timeout,
}
```

---

**Próximo Módulo**: [Módulo 7: Sistemas Industriais](../modulo-07-sistemas-industriais/README.md)

---

**Desenvolvido com ❤️ para a comunidade acadêmica brasileira**

*ETEC Bento Quirino - Curso Completo de Rust para Sistemas Embarcados*
