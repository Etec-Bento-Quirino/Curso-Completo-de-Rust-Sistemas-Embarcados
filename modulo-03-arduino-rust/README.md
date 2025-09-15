# 🔧 Módulo 3: Arduino com Rust (no_std)

## 🎯 **Objetivos de Aprendizagem Acadêmica**

Ao final deste módulo, você será capaz de:
- ✅ Configurar ambiente de desenvolvimento Rust para Arduino
- ✅ Trabalhar com `no_std` e bare metal programming
- ✅ Implementar HAL (Hardware Abstraction Layer)
- ✅ Desenvolver sistemas de interrupção e timers
- ✅ Criar protocolos de comunicação (I2C, SPI, Serial)
- ✅ Realizar projetos de pesquisa com Arduino e Rust

## 📚 **Conteúdo Teórico Acadêmico**

### **3.1 Introdução ao no_std**

#### **Conceitos Fundamentais**
```rust
#![no_std]  // Sem biblioteca padrão
#![no_main] // Sem função main padrão
#![feature(start)] // Feature para entry point

// Sem heap allocation
// Sem std::collections
// Sem std::thread
// Sem std::sync
```

#### **Vantagens do no_std**
- **Controle Total**: Acesso direto ao hardware
- **Previsibilidade**: Sem garbage collection
- **Performance**: Sem overhead de runtime
- **Segurança**: Memory safety mantida

### **3.2 Hardware Abstraction Layer (HAL)**

#### **Estrutura do HAL**
```rust
// hal.rs
pub trait DigitalPin {
    type Error;
    
    fn set_high(&mut self) -> Result<(), Self::Error>;
    fn set_low(&mut self) -> Result<(), Self::Error>;
    fn is_high(&self) -> Result<bool, Self::Error>;
    fn is_low(&self) -> Result<bool, Self::Error>;
}

pub trait AnalogPin {
    type Error;
    
    fn read(&mut self) -> Result<u16, Self::Error>;
}

pub trait Serial {
    type Error;
    
    fn write(&mut self, data: &[u8]) -> Result<(), Self::Error>;
    fn read(&mut self, buffer: &mut [u8]) -> Result<usize, Self::Error>;
}
```

#### **Implementação para Arduino**
```rust
// arduino_hal.rs
use arduino_hal::prelude::*;
use arduino_hal::port::mode::Output;
use arduino_hal::port::Pin;

pub struct ArduinoDigitalPin {
    pin: Pin<Output, arduino_hal::port::mode::Output>,
}

impl DigitalPin for ArduinoDigitalPin {
    type Error = ();
    
    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.pin.set_high();
        Ok(())
    }
    
    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.pin.set_low();
        Ok(())
    }
    
    fn is_high(&self) -> Result<bool, Self::Error> {
        Ok(self.pin.is_set_high())
    }
    
    fn is_low(&self) -> Result<bool, Self::Error> {
        Ok(self.pin.is_set_low())
    }
}
```

### **3.3 Sistema de Interrupções**

#### **Configuração de Interrupções**
```rust
// interrupts.rs
use arduino_hal::interrupt;
use core::sync::atomic::{AtomicBool, Ordering};

static INTERRUPT_FLAG: AtomicBool = AtomicBool::new(false);

#[interrupt]
fn INT0() {
    // Interrupção externa no pino 2
    INTERRUPT_FLAG.store(true, Ordering::Relaxed);
}

pub struct InterruptManager {
    interrupt_enabled: bool,
}

impl InterruptManager {
    pub fn new() -> Self {
        Self {
            interrupt_enabled: false,
        }
    }
    
    pub fn enable_external_interrupt(&mut self, pin: u8) {
        // Configurar interrupção externa
        arduino_hal::interrupt::enable();
        self.interrupt_enabled = true;
    }
    
    pub fn check_interrupt_flag(&self) -> bool {
        INTERRUPT_FLAG.load(Ordering::Relaxed)
    }
    
    pub fn clear_interrupt_flag(&mut self) {
        INTERRUPT_FLAG.store(false, Ordering::Relaxed);
    }
}
```

