# 📡 Módulo 4: ESP32 e IoT com Rust

## 🎯 **Objetivos de Aprendizagem Acadêmica**

Ao final deste módulo, você será capaz de:
- ✅ Configurar ambiente de desenvolvimento ESP32 com Rust
- ✅ Implementar conectividade WiFi e Bluetooth
- ✅ Desenvolver protocolos IoT (MQTT, CoAP, HTTP)
- ✅ Gerenciar energia e power management
- ✅ Implementar OTA (Over-The-Air) updates
- ✅ Criar sistemas IoT seguros e eficientes

## 📋 **Pré-requisitos**

### **Obrigatórios**
- ✅ Conhecimento básico de Rust
- ✅ Conceitos de sistemas embarcados
- ✅ Familiaridade com ESP32
- ✅ Conhecimento de protocolos de rede

### **Recomendados**
- ✅ Experiência com `no_std` programming
- ✅ Conhecimento de WiFi e Bluetooth
- ✅ Familiaridade com protocolos IoT
- ✅ Experiência com sistemas conectados

### **Recursos de Aprendizado**
- 📚 [The Rust Programming Language](https://doc.rust-lang.org/book/)
- 🔧 [Rust Embedded Book](https://docs.rust-embedded.org/book/)
- ⚡ [Embedded Rust Discovery](https://docs.rust-embedded.org/discovery/)
- 🌐 [ESP32 Rust Community](https://github.com/esp-rs)

## 📚 **Conteúdo Teórico Acadêmico**

### **📋 Índice do Módulo**
- [4.1 Introdução ao ESP32](#41-introdução-ao-esp32)
- [4.2 Conectividade WiFi](#42-conectividade-wifi)
- [4.3 Protocolos IoT](#43-protocolos-iot)
- [Exemplos Práticos](#exemplos-práticos-acadêmicos)
- [Projeto Acadêmico](#projeto-acadêmico-sistema-iot-inteligente)
- [Atividades Acadêmicas](#atividades-acadêmicas)

---

### **4.1 Introdução ao ESP32**

#### **Características do ESP32**
- **Dual Core**: 2 processadores Xtensa LX6
- **WiFi e Bluetooth**: Conectividade integrada
- **Memória**: 520KB SRAM, 4MB Flash
- **GPIO**: 34 pinos digitais
- **Periféricos**: ADC, DAC, SPI, I2C, UART

### **4.2 Conectividade WiFi**

```rust
// esp32_wifi.rs
use esp_idf_hal::prelude::*;
use esp_idf_svc::wifi::{EspWifi, WifiController, WifiDevice, WifiDriver};
use esp_idf_svc::nvs::EspDefaultNvsPartition;
use esp_idf_svc::eventloop::EspSystemEventLoop;

pub struct WiFiManager {
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
        
        while !self.controller.is_connected()? {
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
        
        Ok(())
    }
}
```

### **4.3 Protocolos IoT**

#### **MQTT Client**
```rust
// mqtt_client.rs
use esp_idf_svc::mqtt::client::{EspMqttClient, MqttClientConfiguration};
use esp_idf_svc::eventloop::EspSystemEventLoop;

pub struct IoTClient {
    mqtt_client: EspMqttClient<'static>,
    connected: bool,
}

impl IoTClient {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let sys_loop = EspSystemEventLoop::take()?;
        
        let mqtt_config = MqttClientConfiguration {
            broker_url: "mqtt://broker.hivemq.com:1883",
            client_id: Some("esp32_rust_client"),
            ..Default::default()
        };
        
        let mqtt_client = EspMqttClient::new(
            mqtt_config,
            sys_loop.clone(),
        )?;
        
        Ok(Self {
            mqtt_client,
            connected: false,
        })
    }
    
    pub fn publish_data(&mut self, topic: &str, data: &str) -> Result<(), Box<dyn std::error::Error>> {
        if !self.connected {
            return Err("MQTT not connected".into());
        }
        
        self.mqtt_client.publish(topic, 1, false, data.as_bytes())?;
        Ok(())
    }
}
```

## 💻 **Exemplos Práticos Acadêmicos**

### **Exemplo 1: Estação Meteorológica IoT**

**Objetivo de Pesquisa**: Monitoramento ambiental em tempo real via IoT

```rust
// estacao_meteorologica_iot.rs
use esp_idf_hal::prelude::*;
use esp_idf_hal::adc::*;

pub struct EstacaoMeteorologica {
    wifi_manager: WiFiManager,
    iot_client: IoTClient,
    sensors: SensorManager,
}

impl EstacaoMeteorologica {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            wifi_manager: WiFiManager::new()?,
            iot_client: IoTClient::new()?,
            sensors: SensorManager::new()?,
        })
    }
    
    pub async fn run_monitoring(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        loop {
            let sensor_data = self.sensors.read_all()?;
            
            let json_data = serde_json::json!({
                "temperature": sensor_data.temperature,
                "humidity": sensor_data.humidity,
                "pressure": sensor_data.pressure,
                "timestamp": sensor_data.timestamp
            });
            
            self.iot_client.publish_data(
                "estacao/tempo",
                &json_data.to_string()
            )?;
            
            tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
        }
    }
}
```

## 🛠️ **Projeto Acadêmico: Sistema IoT Inteligente**

### **Objetivo**
Desenvolver sistema IoT completo com análise de dados em tempo real.

### **Funcionalidades**
- ✅ Conectividade WiFi robusta
- ✅ Protocolos IoT seguros
- ✅ Análise de dados em tempo real
- ✅ Power management inteligente
- ✅ OTA updates automáticos

---

## 🧭 **Navegação**

### **📚 Material de Apoio**
- [**README Principal**](../../README.md) - Visão geral do curso
- [**Tutoriais Detalhados**](../../TUTORIAIS.md) - Guia completo de tutoriais
- [**Módulo 3: Arduino**](../modulo-03-arduino-rust/README.md) - Módulo anterior
- [**Módulo 5: Raspberry Pi**](../modulo-05-raspberry-pi/README.md) - Próximo módulo

### **🔗 Links Úteis**
- [Rust Embedded Working Group](https://github.com/rust-embedded/wg)
- [Arduino Rust Community](https://github.com/Rahix/avr-hal)
- [ESP32 Rust Community](https://github.com/esp-rs)

### **📖 Documentação Oficial**
- [The Rust Programming Language](https://doc.rust-lang.org/book/)
- [Rust Embedded Book](https://docs.rust-embedded.org/book/)
- [Embedded Rust Discovery](https://docs.rust-embedded.org/discovery/)
- [ESP32 Rust Documentation](https://github.com/esp-rs)

---

**Próximo Módulo**: [Módulo 5: Raspberry Pi](../modulo-05-raspberry-pi/README.md)

---

**Desenvolvido com ❤️ para a comunidade acadêmica brasileira**

*ETEC Bento Quirino - Curso Completo de Rust para Sistemas Embarcados*
