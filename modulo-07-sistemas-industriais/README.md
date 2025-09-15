# 🏭 Módulo 7: Sistemas Industriais

## 🎯 **Objetivos de Aprendizagem Acadêmica**

Ao final deste módulo, você será capaz de:
- ✅ Implementar sistemas PLC com Rust
- ✅ Trabalhar com protocolos industriais
- ✅ Desenvolver sistemas SCADA
- ✅ Implementar sistemas de segurança industrial (SIL)
- ✅ Criar sistemas de monitoramento industrial
- ✅ Desenvolver automação industrial

## 📚 **Conteúdo Teórico Acadêmico**

### **7.1 PLC Programming com Rust**

#### **Sistema PLC**
```rust
// plc_system.rs
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct PLCSystem {
    inputs: Arc<Mutex<HashMap<String, bool>>>,
    outputs: Arc<Mutex<HashMap<String, bool>>>,
    memory: Arc<Mutex<HashMap<String, f32>>>,
    ladder_programs: Vec<LadderProgram>,
}

pub struct LadderProgram {
    rungs: Vec<LadderRung>,
    name: String,
}

pub struct LadderRung {
    contacts: Vec<Contact>,
    coils: Vec<Coil>,
}

#[derive(Debug, Clone)]
pub enum Contact {
    Input(String),
    Output(String),
    Memory(String),
    NotInput(String),
    NotOutput(String),
    NotMemory(String),
}

#[derive(Debug, Clone)]
pub enum Coil {
    Output(String),
    Memory(String),
    SetOutput(String),
    ResetOutput(String),
}

impl PLCSystem {
    pub fn new() -> Self {
        Self {
            inputs: Arc::new(Mutex::new(HashMap::new())),
            outputs: Arc::new(Mutex::new(HashMap::new())),
            memory: Arc::new(Mutex::new(HashMap::new())),
            ladder_programs: Vec::new(),
        }
    }
    
    pub fn add_program(&mut self, program: LadderProgram) {
        self.ladder_programs.push(program);
    }
    
    pub fn scan_cycle(&self) -> Result<(), PLCError> {
        for program in &self.ladder_programs {
            self.execute_program(program)?;
        }
        Ok(())
    }
    
    fn execute_program(&self, program: &LadderProgram) -> Result<(), PLCError> {
        for rung in &program.rungs {
            let result = self.evaluate_rung(rung)?;
            self.execute_coils(rung, result)?;
        }
        Ok(())
    }
    
    fn evaluate_rung(&self, rung: &LadderRung) -> Result<bool, PLCError> {
        let mut result = true;
        
        for contact in &rung.contacts {
            let contact_value = self.read_contact(contact)?;
            result = result && contact_value;
        }
        
        Ok(result)
    }
    
    fn read_contact(&self, contact: &Contact) -> Result<bool, PLCError> {
        match contact {
            Contact::Input(name) => {
                let inputs = self.inputs.lock().unwrap();
                Ok(*inputs.get(name).unwrap_or(&false))
            }
            Contact::Output(name) => {
                let outputs = self.outputs.lock().unwrap();
                Ok(*outputs.get(name).unwrap_or(&false))
            }
            Contact::NotInput(name) => {
                let inputs = self.inputs.lock().unwrap();
                Ok(!*inputs.get(name).unwrap_or(&false))
            }
            _ => Ok(false),
        }
    }
    
    fn execute_coils(&self, rung: &LadderRung, rung_result: bool) -> Result<(), PLCError> {
        for coil in &rung.coils {
            match coil {
                Coil::Output(name) => {
                    let mut outputs = self.outputs.lock().unwrap();
                    outputs.insert(name.clone(), rung_result);
                }
                Coil::SetOutput(name) => {
                    if rung_result {
                        let mut outputs = self.outputs.lock().unwrap();
                        outputs.insert(name.clone(), true);
                    }
                }
                Coil::ResetOutput(name) => {
                    if rung_result {
                        let mut outputs = self.outputs.lock().unwrap();
                        outputs.insert(name.clone(), false);
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum PLCError {
    InvalidContact,
    InvalidCoil,
    ScanError,
}
```

### **7.2 Protocolos Industriais**