## 💻 **Exemplos Práticos Acadêmicos**

### **Exemplo 1: Sistema de Monitoramento Ambiental**

**Objetivo de Pesquisa**: Monitorar temperatura, umidade e qualidade do ar em tempo real

```rust
// environmental_monitor.rs
#![no_std]
#![no_main]

use arduino_hal::prelude::*;
use panic_halt as _;

pub struct EnvironmentalMonitor {
    temperature_sensor: arduino_hal::adc::AdcChannel,
    humidity_sensor: arduino_hal::adc::AdcChannel,
    air_quality_sensor: arduino_hal::adc::AdcChannel,
    serial: arduino_hal::Usart<arduino_hal::pac::USART0>,
    led_status: arduino_hal::port::Pin<arduino_hal::port::mode::Output>,
}

impl EnvironmentalMonitor {
    pub fn new() -> Self {
        let dp = arduino_hal::Peripherals::take().unwrap();
        let pins = arduino_hal::pins!(dp);
        
        let mut adc = arduino_hal::Adc::new(dp.ADC, arduino_hal::DefaultClock);
        let temperature_sensor = pins.a0.into_analog_input(&mut adc);
        let humidity_sensor = pins.a1.into_analog_input(&mut adc);
        let air_quality_sensor = pins.a2.into_analog_input(&mut adc);
        
        let serial = arduino_hal::Usart::new(
            dp.USART0,
            pins.d0,
            pins.d1.into_output(),
            9600.into_baudrate(),
        );
        
        let led_status = pins.d13.into_output();
        
        Self {
            temperature_sensor,
            humidity_sensor,
            air_quality_sensor,
            serial,
            led_status,
        }
    }
    
    pub fn read_sensors(&mut self) -> SensorData {
        let temp_raw = self.temperature_sensor.analog_read(&mut adc);
        let humidity_raw = self.humidity_sensor.analog_read(&mut adc);
        let air_quality_raw = self.air_quality_sensor.analog_read(&mut adc);
        
        SensorData {
            temperature: self.convert_temperature(temp_raw),
            humidity: self.convert_humidity(humidity_raw),
            air_quality: self.convert_air_quality(air_quality_raw),
        }
    }
    
    fn convert_temperature(&self, raw: u16) -> f32 {
        // Conversão ADC para temperatura (exemplo)
        (raw as f32 * 5.0 / 1024.0 - 0.5) * 100.0
    }
    
    fn convert_humidity(&self, raw: u16) -> f32 {
        // Conversão ADC para umidade (exemplo)
        raw as f32 * 100.0 / 1024.0
    }
    
    fn convert_air_quality(&self, raw: u16) -> f32 {
        // Conversão ADC para qualidade do ar (exemplo)
        raw as f32 * 500.0 / 1024.0
    }
    
    pub fn send_data(&mut self, data: &SensorData) {
        let message = format!(
            "T:{:.1}C,H:{:.1}%,AQ:{:.1}\n",
            data.temperature,
            data.humidity,
            data.air_quality
        );
        
        for byte in message.bytes() {
            nb::block!(self.serial.write(byte)).unwrap();
        }
    }
    
    pub fn update_status_led(&mut self, status: bool) {
        if status {
            self.led_status.set_high();
        } else {
            self.led_status.set_low();
        }
    }
}

pub struct SensorData {
    pub temperature: f32,
    pub humidity: f32,
    pub air_quality: f32,
}

#[arduino_hal::entry]
fn main() -> ! {
    let mut monitor = EnvironmentalMonitor::new();
    let mut last_reading = 0u32;
    
    loop {
        let current_time = arduino_hal::time::millis();
        
        // Ler sensores a cada 5 segundos
        if current_time - last_reading > 5000 {
            let sensor_data = monitor.read_sensors();
            monitor.send_data(&sensor_data);
            
            // Atualizar LED de status
            let status = sensor_data.temperature > 25.0 && sensor_data.humidity < 80.0;
            monitor.update_status_led(status);
            
            last_reading = current_time;
        }
        
        arduino_hal::delay_ms(100);
    }
}
```

