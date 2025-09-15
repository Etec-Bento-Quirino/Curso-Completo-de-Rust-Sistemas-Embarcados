# 📚 Tutoriais - Curso Completo de Rust para Sistemas Embarcados

## 🎯 **Guia de Tutoriais para Pesquisa e Desenvolvimento**

Este documento contém tutoriais detalhados para cada módulo do curso, com foco em aplicações acadêmicas e projetos de pesquisa.

---

## 📋 **Índice de Tutoriais Acadêmicos**

### **Módulo 1: 🏗️ Fundamentos de Sistemas Embarcados**
- [Tutorial 1.1: Análise Comparativa Rust vs C](#tutorial-11-análise-comparativa-rust-vs-c)
- [Tutorial 1.2: Arquitetura de Microcontroladores](#tutorial-12-arquitetura-de-microcontroladores)
- [Tutorial 1.3: Limitações de Recursos](#tutorial-13-limitações-de-recursos)

### **Módulo 2: ⚡ Rust no Contexto Embarcado**
- [Tutorial 2.1: Introdução ao no_std](#tutorial-21-introdução-ao-no_std)
- [Tutorial 2.2: Memory Safety em Embarcados](#tutorial-22-memory-safety-em-embarcados)
- [Tutorial 2.3: Interrupt Handling](#tutorial-23-interrupt-handling)

### **Módulo 3: 🔧 Arduino com Rust**
- [Tutorial 3.1: Setup do Ambiente](#tutorial-31-setup-do-ambiente)
- [Tutorial 3.2: GPIO e PWM](#tutorial-32-gpio-e-pwm)
- [Tutorial 3.3: Comunicação Serial](#tutorial-33-comunicação-serial)

### **Módulo 4: 📡 ESP32 e IoT**
- [Tutorial 4.1: WiFi com Rust](#tutorial-41-wifi-com-rust)
- [Tutorial 4.2: Protocolos IoT](#tutorial-42-protocolos-iot)
- [Tutorial 4.3: Power Management](#tutorial-43-power-management)

### **Módulo 5: 🍓 Raspberry Pi**
- [Tutorial 5.1: Linux Embarcado](#tutorial-51-linux-embarcado)
- [Tutorial 5.2: GPIO Control](#tutorial-52-gpio-control)
- [Tutorial 5.3: Camera Interface](#tutorial-53-camera-interface)

### **Módulo 6: 🚗 Sistemas Automotivos**
- [Tutorial 6.1: CAN Bus](#tutorial-61-can-bus)
- [Tutorial 6.2: Telemetria Veicular](#tutorial-62-telemetria-veicular)
- [Tutorial 6.3: Safety Systems](#tutorial-63-safety-systems)

### **Módulo 7: 🏭 Sistemas Industriais**
- [Tutorial 7.1: PLC Programming](#tutorial-71-plc-programming)
- [Tutorial 7.2: Industrial Protocols](#tutorial-72-industrial-protocols)
- [Tutorial 7.3: SCADA Systems](#tutorial-73-scada-systems)

### **Módulo 8: 🧪 Desenvolvimento e Testes**
- [Tutorial 8.1: Debugging](#tutorial-81-debugging)
- [Tutorial 8.2: Hardware-in-the-Loop](#tutorial-82-hardware-in-the-loop)
- [Tutorial 8.3: CI/CD para Embarcados](#tutorial-83-cicd-para-embarcados)

### **Módulo 9: 📊 Análise de Performance**
- [Tutorial 9.1: Profiling](#tutorial-91-profiling)
- [Tutorial 9.2: Power Analysis](#tutorial-92-power-analysis)
- [Tutorial 9.3: Optimization](#tutorial-93-optimization)

### **Módulo 10: 🎯 Projeto de Pesquisa**
- [Tutorial 10.1: Metodologia de Pesquisa](#tutorial-101-metodologia-de-pesquisa)
- [Tutorial 10.2: Documentação Acadêmica](#tutorial-102-documentação-acadêmica)
- [Tutorial 10.3: Publicação de Resultados](#tutorial-103-publicação-de-resultados)

---

## 📖 **Tutoriais Detalhados**

### **Tutorial 1.1: Análise Comparativa Rust vs C**

**Objetivo Acadêmico**: Comparar performance, segurança e manutenibilidade entre Rust e C em sistemas embarcados.

**Metodologia de Pesquisa**:
1. Implementar algoritmos idênticos em Rust e C
2. Medir métricas de performance
3. Analisar segurança de memória
4. Avaliar manutenibilidade do código

**Implementação**:

#### **Algoritmo em C**
```c
// algoritmo_c.c
#include <stdint.h>
#include <string.h>

void bubble_sort_c(int32_t* arr, size_t len) {
    for (size_t i = 0; i < len - 1; i++) {
        for (size_t j = 0; j < len - 1 - i; j++) {
            if (arr[j] > arr[j + 1]) {
                int32_t temp = arr[j];
                arr[j] = arr[j + 1];
                arr[j + 1] = temp;
            }
        }
    }
}

// Função vulnerável a buffer overflow
void unsafe_copy_c(char* dest, const char* src) {
    strcpy(dest, src);  // Potencial buffer overflow
}
```

#### **Algoritmo em Rust**
```rust
// algoritmo_rust.rs
#![no_std]
#![no_main]

use core::cmp::Ordering;

pub fn bubble_sort_rust(arr: &mut [i32]) {
    let len = arr.len();
    for i in 0..len - 1 {
        for j in 0..len - 1 - i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

// Função segura contra buffer overflow
pub fn safe_copy_rust(dest: &mut [u8], src: &[u8]) -> Result<(), &'static str> {
    if dest.len() < src.len() {
        return Err("Destination buffer too small");
    }
    dest[..src.len()].copy_from_slice(src);
    Ok(())
}
```

#### **Benchmark e Análise**
```rust
// benchmark.rs
use core::time::Duration;

pub struct PerformanceMetrics {
    pub execution_time: Duration,
    pub memory_usage: usize,
    pub stack_usage: usize,
    pub binary_size: usize,
}

pub fn benchmark_algorithm() -> (PerformanceMetrics, PerformanceMetrics) {
    let mut rust_metrics = PerformanceMetrics {
        execution_time: Duration::from_micros(0),
        memory_usage: 0,
        stack_usage: 0,
        binary_size: 0,
    };
    
    let mut c_metrics = PerformanceMetrics {
        execution_time: Duration::from_micros(0),
        memory_usage: 0,
        stack_usage: 0,
        binary_size: 0,
    };
    
    // Implementar medições
    // ...
    
    (rust_metrics, c_metrics)
}
```

**Resultados Esperados**:
- Performance similar entre Rust e C
- Rust oferece segurança de memória sem overhead
- Código Rust mais seguro e manutenível

---

### **Tutorial 3.1: Setup do Ambiente Arduino**

**Objetivo Acadêmico**: Configurar ambiente de desenvolvimento para Arduino com Rust.

**Passos Detalhados**:

1. **Instalação do Toolchain**:
```bash
# Instalar Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Adicionar target AVR
rustup target add avr-unknown-gnu-atmega328

# Instalar dependências do sistema
sudo apt-get install gcc-avr binutils-avr avr-libc
```

2. **Configuração do Projeto**:
```toml
# Cargo.toml
[package]
name = "arduino-project"
version = "0.1.0"
edition = "2021"

[dependencies]
arduino-hal = "0.19"
panic-halt = "0.2"

[[bin]]
name = "main"
test = false
bench = false
```

3. **Código Base**:
```rust
// src/main.rs
#![no_std]
#![no_main]

use arduino_hal::prelude::*;
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    
    let mut led = pins.d13.into_output();
    
    loop {
        led.toggle();
        arduino_hal::delay_ms(1000);
    }
}
```

4. **Compilação e Upload**:
```bash
# Compilar
cargo build --release

# Upload para Arduino
avrdude -p atmega328p -c arduino -P /dev/ttyUSB0 -U flash:w:target/avr-atmega328p/release/arduino-project.elf:e
```

**Análise Acadêmica**:
- Comparar setup com Arduino IDE
- Avaliar performance de compilação
- Medir tamanho do binário final

---

### **Tutorial 4.1: WiFi com ESP32**

**Objetivo Acadêmico**: Implementar comunicação WiFi segura com ESP32 usando Rust.

**Implementação**:

```rust
// esp32_wifi.rs
use esp_idf_hal::prelude::*;
use esp_idf_svc::wifi::{EspWifi, WifiController, WifiDevice, WifiDriver};
use esp_idf_svc::nvs::EspDefaultNvsPartition;
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_sys as _;

use std::time::Duration;
use std::thread;

struct WiFiManager {
    wifi: EspWifi<'static>,
    controller: WifiController<'static>,
}

impl WiFiManager {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let peripherals = Peripherals::take()?;
        let sys_loop = EspSystemEventLoop::take()?;
        let nvs = EspDefaultNvsPartition::take()?;
        
        let mut wifi = EspWifi::new(
            peripherals.modem,
            sys_loop.clone(),
            Some(nvs)
        )?;
        
        let controller = wifi.controller();
        
        Ok(Self { wifi, controller })
    }
    
    pub fn connect(&mut self, ssid: &str, password: &str) -> Result<(), Box<dyn std::error::Error>> {
        let wifi_config = esp_idf_svc::wifi::Configuration::Client(
            esp_idf_svc::wifi::ClientConfiguration {
                ssid: ssid.into(),
                password: password.into(),
                ..Default::default()
            }
        );
        
        self.controller.set_configuration(&wifi_config)?;
        self.controller.start()?;
        
        // Aguardar conexão
        while !self.controller.is_connected()? {
            thread::sleep(Duration::from_millis(100));
        }
        
        Ok(())
    }
    
    pub fn get_ip(&self) -> Result<std::net::Ipv4Addr, Box<dyn std::error::Error>> {
        let ip_info = self.controller.get_ip_info()?;
        Ok(ip_info.ip.addr)
    }
}

// Exemplo de uso
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut wifi_manager = WiFiManager::new()?;
    
    // Conectar à rede
    wifi_manager.connect("MinhaRede", "MinhaSenha")?;
    
    // Obter IP
    let ip = wifi_manager.get_ip()?;
    println!("Conectado! IP: {}", ip);
    
    // Implementar servidor HTTP ou cliente MQTT
    // ...
    
    Ok(())
}
```

**Projeto de Pesquisa Sugerido**:
- Implementar protocolo de comunicação segura
- Analisar consumo energético
- Comparar com implementação em C

---

### **Tutorial 5.1: Linux Embarcado com Raspberry Pi**

**Objetivo Acadêmico**: Desenvolver aplicações Rust para Raspberry Pi com Linux embarcado.

**Implementação**:

```rust
// raspberry_pi_app.rs
use rppal::gpio::{Gpio, InputPin, OutputPin, Level};
use rppal::i2c::I2c;
use std::thread;
use std::time::Duration;

struct RaspberryPiController {
    led_pin: OutputPin,
    button_pin: InputPin,
    i2c: I2c,
}

impl RaspberryPiController {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let gpio = Gpio::new()?;
        
        let led_pin = gpio.get(18)?.into_output();
        let button_pin = gpio.get(24)?.into_input();
        
        let mut i2c = I2c::new()?;
        i2c.set_slave_address(0x48)?; // Endereço do sensor
        
        Ok(Self {
            led_pin,
            button_pin,
            i2c,
        })
    }
    
    pub fn read_sensor(&mut self) -> Result<u16, Box<dyn std::error::Error>> {
        let mut buffer = [0u8; 2];
        self.i2c.read(&mut buffer)?;
        
        let value = ((buffer[0] as u16) << 8) | (buffer[1] as u16);
        Ok(value)
    }
    
    pub fn control_led(&mut self, state: bool) -> Result<(), Box<dyn std::error::Error>> {
        self.led_pin.write(if state { Level::High } else { Level::Low });
        Ok(())
    }
    
    pub fn is_button_pressed(&self) -> Result<bool, Box<dyn std::error::Error>> {
        Ok(self.button_pin.read() == Level::Low)
    }
}

// Sistema de monitoramento
fn monitoring_system() -> Result<(), Box<dyn std::error::Error>> {
    let mut controller = RaspberryPiController::new()?;
    
    loop {
        // Ler sensor
        let sensor_value = controller.read_sensor()?;
        
        // Controlar LED baseado no sensor
        if sensor_value > 512 {
            controller.control_led(true)?;
        } else {
            controller.control_led(false)?;
        }
        
        // Verificar botão
        if controller.is_button_pressed()? {
            println!("Botão pressionado! Valor do sensor: {}", sensor_value);
        }
        
        thread::sleep(Duration::from_millis(100));
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    monitoring_system()
}
```

**Projeto Acadêmico**:
- Sistema de monitoramento ambiental
- Análise de performance em tempo real
- Integração com serviços web

---

## 🎯 **Metodologia de Pesquisa**

### **Estrutura de Relatório Acadêmico**

1. **Introdução**
   - Contexto e justificativa
   - Objetivos e hipóteses
   - Metodologia utilizada

2. **Revisão Bibliográfica**
   - Estado da arte em Rust embarcados
   - Comparação com outras tecnologias
   - Trabalhos relacionados

3. **Metodologia**
   - Configuração experimental
   - Ferramentas e equipamentos
   - Procedimentos de teste

4. **Resultados e Discussão**
   - Análise de dados
   - Comparações e métricas
   - Interpretação dos resultados

5. **Conclusões**
   - Principais descobertas
   - Limitações do estudo
   - Trabalhos futuros

### **Métricas de Avaliação**

- **Performance**: Tempo de execução, uso de memória
- **Segurança**: Análise de vulnerabilidades
- **Manutenibilidade**: Complexidade ciclomática, linhas de código
- **Consumo Energético**: Medições de corrente e tensão
- **Confiabilidade**: Tempo entre falhas, taxa de erro

---

**Desenvolvido com ❤️ para a comunidade acadêmica brasileira**

*ETEC Bento Quirino - Curso Completo de Rust para Sistemas Embarcados*