```rust
// modbus_protocol.rs
use std::io::{Read, Write};

pub struct ModbusMaster {
    connection: Box<dyn Read + Write>,
    unit_id: u8,
}

impl ModbusMaster {
    pub fn new(connection: Box<dyn Read + Write>, unit_id: u8) -> Self {
        Self { connection, unit_id }
    }
    
    pub fn read_holding_registers(&mut self, address: u16, count: u16) -> Result<Vec<u16>, ModbusError> {
        let request = ModbusRequest {
            function_code: 0x03,
            starting_address: address,
            quantity: count,
            unit_id: self.unit_id,
        };
        
        let request_bytes = request.to_bytes();
        self.connection.write_all(&request_bytes)?;
        
        let mut response = [0u8; 256];
        let bytes_read = self.connection.read(&mut response)?;
        
        let response = ModbusResponse::from_bytes(&response[..bytes_read])?;
        Ok(response.data)
    }
    
    pub fn write_single_register(&mut self, address: u16, value: u16) -> Result<(), ModbusError> {
        let request = ModbusRequest {
            function_code: 0x06,
            starting_address: address,
            quantity: 1,
            unit_id: self.unit_id,
        };
        
        let request_bytes = request.to_bytes();
        self.connection.write_all(&request_bytes)?;
        
        let mut response = [0u8; 8];
        self.connection.read(&mut response)?;
        
        Ok(())
    }
}

#[derive(Debug)]
struct ModbusRequest {
    function_code: u8,
    starting_address: u16,
    quantity: u16,
    unit_id: u8,
}

impl ModbusRequest {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.push(self.unit_id);
        bytes.push(self.function_code);
        bytes.extend_from_slice(&self.starting_address.to_be_bytes());
        bytes.extend_from_slice(&self.quantity.to_be_bytes());
        
        let crc = calculate_crc(&bytes);
        bytes.extend_from_slice(&crc.to_le_bytes());
        bytes
    }
}

#[derive(Debug)]
struct ModbusResponse {
    unit_id: u8,
    function_code: u8,
    data: Vec<u16>,
}

impl ModbusResponse {
    fn from_bytes(bytes: &[u8]) -> Result<Self, ModbusError> {
        if bytes.len() < 5 {
            return Err(ModbusError::InvalidResponse);
        }
        
        let unit_id = bytes[0];
        let function_code = bytes[1];
        let byte_count = bytes[2] as usize;
        
        if bytes.len() < 3 + byte_count + 2 {
            return Err(ModbusError::InvalidResponse);
        }
        
        let mut data = Vec::new();
        for i in 0..(byte_count / 2) {
            let offset = 3 + i * 2;
            let value = u16::from_be_bytes([bytes[offset], bytes[offset + 1]]);
            data.push(value);
        }
        
        Ok(Self {
            unit_id,
            function_code,
            data,
        })
    }
}

#[derive(Debug)]
pub enum ModbusError {
    InvalidResponse,
    CommunicationError,
    FunctionCodeError,
}

fn calculate_crc(data: &[u8]) -> u16 {
    let mut crc = 0xFFFF;
    
    for byte in data {
        crc ^= *byte as u16;
        
        for _ in 0..8 {
            if crc & 0x0001 != 0 {
                crc >>= 1;
                crc ^= 0xA001;
            } else {
                crc >>= 1;
            }
        }
    }
    
    crc
}
```

## 💻 **Exemplos Práticos Acadêmicos**

### **Exemplo 1: Sistema SCADA Industrial**

**Objetivo de Pesquisa**: Sistema de supervisão e controle para processos industriais