### **Exemplo 2: Sistema de Controle com PID**

**Objetivo de Pesquisa**: Implementar controlador PID para temperatura

```rust
// pid_controller.rs
pub struct PIDController {
    kp: f32,    // Proporcional
    ki: f32,    // Integral
    kd: f32,    // Derivativo
    integral: f32,
    previous_error: f32,
    last_time: u32,
}

impl PIDController {
    pub fn new(kp: f32, ki: f32, kd: f32) -> Self {
        Self {
            kp,
            ki,
            kd,
            integral: 0.0,
            previous_error: 0.0,
            last_time: 0,
        }
    }
    
    pub fn calculate(&mut self, setpoint: f32, current_value: f32, current_time: u32) -> f32 {
        let error = setpoint - current_value;
        let dt = (current_time - self.last_time) as f32 / 1000.0; // Converter para segundos
        
        if dt > 0.0 {
            // Termo proporcional
            let proportional = self.kp * error;
            
            // Termo integral
            self.integral += error * dt;
            let integral = self.ki * self.integral;
            
            // Termo derivativo
            let derivative = if dt > 0.0 {
                self.kd * (error - self.previous_error) / dt
            } else {
                0.0
            };
            
            let output = proportional + integral + derivative;
            
            // Atualizar para próxima iteração
            self.previous_error = error;
            self.last_time = current_time;
            
            // Limitar saída
            output.max(0.0).min(255.0)
        } else {
            0.0
        }
    }
}

// Sistema de controle de temperatura
pub struct TemperatureController {
    pid: PIDController,
    heater_pin: arduino_hal::port::Pin<arduino_hal::port::mode::Output>,
    temperature_sensor: arduino_hal::adc::AdcChannel,
    target_temperature: f32,
}

impl TemperatureController {
    pub fn new(
        heater_pin: arduino_hal::port::Pin<arduino_hal::port::mode::Output>,
        temperature_sensor: arduino_hal::adc::AdcChannel,
        kp: f32, ki: f32, kd: f32,
    ) -> Self {
        Self {
            pid: PIDController::new(kp, ki, kd),
            heater_pin,
            temperature_sensor,
            target_temperature: 25.0,
        }
    }
    
    pub fn set_target_temperature(&mut self, temperature: f32) {
        self.target_temperature = temperature;
    }
    
    pub fn update(&mut self, current_time: u32) -> f32 {
        let current_temp = self.read_temperature();
        let pid_output = self.pid.calculate(
            self.target_temperature,
            current_temp,
            current_time
        );
        
        // Controlar aquecedor via PWM
        self.control_heater(pid_output);
        
        current_temp
    }
    
    fn read_temperature(&self) -> f32 {
        let raw = self.temperature_sensor.analog_read(&mut adc);
        // Conversão específica do sensor
        (raw as f32 * 5.0 / 1024.0 - 0.5) * 100.0
    }
    
    fn control_heater(&mut self, power: f32) {
        // Implementar controle PWM do aquecedor
        let pwm_value = (power * 255.0 / 100.0) as u8;
        // Configurar PWM no pino do aquecedor
    }
}
```

### **Exemplo 3: Comunicação I2C com Sensores**

**Objetivo de Pesquisa**: Implementar protocolo I2C para múltiplos sensores