```rust
// scada_system.rs
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct SCADASystem {
    tags: Arc<Mutex<HashMap<String, Tag>>>,
    alarms: Arc<Mutex<Vec<Alarm>>>,
    trends: Arc<Mutex<HashMap<String, Vec<DataPoint>>>>,
    modbus_masters: Vec<ModbusMaster>,
}

#[derive(Debug, Clone)]
pub struct Tag {
    pub name: String,
    pub value: f32,
    pub timestamp: u64,
    pub quality: Quality,
    pub alarm_limits: Option<AlarmLimits>,
}

#[derive(Debug, Clone)]
pub enum Quality {
    Good,
    Bad,
    Uncertain,
}

#[derive(Debug, Clone)]
pub struct AlarmLimits {
    pub high_high: Option<f32>,
    pub high: Option<f32>,
    pub low: Option<f32>,
    pub low_low: Option<f32>,
}

#[derive(Debug, Clone)]
pub struct Alarm {
    pub tag_name: String,
    pub message: String,
    pub severity: AlarmSeverity,
    pub timestamp: u64,
    pub acknowledged: bool,
}

#[derive(Debug, Clone)]
pub enum AlarmSeverity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone)]
pub struct DataPoint {
    pub value: f32,
    pub timestamp: u64,
    pub quality: Quality,
}

impl SCADASystem {
    pub fn new() -> Self {
        Self {
            tags: Arc::new(Mutex::new(HashMap::new())),
            alarms: Arc::new(Mutex::new(Vec::new())),
            trends: Arc::new(Mutex::new(HashMap::new())),
            modbus_masters: Vec::new(),
        }
    }
    
    pub fn add_tag(&self, name: String, initial_value: f32) {
        let tag = Tag {
            name: name.clone(),
            value: initial_value,
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            quality: Quality::Good,
            alarm_limits: None,
        };
        
        let mut tags = self.tags.lock().unwrap();
        tags.insert(name, tag);
    }
    
    pub fn update_tag(&self, name: &str, value: f32) -> Result<(), SCADAError> {
        let mut tags = self.tags.lock().unwrap();
        
        if let Some(tag) = tags.get_mut(name) {
            tag.value = value;
            tag.timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
            tag.quality = Quality::Good;
            
            // Verificar alarmes
            self.check_alarms(name, value)?;
            
            // Adicionar ao trend
            self.add_trend_point(name, value, tag.timestamp)?;
            
            Ok(())
        } else {
            Err(SCADAError::TagNotFound)
        }
    }
    
    fn check_alarms(&self, tag_name: &str, value: f32) -> Result<(), SCADAError> {
        let tags = self.tags.lock().unwrap();
        let tag = tags.get(tag_name).ok_or(SCADAError::TagNotFound)?;
        
        if let Some(limits) = &tag.alarm_limits {
            let mut alarms = self.alarms.lock().unwrap();
            let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
            
            if let Some(high_high) = limits.high_high {
                if value > high_high {
                    alarms.push(Alarm {
                        tag_name: tag_name.to_string(),
                        message: format!("{} - High High Alarm: {:.2}", tag_name, value),
                        severity: AlarmSeverity::Critical,
                        timestamp,
                        acknowledged: false,
                    });
                }
            }
            
            if let Some(high) = limits.high {
                if value > high {
                    alarms.push(Alarm {
                        tag_name: tag_name.to_string(),
                        message: format!("{} - High Alarm: {:.2}", tag_name, value),
                        severity: AlarmSeverity::High,
                        timestamp,
                        acknowledged: false,
                    });
                }
            }
            
            if let Some(low) = limits.low {
                if value < low {
                    alarms.push(Alarm {
                        tag_name: tag_name.to_string(),
                        message: format!("{} - Low Alarm: {:.2}", tag_name, value),
                        severity: AlarmSeverity::Medium,
                        timestamp,
                        acknowledged: false,
                    });
                }
            }
            
            if let Some(low_low) = limits.low_low {
                if value < low_low {
                    alarms.push(Alarm {
                        tag_name: tag_name.to_string(),
                        message: format!("{} - Low Low Alarm: {:.2}", tag_name, value),
                        severity: AlarmSeverity::Critical,
                        timestamp,
                        acknowledged: false,
                    });
                }
            }
        }
        
        Ok(())
    }
    
    fn add_trend_point(&self, tag_name: &str, value: f32, timestamp: u64) -> Result<(), SCADAError> {
        let mut trends = self.trends.lock().unwrap();
        
        let data_point = DataPoint {
            value,
            timestamp,
            quality: Quality::Good,
        };
        
        trends.entry(tag_name.to_string())
            .or_insert_with(Vec::new)
            .push(data_point);
        
        // Manter apenas os últimos 1000 pontos
        if let Some(trend) = trends.get_mut(tag_name) {
            if trend.len() > 1000 {
                trend.drain(0..trend.len() - 1000);
            }
        }
        
        Ok(())
    }
    
    pub fn get_active_alarms(&self) -> Vec<Alarm> {
        let alarms = self.alarms.lock().unwrap();
        alarms.iter()
            .filter(|alarm| !alarm.acknowledged)
            .cloned()
            .collect()
    }
    
    pub fn acknowledge_alarm(&self, tag_name: &str, timestamp: u64) -> Result<(), SCADAError> {
        let mut alarms = self.alarms.lock().unwrap();
        
        for alarm in alarms.iter_mut() {
            if alarm.tag_name == tag_name && alarm.timestamp == timestamp {
                alarm.acknowledged = true;
                return Ok(());
            }
        }
        
        Err(SCADAError::AlarmNotFound)
    }
}

#[derive(Debug)]
pub enum SCADAError {
    TagNotFound,
    AlarmNotFound,
    CommunicationError,
    DataError,
}
```

---

**Próximo Módulo**: [Módulo 8: Desenvolvimento e Testes](../modulo-08-desenvolvimento-testes/README.md)

---

**Desenvolvido com ❤️ para a comunidade acadêmica brasileira**

*ETEC Bento Quirino - Curso Completo de Rust para Sistemas Embarcados*