```rust
// i2c_communication.rs
use arduino_hal::i2c::I2c;
use arduino_hal::prelude::*;

pub struct I2CSensorManager {
    i2c: I2c<arduino_hal::pac::TWI>,
    sensor_addresses: [u8; 4],
}

impl I2CSensorManager {
    pub fn new() -> Self {
        let dp = arduino_hal::Peripherals::take().unwrap();
        let pins = arduino_hal::pins!(dp);
        
        let i2c = arduino_hal::I2c::new(
            dp.TWI,
            pins.a4.into_pull_up_input(),
            pins.a5.into_pull_up_input(),
            100000, // 100kHz
        );
        
        Self {
            i2c,
            sensor_addresses: [0x48, 0x49, 0x4A, 0x4B], // Endereços exemplo
        }
    }
    
    pub fn scan_devices(&mut self) -> Vec<u8> {
        let mut found_devices = Vec::new();
        
        for address in 0..128 {
            let result = self.i2c.write(address, &[]);
            if result.is_ok() {
                found_devices.push(address);
            }
        }
        
        found_devices
    }
    
    pub fn read_sensor(&mut self, address: u8, register: u8) -> Result<u16, I2CError> {
        let mut buffer = [0u8; 2];
        
        // Escrever endereço do registro
        self.i2c.write(address, &[register])?;
        
        // Ler dados
        self.i2c.read(address, &mut buffer)?;
        
        Ok(((buffer[0] as u16) << 8) | (buffer[1] as u16))
    }
    
    pub fn read_all_sensors(&mut self) -> Vec<SensorReading> {
        let mut readings = Vec::new();
        
        for &address in &self.sensor_addresses {
            if let Ok(value) = self.read_sensor(address, 0x00) {
                readings.push(SensorReading {
                    address,
                    value,
                    timestamp: arduino_hal::time::millis(),
                });
            }
        }
        
        readings
    }
}

pub struct SensorReading {
    pub address: u8,
    pub value: u16,
    pub timestamp: u32,
}

#[derive(Debug)]
pub enum I2CError {
    WriteError,
    ReadError,
    Timeout,
}

impl From<arduino_hal::i2c::Error> for I2CError {
    fn from(_: arduino_hal::i2c::Error) -> Self {
        I2CError::WriteError
    }
}
```

## 🛠️ **Projeto Acadêmico: Estação Meteorológica**

### **Objetivo**
Desenvolver uma estação meteorológica completa com Arduino e Rust.

### **Funcionalidades**
- ✅ Medição de temperatura, umidade, pressão
- ✅ Análise de qualidade do ar
- ✅ Comunicação serial com computador
- ✅ Armazenamento de dados em EEPROM
- ✅ Sistema de alertas
- ✅ Calibração de sensores

### **Estrutura do Projeto**
```
estacao-meteorologica/
├── src/
│   ├── main.rs
│   ├── sensors/
│   │   ├── temperature.rs
│   │   ├── humidity.rs
│   │   ├── pressure.rs
│   │   └── air_quality.rs
│   ├── communication/
│   │   ├── serial.rs
│   │   └── i2c.rs
│   ├── storage/
│   │   └── eeprom.rs
│   └── utils/
│       ├── calibration.rs
│       └── alerts.rs
├── Cargo.toml
└── README.md
```

## 🎯 **Atividades Acadêmicas**

### **Atividade 1: Análise de Performance**
- Medir tempo de resposta dos sensores
- Comparar com implementação em C
- Analisar consumo de memória

### **Atividade 2: Calibração de Sensores**
- Implementar algoritmo de calibração
- Analisar precisão e acurácia
- Documentar procedimentos

### **Atividade 3: Sistema de Comunicação**
- Desenvolver protocolo customizado
- Implementar checksum e validação
- Testar robustez da comunicação

## 📊 **Métricas de Avaliação**

### **Implementação (50%)**
- Funcionalidade correta
- Código limpo e documentado
- Tratamento de erros
- Otimização de recursos

### **Pesquisa (30%)**
- Análise de dados
- Comparação com trabalhos relacionados
- Metodologia científica
- Conclusões fundamentadas

### **Apresentação (20%)**
- Clareza na exposição
- Demonstração prática
- Resposta a questionamentos
- Qualidade do relatório

---

**Próximo Módulo**: [Módulo 4: ESP32 e IoT](../modulo-04-esp32-iot/README.md)

---

**Desenvolvido com ❤️ para a comunidade acadêmica brasileira**

*ETEC Bento Quirino - Curso Acadêmico de Rust para Sistemas Embarcados*
